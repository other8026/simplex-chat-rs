use anyhow::Result;
use simplex_chat::ChatClient;

async fn process_messages(mut chat: ChatClient) -> Result<()> {
    loop {
        let message = chat.next_message().await?;
        println!("Received message: {:?}", message);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut chat = ChatClient::start("ws://localhost:5225").await?;

    // let resp = chat.send_command("/chats").await?;
    // println!("Response: {:#?}", resp);

    let user = chat.api_get_active_user().await?;
    println!("Active User: {:?}", user);

    let address = match chat.api_get_user_address().await? {
        Some(addr) => addr,
        None => chat.api_create_user_address().await?,
    };
    println!("Address: {:?}", address);

    // let chats = chat.api_chats().await?;
    // println!("Chats: {:?}", chats);

    process_messages(chat).await?;

    Ok(())
}
