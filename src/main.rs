use log::error;

mod args;
mod json;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let args = args::parse();
    match json::process_file(&args.file) {
        Err(e) => error!("Error: {}", e),
        _ => (),
    };
}
