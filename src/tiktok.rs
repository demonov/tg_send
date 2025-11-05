use anyhow::{Context, Result};
use std::process::Command;
use std::path::PathBuf;

pub struct TikTokDownloader {
    download_dir: PathBuf,
}

impl TikTokDownloader {
    pub fn new() -> Self {
        Self {
            download_dir: PathBuf::from("./downloads"),
        }
    }

    pub async fn download(&self, url: &str) -> Result<String> {
        // Create downloads directory if it doesn't exist
        tokio::fs::create_dir_all(&self.download_dir).await?;

        // Generate output filename
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let output_template = self.download_dir.join(format!("{}.%(ext)s", timestamp));

        // Use yt-dlp to download the video
        let output = Command::new("yt-dlp")
            .arg(url)
            .arg("-o")
            .arg(output_template.to_str().unwrap())
            .arg("--no-playlist")
            .arg("--quiet")
            .output()
            .context("Failed to execute yt-dlp. Make sure it's installed: pip install yt-dlp")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("yt-dlp failed: {}", error);
        }

        // Find the downloaded file
        let mut entries = tokio::fs::read_dir(&self.download_dir).await?;
        let mut latest_file: Option<(PathBuf, std::time::SystemTime)> = None;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = entry.metadata().await {
                    if let Ok(modified) = metadata.modified() {
                        if let Some((_, latest_time)) = &latest_file {
                            if modified > *latest_time {
                                latest_file = Some((path, modified));
                            }
                        } else {
                            latest_file = Some((path, modified));
                        }
                    }
                }
            }
        }

        let video_path = latest_file
            .context("No video file found after download")?
            .0;

        Ok(video_path.to_string_lossy().to_string())
    }
}
