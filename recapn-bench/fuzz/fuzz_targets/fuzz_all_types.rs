#![no_main]

use std::env;

use fuzz_models::AllTypes;
use lib_recapn_bench::fuzzing::fuzz_all_types_target;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: AllTypes| fuzz_all_types_target(&data)
    .inspect_err(|e| println!("{e}"))
    .unwrap());
