use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "JSON processor CLI", about = "JSON processor CLI")]
pub struct Args {
    #[arg(long, help = "Input JSON file")]
    pub file: String,
}

pub fn parse() -> Args {
    Args::parse()
}
