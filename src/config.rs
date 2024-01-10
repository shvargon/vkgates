use clap::Parser;
use dotenv::dotenv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppState {
    #[clap(long, env, default_value="0.0.0.0")]
    pub host: String,
    #[clap(long, env, default_value="3000")]
    pub port: u16,
}

pub fn read_config() -> AppState {
    dotenv().ok();
    AppState::parse()
}
