use clap::{arg, command, Parser, Subcommand};
use kewb::{
    error::Error,
    fs::decode_table,
    utils::{generate_random_state, scramble_from_string},
};
use kewb::{FaceCube, Move, Solver, State};
use spinners::Spinner;
use std::time::Instant;

const TABLE: &[u8] = include_bytes!("../bin/table.bin");

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
    #[clap(group(
    clap::ArgGroup::new("state")
        .required(true)
        .args(&["scramble", "facelet"]),
    ))]
    Solve {
        #[arg(short, long)]
        scramble: Option<String>,

        #[arg(short, long)]
        facelet: Option<String>,

        #[arg(short, long, default_value_t = 23)]
        max: u8,

        #[arg(short, long)]
        timeout: Option<f32>,

        #[arg(short, long)]
        details: bool,
    },

    #[command(about = "genrate scramble")]
    Scramble {
        #[arg(short, long, default_value_t = 1)]
        number: usize,
    },
}

fn solve_state(state: State, max: u8, timeout: Option<f32>, details: bool) -> Result<(), Error> {
    let table = decode_table(TABLE)?;
    let mut solver = Solver::new(&table, max, timeout);
    let mut spinner = Spinner::new(spinners::Spinners::Dots, "Solving".to_owned());

    let start = Instant::now();
    let solution = solver.solve(state);
    let end = Instant::now();

    spinner.stop_with_newline();

    match timeout {
        Some(timeout) => println!("Timeout: {:.2}s", timeout),
        _ => println!("Finished in {}s", (end - start).as_secs_f32()),
    }

    match solution {
        Some(value) => {
            if details {
                println!("Phase 1: {}", value.phase1_to_string());
                println!("Phase 2: {}", value.phase2_to_string());
            }

            println!("Solution: {value}");
            println!("Move count: {}", value.len())
        }
        None => println!("No solution found"),
    }

    Ok(())
}

fn solve_scramble(
    scramble: &str,
    max: u8,
    timeout: Option<f32>,
    details: bool,
) -> Result<(), Error> {
    if let Some(scramble) = scramble_from_string(scramble) {
        let state = State::from(&scramble);
        Ok(solve_state(state, max, timeout, details)?)
    } else {
        Err(Error::InvalidScramble)
    }
}

fn solve_facelet(facelet: &str, max: u8, timeout: Option<f32>, details: bool) -> Result<(), Error> {
    if let Ok(face_cube) = FaceCube::try_from(facelet) {
        match State::try_from(&face_cube) {
            Ok(state) => Ok(solve_state(state, max, timeout, details)?),
            Err(_) => Err(Error::InvalidFaceletValue),
        }
    } else {
        Err(Error::InvalidFaceletString)
    }
}

fn scramble(number: usize) -> Result<(), Error> {
    let mut spinner = Spinner::new(spinners::Spinners::Dots, "Generating scramble".to_owned());
    let mut scrambles = Vec::new();
    let table = decode_table(TABLE)?;
    let start = Instant::now();

    for _ in 0..number {
        let mut solver = Solver::new(&table, 25, None);
        let state = generate_random_state();
        let scramble = solver.solve(state).unwrap().get_all_moves();
        let scramble: Vec<Move> = scramble.iter().rev().map(|m| m.get_inverse()).collect();
        scrambles.push(scramble);
    }

    let end = Instant::now();
    spinner.stop_with_newline();

    for (i, scramble) in scrambles.iter().enumerate() {
        let stringified = scramble
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        println!(
            "{}{stringified}",
            match number {
                1 => "".to_owned(),
                _ => format!("{} - ", i + 1),
            }
        );
    }

    println!("Done in {}s", (end - start).as_secs_f32());

    Ok(())
}

fn main() -> Result<(), Error> {
    let program = Cli::parse();

    match &program.command {
        Some(Commands::Solve {
            max,
            timeout,
            scramble,
            facelet,
            details,
        }) => {
            let mut error = None;

            if let Some(scramble) = scramble {
                error = solve_scramble(scramble, *max, *timeout, *details).err()
            } else if let Some(facelet) = facelet {
                error = solve_facelet(facelet, *max, *timeout, *details).err()
            }

            if let Some(error) = error {
                println!("Error: {}", error);
            }

            Ok(())
        }
        Some(Commands::Scramble { number }) => scramble(*number),
        _ => Ok(()),
    }
}
