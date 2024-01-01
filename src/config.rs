use clap::Parser;
use dotenv::dotenv;
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, env)]
    vk_confirmation_token: String,
    #[clap(long, env)]
    vk_secret: Option<String>,
    #[clap(long, env)]
    host: Option<String>,
    #[clap(long, env)]
    port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub vk_confirmation_token: String,

    pub vk_secret: Option<String>,
    // vkcommunityid: u32,
    pub telegram_group_id: String,
}

pub fn read_config() -> (Option<String>, Option<u16>, AppState) {
    dotenv().ok();

    let cli = Cli::parse();
    let groupid = env::var("TELEGRAM_GROUP_ID").unwrap();

    let state = AppState {
        vk_confirmation_token: cli.vk_confirmation_token,
        vk_secret: cli.vk_secret,
        telegram_group_id: groupid,
    };

    (cli.host, cli.port, state)
}
