use clap::Parser;
use dotenv::dotenv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppState {
    #[clap(long, env)]
    pub host: Option<String>,
    #[clap(long, env)]
    pub port: Option<u16>,
}

pub fn read_config() -> AppState {
    dotenv().ok();
    AppState::parse()
}
