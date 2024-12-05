
use anyhow::Result;
use simplex_chat::ChatClient;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut chat = ChatClient::start("ws://localhost:5225").await?;

    let resp = chat.send_command("/u").await?;
    println!("Response: {:?}", resp);

    Ok(())
}
