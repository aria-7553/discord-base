#[tokio::main]
async fn main() {
    discord_base::setup::start("config.toml").await;
}
