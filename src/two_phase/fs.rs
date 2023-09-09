use super::{moves::MoveTable, pruning::PruningTable};
use bincode::{config, decode_from_slice, encode_to_vec};
use std::{env, fs, io, path::Path, time::Instant};

pub fn write_move_table(path: &Path) -> Result<(), io::Error> {
    println!("Writing move table to disk...");

    let start = Instant::now();
    let move_table = MoveTable::default();
    let config = config::standard();
    let encoded = encode_to_vec(move_table, config).expect("Error when encoding tables");
    let cache_path = path.join("cache");

    if fs::metadata(&cache_path).is_err() {
        fs::create_dir(&cache_path)?;
    }

    fs::write(cache_path.join("move_table.bin"), encoded)?;

    let end = Instant::now();
    println!("Succes, finished in {:.2}s", (end - start).as_secs_f32());

    Ok(())
}

pub fn write_pruning_table(path: &Path) -> Result<(), io::Error> {
    println!("Writing pruning table to disk...");

    let start = Instant::now();
    let pruning_table = PruningTable::default();
    let config = config::standard();
    let encoded = encode_to_vec(pruning_table, config).expect("Error when encoding tables");
    let cache_path = path.join("cache");

    if fs::metadata(&cache_path).is_err() {
        fs::create_dir(&cache_path)?;
    }

    fs::write(cache_path.join("pruning_table.bin"), encoded)?;

    let end = Instant::now();
    println!("Succes, finished in {:.2}s", (end - start).as_secs_f32());

    Ok(())
}

pub fn read_table() -> Result<(MoveTable, PruningTable), io::Error> {
    let current_exe = env::current_exe().expect("Failed to get current executable path");
    let current_path = &current_exe
        .parent()
        .expect("Failed to get current executable directory");

    if fs::metadata(current_path.join("cache/move_table.bin")).is_err() {
        println!("Move table not found");
        write_move_table(current_path)?;
    }

    let config = config::standard();
    let encoded = fs::read(current_path.join("cache/move_table.bin"))?;
    let (move_table, _) = decode_from_slice(&encoded, config).unwrap();

    if fs::metadata(current_path.join("cache/pruning_table.bin")).is_err() {
        println!("Pruning table not found");
        write_pruning_table(current_path)?;
    }

    let encoded = fs::read(current_path.join("cache/pruning_table.bin"))?;
    let (pruning_table, _) = decode_from_slice(&encoded, config).unwrap();

    Ok((move_table, pruning_table))
}
