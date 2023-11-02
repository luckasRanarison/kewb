use super::utils::DataTable;
use crate::error::Error;
use bincode::{
    config::{self, Configuration},
    decode_from_slice, encode_to_vec,
    error::DecodeError,
};
use std::{fs, path::Path};

const CONFIG: Configuration = config::standard();

pub fn write_table<P>(path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let table = DataTable::default();
    let encoded = encode_to_vec(table, CONFIG)?;

    fs::write(path, encoded)?;

    Ok(())
}

pub fn read_table<P>(path: P) -> Result<DataTable, Error>
where
    P: AsRef<Path>,
{
    let encoded = fs::read(path)?;
    let table = decode_table(&encoded)?;

    Ok(table)
}

pub fn decode_table(bytes: &[u8]) -> Result<DataTable, Error> {
    let (decoded, written) = decode_from_slice(bytes, CONFIG)?;
    let additional = bytes.len() - written;

    if additional != 0 {
        return Err(DecodeError::UnexpectedEnd { additional })?;
    }

    Ok(decoded)
}
