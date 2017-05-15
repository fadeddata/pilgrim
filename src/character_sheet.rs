use characteristics::Characteristics;
use homeworlds::Homeworld;
use skills::Skill;
use careers::Career;
use careers::MaterialBenefit;

#[derive(Debug)]
pub struct Term {
    pub career_name: &'static str,
    pub qualified: bool,
    pub survived: bool,
    pub commisioned: Option<bool>,
    pub advanced: Option<bool>,
    pub reenlisted: bool,
    pub rank: &'static str,
}

#[derive(Debug)]
pub struct CharacterSheet {
    pub characteristics: Characteristics,
    pub homeworld: Homeworld,
    pub skills: Vec<Skill>,
    pub terms: Vec<Term>,
    pub age: u8,
    pub material_benefits: Vec<MaterialBenefit>,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    Pick(u8),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum State {
    SelectStr,
    SelectDex { str: u8 },
    SelectEnd { str: u8, dex: u8 },
    SelectInt { str: u8, dex: u8, end: u8 },
    SelectEdu { str: u8, dex: u8, end: u8, int: u8 },
    SelectSoc { str: u8, dex: u8, end: u8, int: u8, edu: u8 },
    FinishedCharacteristics(Characteristics),
}

impl State {
    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn next(self, event: Event) -> State {
        match (self, event) {
            (State::SelectStr, Event::Pick(val)) => State::SelectDex { str: val },
            (State::SelectDex { str }, Event::Pick(val)) => State::SelectEnd { str: str, dex: val },
            (State::SelectEnd { str, dex }, Event::Pick(val)) => State::SelectInt { str: str, dex: dex, end: val },
            (State::SelectInt { str, dex, end }, Event::Pick(val)) => State::SelectEdu { str: str, dex: dex, end: end, int: val, },
            (State::SelectEdu { str, dex, end, int }, Event::Pick(val)) => State::SelectSoc { str: str, dex: dex, end: end, int: int, edu: val, },
            (State::SelectSoc { str, dex, end, int, edu, }, Event::Pick(val)) => {
                State::FinishedCharacteristics(Characteristics {
                                                   entity_id: 0,
                                                   strength: str,
                                                   dexterity: dex,
                                                   endurance: end,
                                                   intelligence: int,
                                                   education: edu,
                                                   social: val,
                                               })
            },
            (State::FinishedCharacteristics(characteristics), _) => State::FinishedCharacteristics(characteristics)
        }
    }

    pub fn characteristics(&self) -> Option<&Characteristics> {
        match *self {
            State::FinishedCharacteristics(ref characteristics) => Some(characteristics),
            _ => None,
        }
    }

    pub fn display_text(&self) -> &str {
        match *self {
            State::SelectStr => "select strength",
            State::SelectDex { .. } => "select dexterity",
            State::SelectEnd { .. } => "select endurance",
            State::SelectInt { .. } => "select intelligence",
            State::SelectEdu { .. } => "select education",
            State::SelectSoc { .. } => "select social",
            State::FinishedCharacteristics(..) => "done",
        }
    }
}
