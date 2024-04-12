use log::debug;
use std::{fs::File, io::Read};
use token::Tokenizer;

mod token;

pub fn process_file(filename: &str) -> anyhow::Result<()> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    process_str(&contents)?;

    Ok(())
}

pub fn process_str(contents: &str) -> anyhow::Result<()> {
    debug!("File content: {}", contents);

    let tokenizer = Tokenizer::new(contents);
    tokenizer.for_each(|t| {
        debug!("Next token: {:?}", t);
    });

    Ok(())
}
