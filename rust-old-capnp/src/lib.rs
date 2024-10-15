mod address_book;
pub use address_book::*;

mod all_types;
pub use all_types::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
