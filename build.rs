use std::{
    env,
    fs::File,
    io::{Error, Write},
    path::Path,
};

// This could be probably done with a complex macro, but it's way easier this way.
fn generate_impl_tuple_calls<W: Write>(mut w: W) -> Result<(), Error> {
    for variant in 0..=16 {
        writeln!(w, "impl_tuple!({v}, ParseTuple{v}Error;", v = variant)?;

        for field in 0..variant {
            writeln!(w, "\t{f}, T{f}, f{f}, E{f}, Field{f};", f = field)?;
        }

        writeln!(w, ");")?;
    }

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let tuple_calls = Path::new(&out_dir).join("tuple_calls.rs");
    let tuple_calls = File::create(tuple_calls).unwrap();
    generate_impl_tuple_calls(tuple_calls).unwrap();
}
