use characteristics::Characteristic;
use skills::Skill;

pub struct Career {
    name: String,
    qualifications: (Characteristic, u8),
    survival: (Characteristic, u8),
    commission: Option<(Characteristic, u8)>,
    advancement: Option<(Characteristic, u8)>,
    reenlistment: u8,
    ranks: Vec<(Option<String>, Option<Skill>)>,
    material_benefits: Vec<MaterialBenefit>,
    cash_benefits: Vec<u32>,
    personal_development: Vec<Training>,
    service_skills: Vec<Training>,
    specialist: Vec<Training>,
    advanced_education: Vec<Training>,
}

pub enum MaterialBenefit {
    MBC(Characteristic),
    CourierVessel,
    ExplorersSociety,
    HighPassage,
    LowPassage,
    MidPassage,
    ResearchVessel,
    ShipShares,
    Weapon,
}

pub enum Training {
    TC(Characteristic),
    TS(Skill),
}

pub fn careers() -> Vec<Career> {
    use self::MaterialBenefit::*;
    use self::Training::*;
    use characteristics::Characteristic::*;
    use skills::Skill::*;

    vec![
        Career {
            name: String::from("Athlete"),
            qualifications: (Endurance, 8),
            survival: (Dexterity, 5),
            commission: None,
            advancement: None,
            reenlistment: 6,
            ranks: vec![(None, Some(Athletics))],
            material_benefits: vec![
                LowPassage,
                MBC(Intelligence),
                Weapon,
                HighPassage,
                ExplorersSociety,
                HighPassage,
            ],
            cash_benefits: vec![2000, 10000, 20000, 20000, 50000, 100000, 100000],
            personal_development: vec![
                TC(Dexterity),
                TC(Intelligence),
                TC(Education),
                TC(Social),
                TS(Carousing),
                TS(MeleeCombat),
            ],
            service_skills: vec![
                TS(Athletics),
                TS(Admin),
                TS(Carousing),
                TS(Computer),
                TS(Gambling),
                TS(Vehicle),
            ],
            specialist: vec![
                TS(ZeroG),
                TS(Athletics),
                TS(Athletics),
                TS(Computer),
                TS(Leadership),
                TS(Gambling),
            ],
            advanced_education: vec![
                TS(Advocate),
                TS(Computer),
                TS(Liaison),
                TS(Linguistics),
                TS(Medicine),
                TS(Sciences),
            ],
        },
        Career {
            name: String::from("Aerospace Defense"),
            qualifications: (Endurance, 5),
            survival: (Dexterity, 5),
            commission: Some((Education, 6)),
            advancement: Some((Education, 7)),
            reenlistment: 5,
            ranks: vec![
                (Some(String::from("Airman")), Some(Aircraft)),
                (Some(String::from("Flight Officer")), None),
                (Some(String::from("Flight Lieutenant")), None),
                (Some(String::from("Squadron Leader")), Some(Leadership)),
                (Some(String::from("Wing Commander")), None),
                (Some(String::from("Group Captain")), None),
                (Some(String::from("Air Commodore")), None),
            ],
            material_benefits: vec![
                LowPassage,
                MBC(Education),
                Weapon,
                MidPassage,
                Weapon,
                HighPassage,
                MBC(Social),
            ],
            cash_benefits: vec![1000, 5000, 10000, 10000, 20000, 50000, 50000],
            personal_development: vec![
                TC(Strength),
                TC(Dexterity),
                TC(Endurance),
                TS(Athletics),
                TS(MeleeCombat),
                TS(Vehicle),
            ],
            service_skills: vec![
                TS(Electronics),
                TS(GunCombat),
                TS(Gunnery),
                TS(MeleeCombat),
                TS(Survival),
                TS(Aircraft),
            ],
            specialist: vec![
                TS(Comms),
                TS(Gravitics),
                TS(GunCombat),
                TS(Gunnery),
                TS(Recon),
                TS(Piloting),
            ],
            advanced_education: vec![
                TS(Advocate),
                TS(Computer),
                TS(JackOfAllTrades),
                TS(Medicine),
                TS(Leadership),
                TS(Tactics),
            ],
        },
        Career {
            name: String::from("Agent"),
            qualifications: (Social, 6),
            survival: (Intelligence, 6),
            commission: Some((Education, 7)),
            advancement: Some((Education, 6)),
            reenlistment: 6,
            ranks: vec![
                (Some(String::from("Agent")), Some(Streetwise)),
                (Some(String::from("Special Agent")), None),
                (Some(String::from("Sp Agent in Charge")), None),
                (Some(String::from("Unit Chief")), Some(Leadership)),
                (Some(String::from("Section Chief")), Some(Admin)),
                (Some(String::from("Assistant Director")), None),
                (Some(String::from("Director")), None),
            ],
            material_benefits: vec![
                LowPassage,
                MBC(Intelligence),
                Weapon,
                MidPassage,
                MBC(Social),
                HighPassage,
                ExplorersSociety,
            ],
            cash_benefits: vec![1000, 5000, 10000, 10000, 20000, 50000, 50000],
            personal_development: vec![
                TC(Dexterity),
                TC(Endurance),
                TC(Intelligence),
                TC(Education),
                TS(Athletics),
                TS(Carousing),
            ],
            service_skills: vec![
                TS(Admin),
                TS(Computer),
                TS(Streetwise),
                TS(Bribery),
                TS(Leadership),
                TS(Vehicle),
            ],
            specialist: vec![
                TS(GunCombat),
                TS(MeleeCombat),
                TS(Bribery),
                TS(Leadership),
                TS(Recon),
                TS(Survival),
            ],
            advanced_education: vec![
                TS(Advocate),
                TS(Computer),
                TS(Liaison),
                TS(Linguistics),
                TS(Medicine),
                TS(Leadership),
            ],
        },
        Career {
            name: String::from("Barbarian"),
            qualifications: (Endurance, 5),
            survival: (Strength, 6),
            commission: None,
            advancement: None,
            reenlistment: 5,
            ranks: vec![(None, Some(MeleeCombat))],
            material_benefits: vec![
                LowPassage,
                MBC(Intelligence),
                Weapon,
                Weapon,
                MBC(Endurance),
                MidPassage,
            ],
            cash_benefits: vec![0, 1000, 2000, 5000, 5000, 10000, 10000],
            personal_development: vec![
                TC(Strength),
                TC(Dexterity),
                TC(Endurance),
                TC(Intelligence),
                TS(Athletics),
                TS(GunCombat),
            ],
            service_skills: vec![
                TS(Mechanics),
                TS(GunCombat),
                TS(MeleeCombat),
                TS(Recon),
                TS(Survival),
                TS(Animals),
            ],
            specialist: vec![
                TS(GunCombat),
                TS(JackOfAllTrades),
                TS(MeleeCombat),
                TS(Recon),
                TS(Animals),
                TS(Tactics),
            ],
            advanced_education: vec![
                TS(Advocate),
                TS(Linguistics),
                TS(Medicine),
                TS(Leadership),
                TS(Tactics),
                TS(Broker),
            ],
        },
        Career {
            name: String::from("Belter"),
            qualifications: (Intelligence, 4),
            survival: (Dexterity, 7),
            commission: None,
            advancement: None,
            reenlistment: 5,
            ranks: vec![(None, Some(ZeroG))],
            material_benefits: vec![
                LowPassage,
                MBC(Intelligence),
                Weapon,
                MidPassage,
                ShipShares,
                HighPassage,
            ],
            cash_benefits: vec![1000, 5000, 5000, 5000, 10000, 20000, 50000],
            personal_development: vec![
                TC(Strength),
                TC(Dexterity),
                TC(Endurance),
                TS(ZeroG),
                TS(MeleeCombat),
                TS(Gambling),
            ],
            service_skills: vec![
                TS(Comms),
                TS(Demolitions),
                TS(GunCombat),
                TS(Gunnery),
                TS(Prospecting),
                TS(Piloting),
            ],
            specialist: vec![
                TS(ZeroG),
                TS(Computer),
                TS(Electronics),
                TS(Prospecting),
                TS(Sciences),
                TS(Vehicle),
            ],
            advanced_education: vec![
                TS(Advocate),
                TS(Engineering),
                TS(Medicine),
                TS(Navigation),
                TS(Comms),
                TS(Tactics),
            ],
        },
        Career {
            name: String::from("Bureaucrat"),
            qualifications: (Social, 6),
            survival: (Education, 4),
            commission: Some((Social, 5)),
            advancement: Some((Intelligence, 8)),
            reenlistment: 5,
            ranks: vec![
                (Some(String::from("Assistant")), Some(Admin)),
                (Some(String::from("Clerk")), None),
                (Some(String::from("Supervisor")), None),
                (Some(String::from("Manager")), None),
                (Some(String::from("Chief")), None),
                (Some(String::from("Director")), None),
                (Some(String::from("Minister")), None),
            ],
            material_benefits: vec![
                LowPassage,
                MBC(Education),
                MBC(Intelligence),
                MidPassage,
                MidPassage,
                HighPassage,
                MBC(Social),
            ],
            cash_benefits: vec![1000, 5000, 10000, 10000, 20000, 50000, 50000],
            personal_development: vec![
                TC(Dexterity),
                TC(Endurance),
                TC(Intelligence),
                TC(Education),
                TS(Athletics),
                TS(Carousing),
            ],
            service_skills: vec![
                TS(Admin),
                TS(Computer),
                TS(Carousing),
                TS(Bribery),
                TS(Leadership),
                TS(Vehicle),
            ],
            specialist: vec![
                TS(Admin),
                TS(Computer),
                TS(Perception),
                TS(Leadership),
                TS(Steward),
                TS(Vehicle),
            ],
            advanced_education: vec![
                TS(Advocate),
                TS(Computer),
                TS(Liaison),
                TS(Linguistics),
                TS(Medicine),
                TS(Admin),
            ],
        },
    ]
}
