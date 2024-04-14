use jq;
use log::{error, info};

pub mod args;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let input = args::parse().input;

    let result = match (input.file, input.raw) {
        (Some(file), None) => jq::process_file(&file),
        (None, Some(raw)) => jq::process_str(&raw),
        _ => panic!("Should never happen!"),
    };
    match result {
        Ok(_) => info!("JSON is valid."),
        Err(e) => error!("Error: {}", e),
    };
}
