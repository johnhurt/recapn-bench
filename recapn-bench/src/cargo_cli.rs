#![allow(dead_code)]

use eyre::{eyre, Context, ContextCompat, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use which::which;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::OnceLock;

static CARGO_PATH: Lazy<PathBuf> = Lazy::new(|| {
    which("cargo")
        .expect("Cargo needs to be installed XD")
        .into()
});

#[derive(Debug, Clone, Copy)]
enum CargoCommand {
    Fuzz,
}

impl CargoCommand {
    fn to_args(self) -> &'static [&'static str] {
        match self {
            CargoCommand::Fuzz => &["+nightly", "fuzz", "run"],
        }
    }
}

impl ToString for CargoCommand {
    fn to_string(&self) -> String {
        self.to_args().iter().join(" ")
    }
}

fn cargo_cli_command(
    command: CargoCommand,
    env: HashMap<String, String>,
    extra_args: &[&str],
    suppress_output: bool,
) -> Result<String> {
    let mut proc = Command::new(&*CARGO_PATH);
    proc.args(
        command
            .to_args()
            .iter()
            .copied()
            .chain(extra_args.iter().cloned()),
    );

    env.iter().for_each(|(k, v)| {
        proc.env(k, v);
    });

    let mut child = proc
        .stdout(Stdio::piped())
        .spawn()
        .wrap_err("Failed to create cargo process")?;

    let child_stdout = child.stdout.take().wrap_err("Failed to capture stdout")?;

    // This captures the stdout as well as printing it to the console.
    let reader = BufReader::new(child_stdout);
    let std_out_contents = reader
        .lines()
        .map_while(Result::ok)
        .inspect(|line| {
            if !suppress_output {
                println!("{line}")
            }
        })
        .collect::<Vec<_>>();

    if !child
        .wait()
        .wrap_err("Error waiting for process to exit")?
        .success()
    {
        Err(eyre!("Non-Zero Exit code"))
    } else {
        Ok(std_out_contents.into_iter().join("\n"))
    }
}

pub fn cargo_fuzz(target: &str, env: HashMap<String, String>) -> Result<()> {
    cargo_cli_command(CargoCommand::Fuzz, env, &[target], false).map(|_| {})
}
