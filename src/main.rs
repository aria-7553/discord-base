#[tokio::main]
async fn main() {
    discord_base::start::start("config.toml").await;
}
