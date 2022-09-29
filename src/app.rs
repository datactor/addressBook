#![allow(non_snake_case)]
use crate::{structs, handler};
use std::fs::File;
use std::io::ErrorKind;

pub fn run(fileName: &str) {
    let mut addressBook = structs::AddressBook::new();
    let file = File::open(fileName);
    match file {
        Ok(f) => f,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("The file is not exists. Create the file");
            File::create(fileName).expect("File creating failure")
        },
        Err(e) => panic!("There was a problem opening the file: {:?}", e)
    };
    addressBook.readBook(fileName);
    println!(
        "===================================================\n\
        Hello! this is a text address book!\n\
        Input any keys to start\n\
        ===================================================",
    );
    let mut running = true;
    while running {
        match handler::inputString().to_lowercase().as_str() {
            _ => {
                loop {
                    println!("-----------------------------------------------------------------");
                    println!("[main]");
                    println!("1. Add a new personal data.");
                    println!("2. Print memorised data.");
                    println!("3. Search a personal data.");
                    println!("4. Remove a personal data.");
                    println!("5. Update a personal data.");
                    println!("6. Save the update.");
                    println!("0. Quit this program.");
                    println!("-----------------------------------------------------------------");
                    println!("Please input order number. (0 - 6)");

                    match handler::inputNum() {
                        1 => handler::addPerson(&mut addressBook),
                        2 => handler::getPeopleByDiv(&addressBook),
                        3 => handler::getPerson(&addressBook),
                        4 => handler::rmPerson(&mut addressBook),
                        5 => handler::updateAddress(&mut addressBook),
                        6 => addressBook.saveBook(fileName),
                        0 => {
                            println!("Quit");
                            running = false;
                            break
                        },
                        _ => {},
                    }
                }
            },
        }
    }
}