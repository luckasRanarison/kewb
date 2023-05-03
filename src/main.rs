use std::{io, time::Instant};

use clap::{arg, Parser, Subcommand};
use cube::{moves::scramble_from_string, state::State};
use two_phase::Solver;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "solve the cube using two-phase algorithm")]
    Solve {
        #[arg(short, long, value_parser = validate_scramble)]
        scramble: String,

        #[arg(short, long, default_value_t = 23)]
        max: u8,

        #[arg(short, long)]
        timeout: Option<f32>,

        #[arg(short, long)]
        details: bool,
    },
}

fn validate_scramble(scramble: &str) -> Result<String, String> {
    match scramble_from_string(&scramble) {
        Some(_) => Ok(scramble.to_owned()),
        None => Err("Invalid scramble".to_owned()),
    }
}

fn main() -> Result<(), io::Error> {
    let program = Cli::parse();

    if let Some(Commands::Solve {
        max,
        timeout,
        scramble,
        details,
    }) = &program.command
    {
        let mut solver = Solver::new(*max, *timeout)?;
        let scramble_move = scramble_from_string(&scramble).unwrap();
        let start = Instant::now();
        let solution = solver.solve(State::from(&scramble_move));
        let end = Instant::now();

        match &solver.timeout {
            Some(timeout) => println!("Timeout: {:.2}s", timeout.as_secs_f32()),
            _ => println!("Finished in {}s", (end - start).as_secs_f32()),
        }

        match solution {
            Some(value) => {
                if *details {
                    println!("Phase 1: {}", value.phase1_to_string());
                    println!("Phase 2: {}", value.phase2_to_string());
                }

                println!("Solution: {value}");
                println!("Move count: {}", value.len())
            }
            None => println!("No solution found"),
        }
    }

    Ok(())
}
