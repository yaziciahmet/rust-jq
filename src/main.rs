mod args;
mod json;

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let args = args::parse();
    json::process_file(&args.file).expect("Failed to process file");
}
