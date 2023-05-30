use std::{env, fs, io, path::Path, time::Instant};

use bincode::{config, decode_from_slice, encode_to_vec};

use crate::{
    moves::{
        get_co_table, get_cp_table, get_e_combo_table, get_e_ep_table, get_eo_table,
        get_ud_ep_table, MoveTable,
    },
    pruning::{get_prune_table, PruningTable},
    utils::{ALL_MOVES, PHASE2_MOVES},
};

pub fn write_move_table(path: &Path) -> Result<(), io::Error> {
    println!("Writing move table to disk...");
    let start = Instant::now();
    let co = get_co_table();
    let eo = get_eo_table();
    let e_combo = get_e_combo_table();
    let cp = get_cp_table();
    let ep = get_ud_ep_table();
    let e_ep = get_e_ep_table();
    let move_table = MoveTable {
        co,
        eo,
        e_combo,
        cp,
        ep,
        e_ep,
    };
    let config = config::standard();
    let encoded = encode_to_vec(move_table, config).expect("Error when encoding tables");
    let cache_path = path.join("cache");

    if !fs::metadata(&cache_path).is_ok() {
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
    let co_e = get_prune_table(get_co_table, get_e_combo_table, &ALL_MOVES);
    let eo_e = get_prune_table(get_eo_table, get_e_combo_table, &ALL_MOVES);
    let cp_e = get_prune_table(get_cp_table, get_e_ep_table, &PHASE2_MOVES);
    let ep_e = get_prune_table(get_ud_ep_table, get_e_ep_table, &PHASE2_MOVES);
    let pruning_table = PruningTable {
        co_e,
        eo_e,
        cp_e,
        ep_e,
    };
    let config = config::standard();
    let encoded = encode_to_vec(pruning_table, config).expect("Error when encoding tables");
    let cache_path = path.join("cache");

    if !fs::metadata(&cache_path).is_ok() {
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

    if !fs::metadata(&current_path.join("cache/move_table.bin")).is_ok() {
        println!("Move table not found");
        write_move_table(current_path)?;
    }

    let config = config::standard();
    let encoded = fs::read(&current_path.join("cache/move_table.bin")).unwrap();
    let (move_table, _) = decode_from_slice(&encoded, config).unwrap();

    if !fs::metadata(&current_path.join("cache/pruning_table.bin")).is_ok() {
        println!("Pruning table not found");
        write_pruning_table(current_path)?;
    }

    let encoded = fs::read(&current_path.join("cache/pruning_table.bin")).unwrap();
    let (pruning_table, _) = decode_from_slice(&encoded, config).unwrap();

    Ok((move_table, pruning_table))
}
