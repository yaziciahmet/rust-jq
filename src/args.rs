use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "JSON processor CLI")]
pub struct Args {
    #[command(flatten)]
    pub input: Input,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct Input {
    #[arg(short, long, help = "Input JSON file")]
    pub file: Option<String>,
    #[arg(short, long, help = "Raw JSON input")]
    pub raw: Option<String>,
}

pub fn parse() -> Args {
    Args::parse()
}
