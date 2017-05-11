extern crate rusqlite;
extern crate time;
extern crate rand;

mod characteristics;
mod dice;
mod homeworlds;
mod skills;
mod pseudo_hex;
mod careers;

//use time::Timespec;
use rusqlite::Connection;
use characteristics::*;
use dice::Die;
use std::io;
use homeworlds::*;
use skills::*;

fn main() {
    let ch = roll_characteristics();
    let homeworld = choose_homeworld();
    let background_skills = choose_background_skills(&homeworld, ch.education);
    let background_skill_names =
        background_skills.iter()
                         .map(|ref n| format!("{:?}", n))
                         .collect::<Vec<_>>();

    let careers_names = careers::careers()
        .iter()
        .map(|ref n| n.name.clone())
        .collect::<Vec<_>>();

    println!("{:?}", careers_names.join(", "));
    println!("Characteristics: {:?}", ch);
    println!("Characteristics UPP: {}", ch);
    println!("Homeworld: {:?}", homeworld);
    println!("Background skills: {:?}", background_skill_names.join(", "));
}

fn pick_a_thing<T>(things: &[T], header: &str) -> usize {
    let mut pick: usize = 0;

    while pick == 0 {
        println!("{}", header);
        println!("Please select. (1-{}).", things.len());

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        pick = match input.trim().parse() {
            Ok(num) if num > 0 && num <= (things.len() as usize) => num,

            Ok(_) => {
                println!("Please pick a number between 1 and {}.", things.len());
                continue;
            }

            Err(_) => {
                println!("Please type a number.");
                continue;
            }
        };
    }

    pick - 1
}

fn choose_background_skills(homeworld: &Homeworld, education: u8) -> Vec<Skill> {
    let skill_count: usize = (characteristic_modifier(education) + 3) as usize;
    let mut skills: Vec<Skill> = Vec::with_capacity(skill_count);
    let mut skill_choices = education_skills();
    let homeworld_skills = homeworld.skills();

    skill_choices.extend(homeworld_skills);

    for _ in 0..skill_count {
        let rolls_format = format!("Rolls {:?}", skill_choices);
        let pick = pick_a_thing(&skill_choices, rolls_format.as_str());
        skills.push(skill_choices.swap_remove(pick))
    }

    skills
}

fn choose_homeworld() -> Homeworld {
    let descriptors = homeworlds::descriptors();
    let descriptors_names = format!("Descriptors: {:?}", descriptors);
    let descriptor_pick = pick_a_thing(&descriptors, descriptors_names.as_str());
    let descriptor = descriptors[descriptor_pick].clone();

    let trade_codes = homeworlds::trade_codes();
    let trade_codes_name = format!("Trade codes: {:?}", trade_codes);
    let trade_code_pick = pick_a_thing(&trade_codes, trade_codes_name.as_str());
    let trade_code = trade_codes[trade_code_pick].clone();

    Homeworld {
        entity_id: 0,
        descriptor: descriptor,
        trade_code: trade_code,
    }
}

fn roll_characteristics() -> Characteristics {
    use Characteristic::*;

    let mut ch_rolls = Vec::with_capacity(6);
    let mut d6 = Die::new(6, &[55]);
    for _ in 0..6 {
        ch_rolls.push(d6.add_roll(2));
    }

    let chars = vec![
        Strength,
        Dexterity,
        Endurance,
        Intelligence,
        Education,
        Social,
    ];

    let mut strength = 0;
    let mut dexterity = 0;
    let mut endurance = 0;
    let mut intelligence = 0;
    let mut education = 0;
    let mut social = 0;

    for char in chars {
        let rolls_format = format!("Rolls {:?}", ch_rolls);
        let pick = pick_a_thing(&ch_rolls, rolls_format.as_str());

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
