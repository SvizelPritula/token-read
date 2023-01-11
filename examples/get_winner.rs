use std::io::stdin;

use anyhow::Result;
use token_read::TokenReader;

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());

    let (player_count,): (usize,) = input.line()?;

    let mut best = None;

    for _ in 0..player_count {
        let (name, points): (String, i64) = input.line()?;

        if let Some((_, best_points)) = best {
            if points <= best_points {
                continue;
            }
        }

        best = Some((name, points));
    }

    if let Some((name, points)) = best {
        println!("{name} is the winner with {points} points.");
    } else {
        println!("There are no players.")
    }

    Ok(())
}
