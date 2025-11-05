mod config;
mod tiktok;

use anyhow::Result;
use log::{info, warn};
use teloxide::{
    prelude::*,
    types::{InputFile, ChatId},
};

use config::Config;
use tiktok::TikTokDownloader;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    info!("Starting TikTok to Telegram bot...");

    let config = Config::load()?;
    let bot = Bot::from_env();

    info!("Bot started. Waiting for messages...");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let config = config.clone();
        async move {
            handle_message(bot, msg, config).await?;
            Ok(())
        }
    })
    .await;

    Ok(())
}

async fn handle_message(bot: Bot, msg: Message, config: Config) -> Result<()> {
    // Check if message is from authorized user
    if let Some(user) = msg.from() {
        if user.id.0 != config.allowed_user_id {
            warn!("Unauthorized access attempt from user ID: {}", user.id.0);
            bot.send_message(msg.chat.id, "You are not authorized to use this bot.")
                .await?;
            return Ok(());
        }
    } else {
        return Ok(());
    }

    // Get message text
    if let Some(text) = msg.text() {
        info!("Received message: {}", text);

        // Check if it's a TikTok URL
        if text.contains("tiktok.com") || text.contains("vm.tiktok.com") {
            bot.send_message(msg.chat.id, "📥 Downloading TikTok video...")
                .await?;

            match download_and_send(text, &bot, &config).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, "✅ Video posted to channel!")
                        .await?;
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Error: {}", e),
                    )
                    .await?;
                }
            }
        } else {
            bot.send_message(
                msg.chat.id,
                "Please send a TikTok video link.\nExample: https://www.tiktok.com/@user/video/123456789",
            )
            .await?;
        }
    }

    Ok(())
}

async fn download_and_send(url: &str, bot: &Bot, config: &Config) -> Result<()> {
    info!("Downloading video from: {}", url);

    let downloader = TikTokDownloader::new();
    let video_path = downloader.download(url).await?;

    info!("Video downloaded to: {}", video_path);

    // Send video to channel
    let channel_id = ChatId(config.channel_id);
    bot.send_video(channel_id, InputFile::file(&video_path))
        .await?;

    info!("Video sent to channel");

    // Clean up downloaded file
    tokio::fs::remove_file(&video_path).await?;

    Ok(())
}
