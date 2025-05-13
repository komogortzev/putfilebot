use teloxide::{respond, Bot, types::Message }; 
use dotenv::dotenv;
use anyhow::Context;

use std::env;
use std::path::PathBuf;

mod handlers;
use handlers::handle_message;



#[tokio::main]
async fn main() -> anyhow::Result<()> { 
    dotenv().ok(); 
    pretty_env_logger::init();
    log::info!("Starting file saving bot...");

    let bot_token = env::var("TELOXIDE_TOKEN").expect("Couldn't fetch token");
    let download_path_str = env::var("DOWNLOAD_PATH").expect("Couldn't fetch path");
    let download_path: PathBuf = download_path_str.into(); 
    tokio::fs::create_dir_all(&download_path)
        .await
        .context(format!("Failed to create download directory at {:?}", download_path))?;

    let bot = Bot::new(bot_token);
    teloxide::repl(bot, move |bot: Bot, message: Message| {
        let download_path = download_path.clone(); 
        async move {
            match handle_message(bot, message, download_path).await {
                Ok(_) => respond(()), 
                Err(e) => {
                    log::error!("Error handling message: {:?}", e);
                    respond(()) 
                }
            }
        }
    }).await;
    Ok(()) 
}
