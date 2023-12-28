use clap::Parser;
use dotenv::dotenv;
use std::env;
use teloxide::Bot;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, env)]
    vk_confirmation_token: String,
    #[clap(long, env)]
    vk_community_id: String,
    #[clap(long, env)]
    host: Option<String>,
    #[clap(long, env)]
    port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub vk_confirmation_token: String,
    pub vk_community_id: String,
    // vkcommunityid: u32,
    pub bot: Bot,
    pub telegram_group_id: String,
}

pub fn read_config() -> (Option<String>, Option<u16>, AppState) {
    dotenv().ok();

    let bot = Bot::from_env();

    let cli = Cli::parse();
    let groupid = env::var("TELEGRAM_GROUP_ID").unwrap();

    let state = AppState {
        vk_confirmation_token: cli.vk_confirmation_token,
        vk_community_id: cli.vk_community_id,
        // vkcommunityid: cli.vkcommunityid,
        bot: bot,
        telegram_group_id: groupid,
    };

    (cli.host, cli.port, state)
}
