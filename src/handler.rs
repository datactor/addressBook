#![allow(non_snake_case)]
use std::io::stdin;
use crate::structs::{AddressBook, Divisions, Person};

pub fn inputString() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("String input failure");
    buffer.trim().parse().expect("Please try again")
}

pub fn inputNum() -> u8 {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("input error");
    match buffer.trim().parse() {
        Ok(num) => return num,
        Err(__) => return 255,
    }
}

pub fn divLoops() -> Divisions {
    println!("-----------------------------------------------------------------");
    println!("Please enter the number of followed divisions.");
    println!("1. Engineering");
    println!("2. Sales");
    println!("3. Legal");
    println!("4. Develops");
    println!("5. Managements");
    println!("6. CustomerService");
    println!("7. Etc");
    println!("-----------------------------------------------------------------");
    println!("Please input order number. (1 - 6)");
    loop {
        match inputNum() {
            1 => return Divisions::Engineering,
            2 => return Divisions::Sales,
            3 => return Divisions::Legal,
            4 => return Divisions::Develops,
            5 => return Divisions::Managements,
            6 => return Divisions::CustomerService,
            7 => return Divisions::Etc,
            _ => {},
        }
    }
}

pub fn addPerson(addressBook: &mut AddressBook) {
    println!("1. Add a new personal information.");

    let mut dataInputAgain = true;
    while dataInputAgain {
        println!("Input a name");
        let name = inputString();
        println!("Input an address");
        let address = inputString();
        let division = divLoops();

        let newPerson = Person::new(&name, &address);

        match addressBook.inner.contains_key(&newPerson) {
            true => {
                println!("The person is already registered in '{:?}'. Do you want to overwrite? Y/N",
                         addressBook.inner.get(&newPerson).unwrap());
                println!(
                    "*If you want to back to the main, \
                    type 'N' or to re-registration, type any others except Y/N"
                );
                match inputString().to_lowercase().as_str() {
                    "y" | "yes" => {
                        println!("Add ({}, {}) to {:?}", newPerson.name, newPerson.address, &division);
                        addressBook.inner.insert(newPerson, division);
                        println!("Registration success");
                        dataInputAgain = false
                    },
                    "n" | "no" => {
                        println!("back to the main");
                        break;
                    },
                    _ => continue
                }
            },
            _ => {
                println!("Add ({}, {}) to {:?}",
                         newPerson.name, newPerson.address, &division);
                addressBook.inner.insert(newPerson, division);
                println!("Registration success");
                dataInputAgain = false;
            },
        }
    }
}

pub fn getPeopleByDiv(addressBook: &AddressBook) {
    println!("2. Check database");
    println!("-----------------------------------------------------------------");
    println!("1. Print all data.");
    println!("2. Print specific departmental data.");
    println!("0. Go back to the main.");
    println!("-----------------------------------------------------------------");
    println!("Please input order number. (0 - 2)");
    let mut inputAgain = true;
    while inputAgain {
        let mut sorted: Vec<_> = addressBook.inner.iter().collect();
        sorted.sort_by_key(|p| p.0);

        let order = inputNum();
        let mut orderCheck = true;
        while orderCheck {
            match order {
                1 => {
                    println!("[List all - names sorted by alpha]");
                    println!("------------------------------------");
                    for i in sorted.iter() {
                        println!("{} from {}, {:?}", i.0.name, i.0.address, i.1);
                    }
                    println!("------------------------------------");
                    match inputNum() {
                        _ => {
                            inputAgain = false;
                            break
                        },
                    }
                },
                2 => {
                    let division = divLoops();
                    let mut tmp = Vec::new();
                    for i in sorted.iter() {
                        if i.1 == &division {
                            tmp.push((&i.0.name, &i.0.address));
                        }
                    }
                    println!("[{:?}]", &division);
                    println!("------------------------------------");
                    for i in tmp.iter() {
                        println!("{} from {}", i.0, i.1);
                    }
                    println!("------------------------------------");
                    match inputNum() {
                        _ => {
                            inputAgain = false;
                            break
                        },
                    }
                },
                0 => {
                    inputAgain = false;
                    break
                },
                _ => {
                    orderCheck = false;
                },
            }
        }
    }
}

