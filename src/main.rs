mod args;
mod json;

fn main() {
    let args = args::parse();
    json::process_file(&args.file).expect("Failed to process file");
}
