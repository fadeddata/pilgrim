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
pub struct Character {
    pub characteristics: Characteristics,
    pub homeworld: Homeworld,
    pub skills: Vec<Skill>,
    pub terms: Vec<Term>,
    pub age: u8,
    pub material_benefits: Vec<MaterialBenefit>,
}
