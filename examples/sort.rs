use std::io::stdin;

use anyhow::Result;
use token_read::TokenReader;

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());

    let (value_count,): (usize,) = input.line()?;

    let mut values: Vec<i64> = input
        .take(value_count)
        .map(|r| r.map(|(v,)| v))
        .collect::<Result<_, _>>()?;

    values.sort();

    for value in values {
        println!("{value}");
    }

    Ok(())
}
