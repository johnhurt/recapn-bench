use std::io::stdout;

use eyre::Result;
use rust_old_capnp::{
    print_address_book as print_address_book_old_rust,
    write_address_book as write_address_book_old_rust,
};
use rust_recapn::address_book::{
    print_address_book as print_address_book_recapn,
    write_address_book as write_address_book_recapn,
};

pub fn hello_main(packed: bool) -> Result<()> {
    println!("Get packed message from old rust capnp");
    let mut rs_old_target: Vec<u8> = vec![];
    write_address_book_old_rust(&mut rs_old_target, packed).unwrap();

    println!("Get what should be the same message from c++ capnp");
    let mut working_buffer = vec![0; rs_old_target.len()];
    let written = cpp_interop::write_addressbook(&mut working_buffer, packed) as usize;

    println!("Check the old rust and cpp versions emit the same packed message");
    assert_eq!(&working_buffer, &rs_old_target);
    println!("They do!");

    println!("Get what should be the same message from recapn");
    let mut recapn_target: Vec<u8> = vec![];
    write_address_book_recapn(&mut recapn_target, packed);

    println!("Check the recapn and other versions emit the same packed message");
    assert_eq!(recapn_target, rs_old_target);
    println!("They do!");

    println!("\nHere's the message interpreted by c++");
    assert_eq!(
        cpp_interop::read_addressbook(&mut working_buffer, packed),
        0
    );

    println!("\nHere's the message interpreted by old rust");
    print_address_book_old_rust(&working_buffer[0..written], packed).unwrap();

    println!("\nHere's the message interpreted by recapn");
    print_address_book_recapn(&recapn_target, &mut stdout(), packed).unwrap();

    Ok(())
}
