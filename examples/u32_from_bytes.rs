use std::io::stdin;

use anyhow::Result;
use token_read::TokenReader;

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());

    let bytes: [u8; 4] = input.line()?;
    let value = u32::from_be_bytes(bytes);

    println!("{value}");

    Ok(())
}