pub fn getPerson(addressBook : &AddressBook) {
    println!("3. Search a personal information.");
    let mut dataInputAgain = true;
    while dataInputAgain {
        println!("0. If you want to back to the main, input 0");
        println!("$. or input name you want to check");
        let name = inputString();

        match name.as_str() {
            "0" => dataInputAgain = false,
            _ => {
                let mut sorted: Vec<_> = addressBook.inner.iter().collect();
                sorted.sort_by_key(|p| p.0);
                println!("------------------------------------");
                for person in sorted.iter() {
                    // Search contains in String, person.0으로 빼는 순간 참조해서 가져옴
                    if person.0.name.to_lowercase().contains(&name.to_lowercase()) {
                        println!("{} from {}, {:?}", person.0.name, person.0.address, person.1);
                    }
                }
                println!("------------------------------------");
            },
        }
    }
}

pub fn rmPerson(addressBook: &mut AddressBook) {
    println!("4. Delete a personal data.");
    let mut toDel = Vec::new();
    let mut checkDivAgain = true;
    while checkDivAgain {
        let div = divLoops();
        let mut tmp = Vec::new();
        for p in addressBook.inner.iter() {
            // p.1로 빼는 순간 참조해서 가져옴
            if p.1 == &div {
                tmp.push(p);
            }
        }
        let mut nameCheckAgain = true;
        while nameCheckAgain {
            println!("------------------------------------");
            for tm in tmp.iter() {
                println!("name: {}, address: {}, {:?}", tm.0.name, tm.0.address, tm.1);
            }
            println!("------------------------------------");
            println!("0. if you want to back to the main, input 0");
            println!("1. or you want to search other departments, input 1 to back to the previous.");
            println!("$. or there is a data you want to delete, input the correct name");
            let name = inputString();
            if name.as_str() == "0" {
                checkDivAgain = false;
                nameCheckAgain = false;
                // break
            } else if name.as_str() == "1" {
                nameCheckAgain = false;
                // break
            }
            let mut targets = Vec::new();
            for p in tmp.iter() {
                if p.0.name == name {
                    targets.push(p);
                }
            }
            if targets.len() == 0 {
                println!("The name is not exist in this dep.");
                continue
            }
            let mut addCheckAgain = true;
            while addCheckAgain {
                println!("------------------------------------");
                for t in targets.iter() {
                    println!("name: {}, address: {}, {:?}", t.0.name, t.0.address, t.1);
                }
                println!("------------------------------------");
                println!("0. If you don't want to make changes here, input 0 to back to the main");
                println!("1. this is not the one you want to delete, input 1 to back to the previous");
                println!("$. this data is what you intend to delete, input the correct address.");
                let address = inputString();
                if address.as_str() == "0" {
                    checkDivAgain = false;
                    nameCheckAgain = false;
                    // addCheckAgain = false;
                    break
                } else if address.as_str() == "1" {
                    println!("back to the previous");
                    // addCheckAgain = false;
                    break
                }
                for t in targets.iter() {
                    if address == t.0.address {
                        let mut finCheckAgain = true;
                        while finCheckAgain {
                            println!("({} from {}, {:?})", t.0.name, t.0.address, t.1);
                            println!("Are you sure to delete? Y/N");
                            match inputString().to_lowercase().as_str() {
                                "y" | "yes" => {
                                    toDel.push(t.0.clone());
                                    println!("({} from {}, {:?})", t.0.name, t.0.address, t.1);
                                    checkDivAgain = false;
                                    nameCheckAgain = false;
                                    addCheckAgain = false;
                                    finCheckAgain = false;
                                    // break
                                },
                                "n" | "no" => {
                                    println!("Back to previous");
                                    finCheckAgain = false;
                                    // break
                                },
                                _ => continue,
                            }
                        }
                    }
                }
            }
        }
    }
    if toDel.is_empty() == false {
        addressBook.inner.remove(&toDel[0]).expect("DB update failure");
        println!("Memory updated");
    }
}

