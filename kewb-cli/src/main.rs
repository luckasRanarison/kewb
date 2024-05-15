use clap::{arg, command, Parser, Subcommand, ValueEnum};
use crossterm::{
    cursor::{MoveLeft, MoveRight, MoveUp},
    execute,
    style::{Attribute, Color as TermColor, SetBackgroundColor, Stylize},
};
use kewb::{
    error::Error,
    fs::{decode_table, write_table},
    generators::*,
    scramble::{scramble_from_state, scramble_from_str},
    Color,
};
use kewb::{CubieCube, FaceCube, Solver};
use spinners::Spinner;
use std::{
    io::{self, stdout},
    process,
    time::Instant,
};

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
    #[command(about = "solves the cube using two-phase algorithm")]
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

    #[command(about = "generates scramble")]
    Scramble {
        #[arg(default_value = "random")]
        state: State,

        #[arg(short, long, default_value_t = 1)]
        number: usize,

        #[arg(short, long)]
        preview: bool,
    },

    #[command(about = "generates the table used by the solver")]
    Table { path: String },
}

#[derive(ValueEnum, Clone)]
enum State {
    Random,
    CrossSolved,
    F2LSolved,
    OllSolved,
    OllCrossSolved,
    EdgesSolved,
    CornersSolved,
}

fn solve(
    scramble: &Option<String>,
    facelet: &Option<String>,
    max: u8,
    timeout: Option<f32>,
    details: bool,
) -> Result<(), Error> {
    if let Some(scramble) = scramble {
        solve_scramble(scramble, max, timeout, details)?;
    } else if let Some(facelet) = facelet {
        solve_facelet(facelet, max, timeout, details)?;
    }

    Ok(())
}

fn solve_state(
    state: CubieCube,
    max: u8,
    timeout: Option<f32>,
    details: bool,
) -> Result<(), Error> {
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
    let scramble = scramble_from_str(scramble)?;
    let state = CubieCube::from(&scramble);

    solve_state(state, max, timeout, details)
}

fn solve_facelet(facelet: &str, max: u8, timeout: Option<f32>, details: bool) -> Result<(), Error> {
    if let Ok(face_cube) = FaceCube::try_from(facelet) {
        match CubieCube::try_from(&face_cube) {
            Ok(state) => Ok(solve_state(state, max, timeout, details)?),
            Err(_) => Err(Error::InvalidFaceletValue),
        }
    } else {
        Err(Error::InvalidFaceletString)
    }
}

fn color_to_termcolor(color: Color) -> TermColor {
    match color {
        Color::U => TermColor::White,
        Color::R => TermColor::Red,
        Color::F => TermColor::Green,
        Color::D => TermColor::Yellow,
        Color::L => TermColor::Magenta,
        Color::B => TermColor::Blue,
    }
}

fn print_face(face: &[Color], offset: u16) -> Result<(), io::Error> {
    for i in 0..3 {
        let layer = format!(
            "{}  {}  {}  {}",
            SetBackgroundColor(color_to_termcolor(face[3 * i])),
            SetBackgroundColor(color_to_termcolor(face[(3 * i) + 1])),
            SetBackgroundColor(color_to_termcolor(face[(3 * i) + 2])),
            SetBackgroundColor(TermColor::Reset)
        );

        println!("{layer}");

        if offset != 0 {
            execute!(stdout(), MoveRight(offset))?;
        }
    }

    Ok(())
}

fn print_facelet(facelet: &FaceCube) -> Result<(), io::Error> {
    let stdout = stdout();

    println!();
    execute!(&stdout, MoveRight(6))?;
    print_face(&facelet.f[0..9], 6)?; // U (white)
    execute!(&stdout, MoveLeft(6))?;
    print_face(&facelet.f[36..45], 0)?; // L (orange)
    execute!(&stdout, MoveRight(6), MoveUp(3))?;
    print_face(&facelet.f[18..27], 6)?; // F (green)
    execute!(&stdout, MoveLeft(12), MoveUp(3), MoveRight(12))?;
    print_face(&facelet.f[9..18], 12)?; // R (red)
    execute!(&stdout, MoveLeft(12), MoveUp(3), MoveRight(18))?;
    print_face(&facelet.f[45..54], 18)?; // B (blue)
    execute!(&stdout, MoveLeft(12))?;
    print_face(&facelet.f[27..36], 6)?; // D (yellow)
    execute!(&stdout, MoveLeft(12))?;
    println!();

    Ok(())
}

fn scramble(state: &State, number: usize, preview: bool) -> Result<(), Error> {
    let table = decode_table(TABLE)?;
    let start = Instant::now();
    let mut spinner = Spinner::new(spinners::Spinners::Dots, "Generating scramble".to_owned());
    let mut solver = Solver::new(&table, 25, None);
    let mut scrambles = Vec::new();
    let mut states = Vec::new();

    for _ in 0..number {
        let state = match state {
            State::Random => generate_random_state(),
            State::CrossSolved => generate_state_cross_solved(),
            State::F2LSolved => generate_state_f2l_solved(),
            State::OllSolved => generate_state_oll_solved(),
            State::OllCrossSolved => generate_state_oll_cross_solved(),
            State::EdgesSolved => generate_state_edges_solved(),
            State::CornersSolved => generate_state_corners_solved(),
        };
        let scramble = scramble_from_state(state, &mut solver)?;

        states.push(state);
        scrambles.push(scramble);
        solver.clear();
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

        if preview {
            let facelet = FaceCube::try_from(&states[i])?;
            print_facelet(&facelet)?;
        }
    }

    println!("Done in {}s", (end - start).as_secs_f32());

    Ok(())
}

fn table(path: &str) -> Result<(), Error> {
    let mut spinner = Spinner::new(spinners::Spinners::Dots, "Generating scramble".to_owned());
    let start = Instant::now();

    write_table(path)?;

    let end = Instant::now();
    spinner.stop_with_newline();

    println!("Done in {}s", (end - start).as_secs_f32());

    Ok(())
}

fn main() {
    let program = Cli::parse();

    let result = match &program.command {
        Some(Commands::Solve {
            scramble,
            facelet,
            max,
            timeout,
            details,
        }) => solve(scramble, facelet, *max, *timeout, *details),
        Some(Commands::Scramble {
            state,
            number,
            preview,
        }) => scramble(state, *number, *preview),
        Some(Commands::Table { path }) => table(path),
        _ => Ok(()),
    };

    if let Err(error) = result {
        let styled = "error:".with(TermColor::Red).attribute(Attribute::Bold);
        eprintln!("{styled} {error}");
        process::exit(1);
    }
}
