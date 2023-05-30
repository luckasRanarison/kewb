use std::io;

use cli::{command::Commands, Cli};

fn main() -> Result<(), io::Error> {
    let program = Cli::parse_args();

    match &program.command {
        Some(Commands::Solve {
            max,
            timeout,
            scramble,
            details,
        }) => Commands::solve(scramble, *max, *timeout, *details),
        Some(Commands::Scramble { number }) => Commands::scramble(*number),
        _ => Ok(()),
    }
}