pub fn updateAddress(addressBook: &mut AddressBook) {
    println!("5. Update a personal data.");
    println!(
        "*If you want to update department who belong to, \
        go back to the main and access to the '1. Add a new personal data.' \
        and overwrite previous data.");

    let mut toDel = Vec::new();
    let mut toPush = Vec::new();
    let mut checkDivAgain = true;
    while checkDivAgain {
        let div = divLoops();
        let mut tmp = Vec::new();
        for p in addressBook.inner.iter() {
            // p.1로 빼는 순간 참조해서 가져옴
            if p.1 == &div {
                tmp.push(p);
            }
        }
        let mut nameCheckAgain = true;
        while nameCheckAgain {
            println!("------------------------------------");
            for tm in tmp.iter() {
                println!("name: {}, address: {}, {:?}", tm.0.name, tm.0.address, tm.1);
            }
            println!("------------------------------------");
            println!("0. if you want to back to the main, input 0");
            println!("1. or you want to search other departments, input 1 to back to the previous.");
            println!("$. or there is a data you want to update, input the correct name");
            let name = inputString();
            if name.as_str() == "0" {
                checkDivAgain = false;
                nameCheckAgain = false;
                // break
            } else if name.as_str() == "1" {
                nameCheckAgain = false;
                // break
            }
            let mut targets = Vec::new();
            for p in tmp.iter() {
                if p.0.name == name {
                    targets.push(p);
                }
            }
            if targets.len() == 0 {
                println!("The name is not exist in this dep.");
                continue
            }
            let mut addCheckAgain = true;
            while addCheckAgain {
                println!("------------------------------------");
                for t in targets.iter() {
                    println!("name: {}, address: {}, {:?}", t.0.name, t.0.address, t.1);
                }
                println!("------------------------------------");
                println!("0. If you don't want to make changes here, input 0 to back to the main");
                println!("1. this is not the one you want to delete, input 1 to back to the previous");
                println!("$. this data is what you intend to delete, input the correct address.");
                let address = inputString();
                if address.as_str() == "0" {
                    checkDivAgain = false;
                    nameCheckAgain = false;
                    // addCheckAgain = false;
                    break
                } else if address.as_str() == "1" {
                    println!("back to the previous");
                    // addCheckAgain = false;
                    break
                }
                for t in targets.iter() {
                    if address == t.0.address {
                        println!("Input new address");
                        let newAddress = inputString();
                        let mut finCheckAgain = true;
                        while finCheckAgain {
                            println!("({} from {}, {:?}) -> ({} from {}, {:?})"
                                     , t.0.name, t.0.address, t.1, t.0.name, newAddress, t.1);
                            println!("Are you sure to change? Y/N");
                            let newName = inputString();
                            match newName.to_lowercase().as_str() {
                                "y" | "yes" => {
                                    let newPerson = Person::new(&newName, &newAddress);
                                    toDel.push(t.0.clone());
                                    toPush.push((newPerson, t.1.clone()));
                                    println!("({} from {}, {:?})", t.0.name, t.0.address, t.1);
                                    checkDivAgain = false;
                                    nameCheckAgain = false;
                                    addCheckAgain = false;
                                    finCheckAgain = false;
                                    // break
                                },
                                "n" | "no" => {
                                    println!("Back to previous");
                                    finCheckAgain = false;
                                    // break
                                },
                                _ => continue,
                            }
                        }
                    }
                }
            }
        }
    }
    if toDel.is_empty() == false && toPush.is_empty() == false {
        addressBook.inner.remove(&toDel[0]).expect("DB update failure");
        addressBook.inner.insert(toPush[0].0.clone(), toPush[0].1.clone());
        // addressBook.delPerson(todel[0].clone());
        // addressBook.db.insert(toUpdate[0].0.clone(), toUpdate[0].1.clone());
        println!("Memory updated");
    }
}
