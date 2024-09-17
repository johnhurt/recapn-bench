use clap::{Parser, Subcommand};
use fuzz_flat::fuzz_flat;
use hello::hello_main;

mod fuzz_flat;
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

    /// Run some fuzzing tests on a capnp example on the all-types, flat struct
    FuzzFlat {
        #[arg(long)]
        valid: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Hello { packed }) => {
            hello_main(*packed);
        }
        Some(Commands::FuzzFlat { valid }) => {
            fuzz_flat(*valid);
        }
        None => {
            run_all();
        }
    }
}

fn run_all() {
    hello_main(true);
    hello_main(false);

    fuzz_flat(true);
    fuzz_flat(false);
}
