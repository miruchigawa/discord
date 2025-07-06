[![Build & Push Docker Image](https://github.com/miruchigawa/discord/actions/workflows/docker.yml/badge.svg?branch=main)](https://github.com/miruchigawa/discord/actions/workflows/docker.yml)

This project is a minimal template for creating a Discord bot using Rust.

## Prerequisites

* Rust and Cargo installed (minimum version `1.87`)
* A Discord bot token (create one in the [Discord Developer Portal](https://discord.com/developers/applications))

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/miftahfauzanworks/discord-rs.git
   cd discord-rs
   ```

2. Create a `.env` file in the root directory:

   ```env
   DISCORD_TOKEN=your_discord_bot_token_here
   ```

3. Build the project:

   ```bash
   cargo build --release
   ```

## Configuration

* **.env**

  * `DISCORD_TOKEN`: Your Discord bot token.

## Usage

Run the bot with:

```bash
cargo run --release
```

Invite the bot to your server and use the slash command `/ping`. The bot will reply with "Pong!".

## Extending the Bot

1. In `src/commands/mod.rs`, register your new slash command module.
2. Create a new file in `src/commands/` for your command logic.
3. Implement the command handler in the new module.
4. Rebuild and restart the bot.

## Contributing

1. Fork this repository
2. Create a feature branch: `git checkout -b feature/your-command`
3. Commit your changes: `git commit -m "Add /your-command slash command"`
4. Push to your branch: `git push origin feature/your-command`
5. Open a pull request

## License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](LICENSE) file for details.
