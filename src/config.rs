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
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub vk_confirmation_token: String,
    pub vk_community_id: String,
    // vkcommunityid: u32,
    pub bot: Bot,
    pub telegram_group_id: String,
}

pub fn read_config(bot: Bot) -> AppState {
    dotenv().ok();

    let cli = Cli::parse();
    let groupid = env::var("TELEGRAM_GROUP_ID").unwrap();

    AppState {
        vk_confirmation_token: cli.vk_confirmation_token,
        vk_community_id: cli.vk_community_id,
        // vkcommunityid: cli.vkcommunityid,
        bot: bot,
        telegram_group_id: groupid,
    }
}
