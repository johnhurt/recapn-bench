use std::{collections::HashMap, env};

use eyre::Result;
use fuzz_models::AllTypes;

use crate::{all_types::validate_all_types, cargo_cli::cargo_fuzz};

pub const PACKED: &str = "PACKED";
pub const VALID: &str = "VALID";

pub fn launch_fuzz_all_types(valid: bool, packed: bool) -> Result<()> {
    cargo_fuzz(
        "all_types",
        [
            (PACKED.to_owned(), packed.to_string()),
            (VALID.to_owned(), valid.to_string()),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    )
}

pub fn fuzz_all_types_target(data: &AllTypes) -> Result<()> {
    let valid = env::var(VALID)
        .expect("VALID environment variable not set")
        .parse::<bool>()
        .expect("VALID environment variable not set to true/false");

    let packed = env::var(PACKED)
        .expect("PACKED environment variable not set")
        .parse::<bool>()
        .expect("PACKED environment variable not set to true/false");

    validate_all_types(data, valid, packed)
}
