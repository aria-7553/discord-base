#[tokio::main]
async fn main() {
    discord_base::start("config.toml").await;
}
