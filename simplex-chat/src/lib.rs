mod responses;
mod types;

use anyhow::Result;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
pub use responses::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use std::{sync::mpsc, time::Duration};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

type ChatWebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

type CorrId = String;

#[derive(Debug)]
pub struct ChatClient {
    uri: String,
    command_counter: AtomicU64,
    timeout: Duration,
    write_stream: SplitSink<ChatWebSocket, Message>,
    listener_handle: JoinHandle<()>,
    command_waiters: Arc<Mutex<HashMap<CorrId, mpsc::Sender<ChatResponse>>>>,
    message_queue: mpsc::Receiver<ChatSrvResponse>, // Note that command_waiters has precedence over message_queue
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ChatSrvRequest {
    corr_id: CorrId,
    cmd: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatSrvResponse {
    corr_id: Option<CorrId>,
    resp: ChatResponse,
}

impl ChatClient {
    pub async fn start(uri: &str) -> Result<ChatClient> {
        log::debug!("Connecting to SimpleX chat client at URI: {}", uri);
        let (ws_stream, resp) = connect_async(uri).await?;

        // There will be one reader per client, but there can be many writers
        // For that reason, we will only store the writer stream and move
        // the reader stream into the asynchronous `run_client` function
        // Note that we don't have to use locks, because the streams themselves
        // already have internal locks
        let (write_stream, read_stream) = ws_stream.split();

        log::debug!(
            "Successfully connected to SimpleX chat client with response: {:?}",
            resp
        );

        let command_waiters = Arc::new(Mutex::new(HashMap::new()));
        let command_waiters_copy = command_waiters.clone();
        let uri_copy = uri.to_owned();
        let (tx, rx) = mpsc::channel::<ChatSrvResponse>();
        let listener_handle = tokio::spawn(async {
            Self::message_listener(read_stream, uri_copy, command_waiters_copy, tx).await
        });

        let client = ChatClient {
            uri: uri.to_owned(),
            command_counter: AtomicU64::new(0),
            write_stream,
            listener_handle,
            command_waiters,
            message_queue: rx,
            timeout: Duration::from_millis(3000),
        };

        Ok(client)
    }

    pub async fn message_listener(
        read_stream: SplitStream<ChatWebSocket>,
        uri: String,
        command_waiters: Arc<Mutex<HashMap<CorrId, mpsc::Sender<ChatResponse>>>>,
        message_queue: mpsc::Sender<ChatSrvResponse>,
    ) {
        read_stream
            .for_each(|message| async {
                let message = message.unwrap().into_text().unwrap();
                log::debug!("New message for client '{}': {:?}", uri, message);

                let srv_resp = serde_json::from_str::<ChatSrvResponse>(&message).unwrap();

                log::trace!("Deserialized server resposne: {:?}", srv_resp);

                match srv_resp.corr_id {
                    Some(ref corr_id) => {
                        // Send message to command waiter (if there is one),
                        // or just forward it to the message queue as well
                        let command_waiters = command_waiters.lock().unwrap();
                        match command_waiters.get(corr_id) {
                            Some(chan) => {
                                chan.send(srv_resp.resp).unwrap();
                            }
                            None => message_queue.send(srv_resp).unwrap(),
                        }
                    }
                    None => {
                        // No corrId means the message was not result of a command,
                        // so just put it in the queue right away
                        message_queue.send(srv_resp).unwrap()
                    }
                };
            })
            .await;
    }

    pub async fn send_command(&mut self, command: &str) -> Result<ChatResponse> {
        let corr_id = (self.command_counter.fetch_add(1, Ordering::Relaxed) + 1).to_string();

        // Create channel for receiving back the command return
        let (tx, rx) = mpsc::channel::<ChatResponse>();

        {
            let mut command_waiters = self.command_waiters.lock().unwrap();
            command_waiters.insert(corr_id.clone(), tx);
            log::trace!(
                "Inserted '{}' to command waiters of client '{}': {:?}",
                corr_id,
                self.uri,
                command_waiters
            );
        }

        log::debug!(
            "Sending command `{}` ({}) to '{}'",
            command,
            corr_id,
            self.uri
        );

        let srv_req = ChatSrvRequest {
            corr_id: corr_id.to_string(),
            cmd: command.to_owned(),
        };
        let cmd_json = serde_json::to_string(&srv_req)?;
        log::trace!("Serialized command: {}", cmd_json);

        self.write_stream.send(Message::Text(cmd_json)).await?;

        log::debug!("Command '{}' send successfully to '{}'", corr_id, self.uri);

        log::debug!(
            "Waiting for response to command '{}' on client '{}'... (timeout = {:?})",
            corr_id,
            self.uri,
            self.timeout
        );

        let resp = rx.recv_timeout(self.timeout);

        {
            let mut command_waiters = self.command_waiters.lock().unwrap();
            command_waiters.remove(&corr_id);
            log::trace!(
                "Removed '{}' from command waiters of client '{}': {:?}",
                corr_id,
                self.uri,
                command_waiters
            );
        }

        let resp = resp?;

        Ok(resp)
    }

    pub async fn listen(&mut self, message_listener_callback: impl Fn(ChatSrvResponse) -> ()) {
        loop {
            let message = self.message_queue.recv().unwrap();
            message_listener_callback(message);
        }
    }
}

impl Drop for ChatClient {
    fn drop(&mut self) {
        self.listener_handle.abort();
    }
}
