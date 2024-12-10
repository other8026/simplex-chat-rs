use anyhow::Result;
use simplex_chat::ChatClient;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let mut chat = ChatClient::start("ws://localhost:5225").await?;

    // let resp = chat.send_command("/chats").await?;
    // println!("Response: {:#?}", resp);

    let user = chat.api_get_active_user().await?;
    // println!("Active User: {:?}", user);

    let address = chat.api_get_user_address().await?;
    println!("Address: {:?}", address);

    // let chats = chat.api_chats().await?;
    // println!("Chats: {:?}", chats);

    chat.listen(|srv_resp| println!("NEW SERVER MESSAGE: {:?}", srv_resp))
        .await;

    Ok(())
}
