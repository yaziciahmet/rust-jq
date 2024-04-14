use log::debug;
use std::{fs::File, io::Read};
use tokenizer::Tokenizer;

pub mod parser;
pub mod tokenizer;

pub fn process_file(filename: &str) -> anyhow::Result<()> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    process_str(&contents)?;

    Ok(())
}

pub fn process_str(contents: &str) -> anyhow::Result<()> {
    debug!("Content: {}", contents);

    let tokenizer = Tokenizer::new(contents);
    let tokens = Tokenizer::try_collect(tokenizer)?;
    debug!("Tokens: {:?}", tokens);

    parser::parse(&mut tokens.iter().peekable())?;

    Ok(())
}
