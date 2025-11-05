# tg_send

A Telegram bot that downloads TikTok videos and posts them to a specified Telegram channel. The bot only accepts commands from an authorized user.

## Features

- Accepts TikTok video links from an authorized user only
- Downloads TikTok videos using yt-dlp
- Automatically posts downloaded videos to a specified Telegram channel
- Cleans up downloaded files after posting

## Prerequisites

- Rust (latest stable version)
- yt-dlp installed on your system: `pip install yt-dlp`
- A Telegram Bot Token (get one from [@BotFather](https://t.me/botfather))

## Installation

1. Clone this repository
2. Copy the example configuration files:
   ```bash
   cp config.json.example config.json
   cp .env.example .env
   ```

3. Edit `config.json` with your settings:
   ```json
   {
     "allowed_user_id": 123456789,
     "channel_id": -1001234567890
   }
   ```

4. Edit `.env` and add your bot token:
   ```
   TELOXIDE_TOKEN=your_bot_token_here
   ```

## Configuration

### Getting Your User ID

To find your Telegram user ID:
1. Message [@userinfobot](https://t.me/userinfobot)
2. It will reply with your user ID
3. Add this ID to `config.json` as `allowed_user_id`

### Getting Channel ID

To find your channel ID:
1. Add [@getidsbot](https://t.me/getidsbot) to your channel as admin
2. It will send you the channel ID
3. Add this ID to `config.json` as `channel_id` (it should be negative, like -1001234567890)
4. Make sure your bot is also added as an admin to the channel

## Building and Running

```bash
# Build the project
cargo build --release

# Run the bot
cargo run --release
```

## Usage

1. Start a chat with your bot on Telegram
2. Send a TikTok video link (e.g., https://www.tiktok.com/@user/video/123456789)
3. The bot will download the video and post it to your configured channel

## Security

The bot will only respond to messages from the user ID specified in `config.json`. Any other user attempting to use the bot will receive an "unauthorized" message.

## License

MIT