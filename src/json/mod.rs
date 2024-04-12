use std::{fs::File, io::Read};

pub fn process_file(filename: &str) -> anyhow::Result<()> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    process_str(&contents)?;

    Ok(())
}

pub fn process_str(contents: &str) -> anyhow::Result<()> {
    println!("{}", contents);

    Ok(())
}
