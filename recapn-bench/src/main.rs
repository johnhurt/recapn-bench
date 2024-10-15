use clap::{Parser, Subcommand};
use eyre::{Ok, Result};
use fuzzing::launch_fuzz_all_types;
use hello::hello_main;

mod all_types;
mod cargo_cli;
mod fuzzing;
mod hello;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the addressbook example for all capnp implementations and verify the
    /// output
    Hello {
        #[arg(short, long)]
        packed: bool,
    },

    /// Run some fuzzing tests on a capnp example on the all-types struct
    FuzzAllTypes {
        /// Use valid values for all-types only
        #[arg(long)]
        valid: bool,

        #[arg(short, long)]
        packed: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Hello { packed }) => hello_main(*packed),
        Some(Commands::FuzzAllTypes { packed, valid }) => launch_fuzz_all_types(*valid, *packed),
        None => run_all(),
    }
}

fn run_all() -> Result<()> {
    hello_main(true)?;
    hello_main(false)?;

    Ok(())
}
