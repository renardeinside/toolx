use clap::{CommandFactory, Parser, Subcommand};
use pyo3::prelude::*;

#[derive(Parser)]
#[command(name = "toolx", version, about = "A super simple CLI tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Outputs "hello"
    Hey,
}

#[pyfunction]
fn run_cli(args: Vec<String>) -> i32 {
    match Cli::try_parse_from(args) {
        Ok(cli) => match cli.command {
            Some(Commands::Hey) => {
                println!("hello");
                0
            }
            None => {
                // No subcommand: show help by default (nicer UX for a tiny CLI).
                let mut cmd = Cli::command();
                let _ = cmd.print_help();
                println!();
                0
            }
        },
        Err(e) => {
            let code = e.exit_code();
            let _ = e.print();
            code
        }
    }
}

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from toolx!".to_string()
}

/// A Python module implemented in Rust. The name of this module must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_cli, m)?)?;
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    Ok(())
}
