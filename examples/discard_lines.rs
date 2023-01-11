use std::{io::stdin, collections::HashSet};

use anyhow::Result;
use token_read::TokenReader;

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());

    let (line_count, _discard_count): (usize, usize) = input.line()?;
    let discards: HashSet<usize> = input.line()?;

    for i in 0..line_count {
        let line = input.line_raw()?;

        if !discards.contains(&i) {
            println!("{line}");
        }
    }

    Ok(())
}
