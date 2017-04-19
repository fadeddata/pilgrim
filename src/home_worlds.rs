use skills::Skill::*;
use skills::Skill;
use skills::Skills;

enum Descriptors {
    NoLaw,
    LowLaw,
    MediumLaw,
    HighLaw,
}

impl Skills for Descriptors {
    fn skills(&self) -> Vec<Skill> {
        use home_worlds::Descriptors::*;

        match *self {
            NoLaw | LowLaw | MediumLaw => vec![GunCombat],
            HighLaw => vec![MeleeCombat]
        }
    }
}

enum TradeCodes {
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

impl TradeCodes {
    fn skills(&self) -> Vec<Skill> {
        use home_worlds::TradeCodes::*;
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
