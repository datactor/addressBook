#![allow(non_snake_case)]
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Ord, PartialOrd)]
pub struct Person {
    pub name: String,
    pub address: String,
}

impl Person {
    pub fn new(inputName: &String, inputAddress: &String) -> Self {
        Person {
            name: inputName.to_string(),
            address: inputAddress.to_string(),
        }
    }
}

#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum Divisions {
    Engineering,
    Sales,
    Legal,
    Develops,
    Managements,
    CustomerService,
    Etc,
}

fn divChecker(division: String) -> Divisions {
    match division.as_str() {
        "Engineering" => return Divisions::Engineering,
        "Sales" => return Divisions::Sales,
        "Legal" => return Divisions::Legal,
        "Develops" => return Divisions::Develops,
        "Managements" => return Divisions::Managements,
        "CustomerService" => return Divisions::CustomerService,
        "Etc" => return Divisions::Etc,
        _ => panic!("{:?} is not valid department", division)
    }
}

#[derive(PartialEq, Debug, Eq)]
pub struct AddressBook {
    pub inner: HashMap<Person, Divisions>,
}

impl AddressBook {
    pub fn new() -> Self {
        AddressBook {
            inner: HashMap::<Person, Divisions>::new(),
            // inner: HashMap::<Person, Divisions>::from([(person, division)]),
        }
    }

    pub fn readBook(&mut self, fileName: &str) {
        let fileData: String = fs::read_to_string(fileName)
            .expect("File reading failure");
        let row = fileData.split("\n");
        for r in row {
            if r.is_empty() == false {
                let mut divSlice = r.split(": ");
                let div = divSlice.next().unwrap().to_string();
                let person = divSlice.next().unwrap();
                let mut personSlice = person.split(", ");
                let name = personSlice.next().unwrap().to_string();
                let address = personSlice.next().unwrap().to_string();
                self.inner.insert(Person::new(&name, &address), divChecker(div));
            }
        }
    }

    // pub fn pushPerson(&mut self, person: Person, div: &Divisions) {
    //     self.db.insert(person.clone(), *div).expect("DB insert failure");
    //     println!("[{:?}, {} :{}] is added", div, &person.name, &person.address);
    // }
    //
    // pub fn delPerson(&mut self, person: Person) {
    //     self.db.remove(&person).expect("DB remove failure");
    // }

    pub fn saveBook(&self, fileName: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(fileName)
            .expect("File opening failure");

        // // sorting
        // let mut sorted: Vec<_> = self.db.iter().collect();
        // sorted.sort_by_key(|n| n.0);
        // sorted.sort_by_key(|n| n.1);
        // for data in sorted {
        //     let writing = format!("{:?}: {}, {}\n", data.1, data.0.name, data.0.address);
        //     file.write(writing.as_bytes()).expect("File writing failure");
        // }

        for data in &self.inner {
            let writing = format!("{:?}: {}, {}\n", data.1, data.0.name, data.0.address);
            file.write(writing.as_bytes()).expect("File writing failure");
        }
        println!("File updated");
    }
}