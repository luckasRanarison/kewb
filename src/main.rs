use std::{fmt, io, time::Instant};

use clap::{arg, command, Parser, Subcommand};
use kewb::{
    fs::read_table,
    utils::{generate_random_state, scramble_from_string},
};
use kewb::{solve, FaceCube, Move, Solver, State};
use spinners::Spinner;

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

enum SolverError {
    InvalidScramble,
    InvalidFaceletString,
    InvalidFaceletValue,
    IOError(io::Error),
}

impl From<io::Error> for SolverError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl fmt::Display for SolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidScramble => write!(f, "Invalid scramble"),
            Self::InvalidFaceletString => write!(f, "Invalid facelet string"),
            Self::InvalidFaceletValue => write!(f, "Invalid facelet reperesentation"),
            Self::IOError(value) => value.fmt(f),
        }
    }
}

fn solve_state(
    state: State,
    max: u8,
    timeout: Option<f32>,
    details: bool,
) -> Result<(), io::Error> {
    let mut spinner = Spinner::new(spinners::Spinners::Dots, "Solving".to_owned());

    let start = Instant::now();
    let solution = solve(state, max, timeout);
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
) -> Result<(), SolverError> {
    if let Some(scramble) = scramble_from_string(scramble) {
        let state = State::from(&scramble);
        Ok(solve_state(state, max, timeout, details)?)
    } else {
        Err(SolverError::InvalidScramble)
    }
}

fn solve_facelet(
    facelet: &str,
    max: u8,
    timeout: Option<f32>,
    details: bool,
) -> Result<(), SolverError> {
    if let Ok(face_cube) = FaceCube::try_from(facelet) {
        match State::try_from(&face_cube) {
            Ok(state) => Ok(solve_state(state, max, timeout, details)?),
            Err(_) => Err(SolverError::InvalidFaceletValue),
        }
    } else {
        Err(SolverError::InvalidFaceletString)
    }
}

fn scramble(number: usize) -> Result<(), io::Error> {
    let mut spinner = Spinner::new(spinners::Spinners::Dots, "Generating scramble".to_owned());
    let mut scrambles = Vec::new();
    let (move_table, pruning_table) = read_table()?;
    let start = Instant::now();

    for _ in 0..number {
        let mut solver = Solver::new(&move_table, &pruning_table, 25, None);
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

fn main() -> Result<(), io::Error> {
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
                println!("error: {}", error);
            }

            Ok(())
        }
        Some(Commands::Scramble { number }) => scramble(*number),
        _ => Ok(()),
    }
}
