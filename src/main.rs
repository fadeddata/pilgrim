extern crate rusqlite;
extern crate time;
extern crate rand;

mod characteristics;
mod dice;

//use time::Timespec;
use rusqlite::Connection;
use characteristics::*;
use dice::Die;
use std::io;


fn main() {
    let conn = Connection::open_in_memory().unwrap();
    Characteristics::create_tables(&conn);

    let ch = Characteristics {
        entity_id: 0,
        strength: 1,
        dexterity: 2,
        endurance: 3,
        intelligence: 4,
        education: 5,
        social: 6,
    };

    ch.insert(&conn);

    let ch_iter = Characteristics::get_all(&conn);

    for characteristics in ch_iter {
        println!("Found {:?}", characteristics);
    }

    roll_characteristics();
}

fn roll_characteristics() {
    let chars = vec![Values::Strength, Values::Dexterity];

    let mut ch_rolls = Vec::with_capacity(6);
    let mut d6 = Die::new(6, &[55]);
    for _ in 0..6 {
        ch_rolls.push(d6.add_roll(2));
    }

    println!("Rolls {:?}", ch_rolls);

    loop {
        println!("Pick a roll for strength (1-{}).", ch_rolls.len());
        let mut pick = String::new();
        io::stdin()
            .read_line(&mut pick)
            .expect("failed to read line");

        let pick: u8 = match pick.trim().parse() {
            Ok(num) if num > 0 && num < (ch_rolls.len() as u8) => num,
            Ok(_) => {
                println!("Please pick a number between 1 and {}.", ch_rolls.len());
                continue;
            }
            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };
    }
}