use std::{collections::HashSet, io::stdin};

use anyhow::Result;
use token_read::TokenReader;

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());

    let (line_count, _discard_count): (u64, usize) = input.line()?;
    let discards: HashSet<u64> = input.line()?;

    for i in 0..line_count {
        let line = input.line_raw()?;

        if !discards.contains(&i) {
            println!("{line}");
        }
    }

    Ok(())
}
