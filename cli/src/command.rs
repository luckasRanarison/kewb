use std::{io, time::Instant};

use clap::{arg, Subcommand};
use cube::{
    moves::{scramble_from_string, Move},
    state::State,
    utils::generate_random_state,
};
use spinners::Spinner;
use two_phase::{fs::read_table, Solver};

use crate::validator::validate_scramble;

#[derive(Subcommand)]
pub enum Commands {
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

    #[command(about = "genrate scramble")]
    Scramble {
        #[arg(short, long, default_value_t = 1)]
        number: usize,
    },
}

impl Commands {
    pub fn solve(
        scramble: &str,
        max: u8,
        timeout: Option<f32>,
        details: bool,
    ) -> Result<(), io::Error> {
        let (move_table, pruning_table) = read_table()?;
        let mut solver = Solver::new(&move_table, &pruning_table, max, timeout);
        let scramble_move = scramble_from_string(scramble).unwrap();
        let start = Instant::now();
        let mut spinner = Spinner::new(spinners::Spinners::Dots, "Solving".to_owned());
        let solution = solver.solve(State::from(&scramble_move));
        let end = Instant::now();
        spinner.stop_with_newline();

        match &solver.timeout {
            Some(timeout) => println!("Timeout: {:.2}s", timeout.as_secs_f32()),
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

    pub fn scramble(number: usize) -> Result<(), io::Error> {
        let mut spinner = Spinner::new(spinners::Spinners::Dots, "Generating scramble".to_owned());
        let mut scrambles = Vec::new();

        let start = Instant::now();
        let (move_table, pruning_table) = read_table()?;
        for _i in 0..number {
            let state = generate_random_state();
            let mut solver = Solver::new(&move_table, &pruning_table, 24, None);
            let scramble = solver.solve(state).unwrap().get_all_moves();
            let scramble: Vec<Move> = scramble.iter().rev().map(|m| m.get_inverse()).collect();
            scrambles.push(scramble)
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
}
