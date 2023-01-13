# token-read

[![Crates.io version](https://img.shields.io/crates/v/token-read?style=flat-square)](https://crates.io/crates/token-read) ![License](https://img.shields.io/crates/l/token-read?style=flat-square) [![GitHub last commit](https://img.shields.io/github/last-commit/SvizelPritula/token-read?style=flat-square)](https://github.com/SvizelPritula/token-read) [![GitHub issues](https://img.shields.io/github/issues/SvizelPritula/token-read?style=flat-square)](https://github.com/SvizelPritula/token-read/issues)

This is a simple crate that allows for easy parsing of whitespace delimited files.

It is primarily intended for competitive programming, where such files are commonly used as inputs due to being easy to parse in C and C++. This crate aims to bring this ease to Rust.

## Examples

For complete programs, see the [examples](https://github.com/SvizelPritula/token-read/tree/main/examples) in the source repository.

### Initialization

A `TokenReader` can be constructed from any type implementing `BufRead`, such as a file, standard input or a byte slice.

The easiest way to handle errors is to use [anyhow](https://crates.io/crates/anyhow).

```rust
use std::io::stdin;

use anyhow::Result;
use token_read::TokenReader;

fn main() -> Result<()> {
    let mut input = TokenReader::new(stdin().lock());

    // Do IO and computation

    Ok(())
}
```

### Reading one or more values

A tuple of one or more values of any type implementing `FromStr` can be read using the `line` function.

```rust
let (budget, ): (u64, ) = input.line()?;
let (product, cost): (String, u64) = input.line()?;
```

#### Sample input

```
10000
Sandwich 80
```

### Reading a raw line

In order to read a line without any modifications, you can use the `line_raw` function.

```rust
let sentence: String = input.line_raw()?;
```

#### Sample input

```
All human beings are born free and equal in dignity and rights.
```

### Reading a collection of values

The `line` function can also be used to read a variable amount values of a type implementing `FromStr` into most standard collections.

```rust
let temperatures: Vec<f64> = input.line()?;
let allowed_letters: HashSet<char> = input.line()?;
```

#### Sample input

```
18.7 19.2 19.4 18.9
A B E I J M N
```

### Reading several lines

The `take` function can be used to create an iterator consuming a specific number of lines. You can use it to make a simple `for` loop.

```rust
let (city_count, ): (usize, ) = input.line()?;

for city in input.take(city_count) {
    let (name, population): (String, u64) = city?;
}
```

Alternatively, it can be collected into any data structure.

```rust
let (city_count, ): (usize, ) = input.line()?;
let cities: Vec<(String, u64)> = input.take(city_count).collect::<Result<_, _>>()?;
```

#### Sample input

```
3
Prague 1309000
New York 8468000
Tokio 13960000
```

## Installation

This crate is [available from crates.io](https://crates.io/crates/token-read). To install, simply run:

```sh
cargo add token-read
```
