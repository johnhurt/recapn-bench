pub mod gen {
    include!(concat!(env!("OUT_DIR"), "/mod.rs"));
}

use std::{
    borrow::{Borrow, BorrowMut},
    io::Write,
};

use arena::ReadArena;
use gen::capnp_addressbook_capnp::*;
use io::{
    read_from_slice, read_from_stream, read_packed_from_stream, PackedStream, SegmentSet,
    SegmentSetTable, StreamOptions,
};
use message::Message;
use recapn::*;
use ty::Struct;

pub fn write_address_book<W: Write>(target: &mut W, packed: bool) {
    let mut message = Message::global();
    let mut builder = message.builder().init_struct_root::<AddressBook>();
    {
        let mut people = builder.people().init(2);

        {
            let mut alice = people.at(0).get();
            alice.id().set(123);
            alice.name().set(text!("Alice"));
            alice.email().set(text!("alice@example.com"));
            {
                let mut alice_phones = alice.borrow_mut().phones().init(1);
                let mut phone = alice_phones.borrow_mut().at(0).get();
                phone.number().set(text!("555-1212"));
                phone.r#type().set(person::phone_number::Type::Mobile);
            }
            let mut employment = alice.employment();
            let school = employment.school();
            let field = school.init_field();
            field.set_str("MIT");
        }

        {
            let mut bob = people.at(1).get();
            bob.id().set(456);
            bob.name().set(text!("Bob"));
            bob.email().set(text!("bob@example.com"));
            {
                let mut bob_phones = bob.borrow_mut().phones().init(2);
                bob_phones
                    .borrow_mut()
                    .at(0)
                    .get()
                    .number()
                    .set(text!("555-4567"));
                bob_phones
                    .borrow_mut()
                    .at(0)
                    .get()
                    .r#type()
                    .set(person::phone_number::Type::Home);
                bob_phones
                    .borrow_mut()
                    .at(1)
                    .get()
                    .number()
                    .set(text!("555-7654"));
                bob_phones
                    .borrow_mut()
                    .at(1)
                    .get()
                    .r#type()
                    .set(person::phone_number::Type::Work);
            }
            bob.employment().unemployed();
        }
    }

    if packed {
        recapn::io::write_message_packed(target, &message.segments().unwrap()).unwrap();
    } else {
        recapn::io::write_message(target, &message.segments().unwrap()).unwrap();
    }
}

pub fn print_address_book<W: Write>(
    src: &[u8],
    output: &mut W,
    packed: bool,
) -> ::recapn::Result<()> {
    let segments;

    if packed {
        let mut message_reader = PackedStream::new(src);
        segments = read_packed_from_stream(&mut message_reader, StreamOptions::default()).unwrap();
    } else {
        segments = read_from_stream(src, StreamOptions::DEFAULT).unwrap();
    }
    let message = message::Reader::new(&segments, message::ReaderOptions::default());
    let address_book = message.read_as_struct::<AddressBook>();

    for person in address_book.people() {
        write!(
            output,
            "{}: {}\n",
            person.name().as_str().unwrap(),
            person.email().as_str().unwrap()
        );
        for phone in person.phones() {
            let type_name = match phone.r#type() {
                Ok(person::phone_number::Type::Mobile) => "mobile",
                Ok(person::phone_number::Type::Home) => "home",
                Ok(person::phone_number::Type::Work) => "work",
                Err(::recapn::NotInSchema(_)) => "UNKNOWN",
            };
            write!(
                output,
                "  {} phone: {}\n",
                type_name,
                phone.number().as_str().unwrap()
            );
        }
        match person.employment().which() {
            Ok(person::employment::Which::Unemployed(())) => {
                write!(output, "  unemployed\n");
            }
            Ok(person::employment::Which::Employer(employer)) => {
                write!(output, "  employer: {}\n", employer.as_str().unwrap());
            }
            Ok(person::employment::Which::School(school)) => {
                write!(output, "  student at: {}\n", school.as_str().unwrap());
            }
            Ok(person::employment::Which::SelfEmployed(())) => {
                write!(output, "  self-employed\n");
            }
            Err(_) => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn test(#[case] packed: bool) {
        let mut t: Vec<u8> = vec![];
        write_address_book(&mut t, packed);

        let mut string_repr: Vec<u8> = vec![];

        print_address_book(&t, &mut string_repr, packed).unwrap();

        assert_eq!(
            String::from_utf8_lossy(&string_repr),
            "\
Alice: alice@example.com
  mobile phone: 555-1212
  student at: MIT
Bob: bob@example.com
  home phone: 555-4567
  work phone: 555-7654
  unemployed
"
        );
    }
}
