# Bench Tests for Recapn

Recapn is a rewrite of the capnproto compiler and runtime for rust. This repo 
contains some tools to ensure that the recapn is:

1. **compatible** with capnproto-c++ as well as the existing rust implementation
2. **at parity** with the rust implementation and (eventually) the c++ version
3. **safe** withstands fuzzed input
4. **performant** (or as close as possible) as the existing implementations

## Usage

Everything in this project is run through the `recapn-bench` rust binary. You 
can run all the verifications by running `recapn-bench` with no arguments. Other 
cli options are available to run individual test types with other options.

## Requirements

There are a few requirements you need in order to run the benchmarks.

- Linux
- capnproto*
- rust
- cargo
- make
- cmake
- gcc

> *The dependency on capnproto is what provides the c++ implementation. You need to have `capnpc` on your path

## Progress

This (and recapn) is a work in progress. Below is a matrix of bench tests that are covered by this project.

| Category       | Args             | Test                                                                                 | Supported | Status | Notes                                                       |
| -------------- | ---------------- | ------------------------------------------------------------------------------------ | --------- | ------ | ----------------------------------------------------------- |
| Basic          | `hello`          | Run the addressbook example in all capnps, and compare the output                    | ✅         | ✅      | Done                                                        |
| Fuzz All Types | `fuzz-all-types` | Fuzz valid inputs on the all-types struct with all supported types and verify output | ✅         | 🕙      | Only scalar (primitive + enum) types are verified right now |

