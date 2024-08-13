
#include "addressbook.capnp.h"
#include <capnp/message.h>
#include <capnp/serialize-packed.h>
#include <kj/io.h>
#include <iostream>
#include <vector>
#include <string>
#include "recapn_bench_cpp.h"

using addressbook::AddressBook;
using addressbook::Person;

using namespace std;

void writeAddressBook(kj::OutputStream &output)
{
    ::capnp::MallocMessageBuilder message;

    AddressBook::Builder addressBook = message.initRoot<AddressBook>();
    {
        ::capnp::List<Person>::Builder people = addressBook.initPeople(2);

        Person::Builder alice = people[0];
        alice.setId(123);
        alice.setName("Alice");
        alice.setEmail("alice@example.com");
        // Type shown for explanation purposes; normally you'd use auto.
        ::capnp::List<Person::PhoneNumber>::Builder alicePhones =
            alice.initPhones(1);
        alicePhones[0].setNumber("555-1212");
        alicePhones[0].setType(Person::PhoneNumber::Type::MOBILE);
        alice.getEmployment().setSchool("MIT");

        Person::Builder bob = people[1];
        bob.setId(456);
        bob.setName("Bob");
        bob.setEmail("bob@example.com");
        auto bobPhones = bob.initPhones(2);
        bobPhones[0].setNumber("555-4567");
        bobPhones[0].setType(Person::PhoneNumber::Type::HOME);
        bobPhones[1].setNumber("555-7654");
        bobPhones[1].setType(Person::PhoneNumber::Type::WORK);
        bob.getEmployment().setUnemployed();
    }

    writePackedMessage(output, message);
}

void printAddressBook(kj::BufferedInputStream &input)
{
    ::capnp::PackedMessageReader message(input);

    AddressBook::Reader addressBook = message.getRoot<AddressBook>();

    for (Person::Reader person : addressBook.getPeople())
    {
        std::cout << person.getName().cStr() << ": "
                  << person.getEmail().cStr() << std::endl;
        for (Person::PhoneNumber::Reader phone : person.getPhones())
        {
            const char *typeName = "UNKNOWN";
            switch (phone.getType())
            {
            case Person::PhoneNumber::Type::MOBILE:
                typeName = "mobile";
                break;
            case Person::PhoneNumber::Type::HOME:
                typeName = "home";
                break;
            case Person::PhoneNumber::Type::WORK:
                typeName = "work";
                break;
            }
            std::cout << "  " << typeName << " phone: "
                      << phone.getNumber().cStr() << std::endl;
        }
        Person::Employment::Reader employment = person.getEmployment();
        switch (employment.which())
        {
        case Person::Employment::UNEMPLOYED:
            std::cout << "  unemployed" << std::endl;
            break;
        case Person::Employment::EMPLOYER:
            std::cout << "  employer: "
                      << employment.getEmployer().cStr() << std::endl;
            break;
        case Person::Employment::SCHOOL:
            std::cout << "  student at: "
                      << employment.getSchool().cStr() << std::endl;
            break;
        case Person::Employment::SELF_EMPLOYED:
            std::cout << "  self-employed" << std::endl;
            break;
        }
    }
}

int hello(void *buf, int len)
{
    vector<string> msg{"Hello", "FFI", "World", "from", "VS Code", "and the C++ extension!"};

    for (const string &word : msg)
    {
        cout << word << " ";
    }
    cout << endl;

    return 0;
}

int write(void *buf, int len)
{
    try
    {
        kj::ArrayOutputStream stream(kj::ArrayPtr<kj::byte>((kj::byte *)buf, len));
        writeAddressBook(stream);
        return (int)stream.getArray().size();
    }
    catch (const std::exception &e)
    {
        // catch anything thrown within try block that derives from std::exception
        std::cerr << e.what();
        return -1;
    }
}

int read(void *buf, int len)
{
    try
    {
        kj::ArrayInputStream stream(kj::ArrayPtr<kj::byte>((kj::byte *)buf, len));
        printAddressBook(stream);
        return 0;
    }
    catch (const std::exception &e)
    {
        // catch anything thrown within try block that derives from std::exception
        std::cerr << e.what();
        return -1;
    }
}