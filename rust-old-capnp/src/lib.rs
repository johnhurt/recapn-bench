include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::io::Write;

use crate::addressbook_capnp::{address_book, person};
use capnp::serialize_packed;

pub fn write_address_book<W: Write>(target: &mut W) -> ::capnp::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();
    {
        let address_book = message.init_root::<address_book::Builder>();

        let mut people = address_book.init_people(2);

        {
            let mut alice = people.reborrow().get(0);
            alice.set_id(123);
            alice.set_name("Alice");
            alice.set_email("alice@example.com");
            {
                let mut alice_phones = alice.reborrow().init_phones(1);
                alice_phones.reborrow().get(0).set_number("555-1212");
                alice_phones
                    .reborrow()
                    .get(0)
                    .set_type(person::phone_number::Type::Mobile);
            }
            alice.get_employment().set_school("MIT");
        }

        {
            let mut bob = people.get(1);
            bob.set_id(456);
            bob.set_name("Bob");
            bob.set_email("bob@example.com");
            {
                let mut bob_phones = bob.reborrow().init_phones(2);
                bob_phones.reborrow().get(0).set_number("555-4567");
                bob_phones
                    .reborrow()
                    .get(0)
                    .set_type(person::phone_number::Type::Home);
                bob_phones.reborrow().get(1).set_number("555-7654");
                bob_phones
                    .reborrow()
                    .get(1)
                    .set_type(person::phone_number::Type::Work);
            }
            bob.get_employment().set_unemployed(());
        }
    }

    serialize_packed::write_message(target, &message)
}

pub fn print_address_book(src: &[u8]) -> ::capnp::Result<()> {
    let message_reader =
        serialize_packed::read_message(src, ::capnp::message::ReaderOptions::new())?;
    let address_book = message_reader.get_root::<address_book::Reader>()?;

    for person in address_book.get_people()? {
        println!(
            "{}: {}",
            person.get_name()?.to_str()?,
            person.get_email()?.to_str()?
        );
        for phone in person.get_phones()? {
            let type_name = match phone.get_type() {
                Ok(person::phone_number::Type::Mobile) => "mobile",
                Ok(person::phone_number::Type::Home) => "home",
                Ok(person::phone_number::Type::Work) => "work",
                Err(::capnp::NotInSchema(_)) => "UNKNOWN",
            };
            println!("  {} phone: {}", type_name, phone.get_number()?.to_str()?);
        }
        match person.get_employment().which() {
            Ok(person::employment::Unemployed(())) => {
                println!("  unemployed");
            }
            Ok(person::employment::Employer(employer)) => {
                println!("  employer: {}", employer?.to_str()?);
            }
            Ok(person::employment::School(school)) => {
                println!("  student at: {}", school?.to_str()?);
            }
            Ok(person::employment::SelfEmployed(())) => {
                println!("  self-employed");
            }
            Err(::capnp::NotInSchema(_)) => {}
        }
    }
    Ok(())
}
