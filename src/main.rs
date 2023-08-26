use std::{io, time::Instant};

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
        #[arg(short, long, value_parser = validate_scramble)]
        scramble: Option<String>,

        #[arg(short, long, value_parser = validate_facelet)]
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
) -> Result<(), io::Error> {
    let scramble_moves = scramble_from_string(scramble).unwrap();
    let state = State::from(&scramble_moves);
    solve_state(state, max, timeout, details)
}

fn solve_facelet(
    facelet: &str,
    max: u8,
    timeout: Option<f32>,
    details: bool,
) -> Result<(), io::Error> {
    let face_cube = kewb::FaceCube::try_from(facelet).unwrap();
    let state = kewb::State::try_from(&face_cube).unwrap();
    solve_state(state, max, timeout, details)
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

fn validate_scramble(scramble: &str) -> Result<String, String> {
    match scramble_from_string(&scramble) {
        Some(_) => Ok(scramble.to_owned()),
        None => Err("Invalid scramble".to_owned()),
    }
}

fn validate_facelet(faces: &str) -> Result<String, String> {
    match FaceCube::try_from(faces) {
        Ok(_) => Ok(faces.to_owned()),
        Err(_) => Err("Invalid facelet string".to_owned()),
    }
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
            if let Some(scramble) = scramble {
                return solve_scramble(scramble, *max, *timeout, *details);
            } else if let Some(facelet) = facelet {
                return solve_facelet(facelet, *max, *timeout, *details);
            }
            unreachable!();
        }
        Some(Commands::Scramble { number }) => scramble(*number),
        _ => Ok(()),
    }
}
