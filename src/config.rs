use clap::Parser;
use dotenv::dotenv;

const SEP: &'static str = std::path::MAIN_SEPARATOR_STR;

fn config_dir() -> String {
    let mut path = dirs::config_dir().unwrap();
    path.push("vkgates");
    path.to_str().unwrap().to_string()
}

#[derive(Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct AppState {
    #[clap(long, env, default_value = "0.0.0.0")]
    pub host: String,
    #[clap(long, env, default_value = "3000")]
    pub port: u16,
    #[clap(long, env, default_value_t = config_dir())]
    pub config_path: String,
}

impl AppState {
    pub fn get_config_file_path(&self, filename: &str) -> String {
        format!("{}{}{}", self.config_path, SEP, filename)
    }
}

pub fn read_config() -> AppState {
    dotenv().ok();
    AppState::parse()
}
