use kewb::{error::Error, fs::write_table};
use std::fs::read;

fn main() -> Result<(), Error> {
    let table = read("bin/table.bin")?;

    if table.is_empty() {
        write_table("bin/table.bin")?;
    }

    Ok(())
}
