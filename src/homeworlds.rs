use skills::*;
use skills::Skill::*;


#[derive(Debug)]
pub struct Homeworld {
    pub entity_id: u32,
    pub descriptor: Descriptor,
    pub trade_code: TradeCode,
}

impl Skills for Homeworld {
    fn skills(&self) -> Vec<Skill> {
        let mut descriptor_skills = self.descriptor.skills();
        let trade_code_skills = self.trade_code.skills();

        descriptor_skills.extend(trade_code_skills);

        descriptor_skills
    }
}

pub fn descriptors() -> Vec<Descriptor> {
    use Descriptor::*;

    vec![NoLaw, LowLaw, MediumLaw, HighLaw]
}

#[derive(Clone, Debug)]
pub enum Descriptor {
    NoLaw,
    LowLaw,
    MediumLaw,
    HighLaw,
}

impl Skills for Descriptor {
    fn skills(&self) -> Vec<Skill> {
        use Descriptor::*;

        match *self {
            NoLaw | LowLaw | MediumLaw => vec![GunCombat],
            HighLaw => vec![MeleeCombat],
        }
    }
}

#[derive(Clone, Debug)]
pub enum TradeCode {
    Agricultural,
    Asteroid,
    Desert,
    FluidOceans,
    Garden,
    HighTechnology,
    HighPopulation,
    IceCapped,
    Industrial,
    LowTechnology,
    Poor,
    Rich,
    WaterWorld,
    Vacuum,
}

pub fn trade_codes() -> Vec<TradeCode> {
    use TradeCode::*;
    vec![
        Agricultural,
        Asteroid,
        Desert,
        FluidOceans,
        Garden,
        HighTechnology,
        HighPopulation,
        IceCapped,
        Industrial,
        LowTechnology,
        Poor,
        Rich,
        WaterWorld,
        Vacuum,
    ]
}

impl TradeCode {
    fn skills(&self) -> Vec<Skill> {
        use TradeCode::*;

        match *self {
            Agricultural | Garden | Poor => vec![Animals],
            Asteroid | IceCapped | Vacuum => vec![ZeroG],
            Desert | LowTechnology => vec![Survival],
            FluidOceans => vec![Watercraft],
            HighTechnology => vec![Computer],
            HighPopulation => vec![Streetwise],
            Industrial => vec![Broker], 
            Rich => vec![Carousing],
            WaterWorld => vec![Watercraft],
        }
    }
}
