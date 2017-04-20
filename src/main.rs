extern crate rusqlite;
extern crate time;
extern crate rand;

mod characteristics;
mod dice;
mod homeworlds;
mod skills;
mod pseudo_hex;

//use time::Timespec;
use rusqlite::Connection;
use characteristics::*;
use dice::Die;
use std::io;
use homeworlds::*;

fn main() {
    let ch = roll_characteristics();
    let homeworld = choose_homeworld();

    println!("Characteristics: {:?}", ch);
    println!("Characteristics UPP: {}", ch);
    println!("Homeworld : {:?}", homeworld);
}

fn choose_homeworld() -> Homeworld {
    use Descriptor::*;
    use TradeCode::*;

    let mut descriptor_pick: usize = 0;
    let descriptors = homeworlds::descriptors();

    while descriptor_pick == 0 {
        println!("Pick a descriptor. (1-{}).", descriptors.len());

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        descriptor_pick = match input.trim().parse() {
            Ok(num) if num > 0 && num <= (descriptors.len() as usize) => num,

            Ok(_) => {
                println!("Please pick a number between 1 and {}.", descriptors.len());
                continue;
            }

            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };
    }

    descriptor_pick = descriptor_pick - 1;
    let descriptor = descriptors[descriptor_pick].clone();

    Homeworld {
        entity_id: 0,
        descriptor: descriptor,
        trade_code: Agricultural,
    }
}

fn roll_characteristics() -> Characteristics {
    use Characteristic::*;

    let mut ch_rolls = Vec::with_capacity(6);
    let mut d6 = Die::new(6, &[55]);
    for _ in 0..6 {
        ch_rolls.push(d6.add_roll(2));
    }

    let chars = vec![Strength,
                     Dexterity,
                     Endurance,
                     Intelligence,
                     Education,
                     Social];

    let mut strength = 0;
    let mut dexterity = 0;
    let mut endurance = 0;
    let mut intelligence = 0;
    let mut education = 0;
    let mut social = 0;

    let mut done: bool;

    for char in chars {
        let mut pick: usize = 0;
        done = false;

        while !done {
            println!("Rolls {:?}", ch_rolls);
            println!("Pick a roll for {} (1-{}).", char, ch_rolls.len());

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            pick = match input.trim().parse() {
                Ok(num) if num > 0 && num <= (ch_rolls.len() as usize) => num,

                Ok(_) => {
                    println!("Please pick a number between 1 and {}.", ch_rolls.len());
                    done = false;
                    continue;
                }

                Err(_) => {
                    println!("Please type a number.");
                    done = false;
                    continue;
                }
            };

            done = true;
        }
        pick = pick - 1;
        match char {
            Strength => strength = ch_rolls.swap_remove(pick),
            Dexterity => dexterity = ch_rolls.swap_remove(pick),
            Endurance => endurance = ch_rolls.swap_remove(pick),
            Intelligence => intelligence = ch_rolls.swap_remove(pick),
            Education => education = ch_rolls.swap_remove(pick),
            Social => social = ch_rolls.swap_remove(pick),
        }
    }

    Characteristics {
        entity_id: 0,
        strength: strength,
        dexterity: dexterity,
        endurance: endurance,
        intelligence: intelligence,
        education: education,
        social: social,
    }
}