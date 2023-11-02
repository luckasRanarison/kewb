use kewb::{error::Error, fs::write_table};

fn main() -> Result<(), Error> {
    write_table("bin/table.bin")?;
    Ok(())
}
