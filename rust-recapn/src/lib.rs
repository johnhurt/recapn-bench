pub mod address_book;
pub mod all_types;

pub mod gen {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
