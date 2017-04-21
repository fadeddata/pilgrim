use std::collections::HashMap;

pub trait Skills {
    fn skills(&self) -> Vec<Skill>;
}

fn education_skills() -> Vec<Skill> {
    use skills::Skill::*;
    vec![
        Admin,
        Advocate,
        Animals,
        Carousing,
        Comms,
        Computer,
        Electronics,
        Engineering,
        LifeSciences,
        Linguistics,
        Mechanics,
        Medicine,
        PhysicalSciences,
        SocialSciences,
        SpaceSciences,
    ]
}

fn cascade_from_skill(skill: Skill) -> Skill {
    use skills::Skill::*;
    match skill {
        Farming | Riding | Survival | VeterinaryMedicine => Animals,
        LifeSciences | PhysicalSciences | SocialSciences | SpaceSciences => Sciences,
        Archery | EnergyPistol | EnergyRifle | Shotgun | SlugPistol | SlugRifle => GunCombat,
        BayWeapons | HeavyWeapons | Screens | SpinalMounts | TurretWeapons => Gunnery,
        BludgeoningWeapons | NaturalWeapons | PiercingWeapons | SlashingWeapons => MeleeCombat,
        Aircraft | Mole | Watercraft | WheeledVehicle => Vehicle,
        GravVehicle | RotorAircraft | WingedAircraft => Aircraft,
        Motorboats | OceanShips | SailingShips | Submarine => Watercraft,
        _ => skill,
    }
}

fn skills_from_cascade(skill: Skill) -> Vec<Skill> {
    use skills::Skill::*;

    match skill {
        Animals => vec![Farming, Riding, Survival, VeterinaryMedicine],
        Sciences => {
            vec![
                LifeSciences,
                PhysicalSciences,
                SocialSciences,
                SpaceSciences,
            ]
        }
        GunCombat => {
            vec![
                Archery,
                EnergyPistol,
                EnergyRifle,
                Shotgun,
                SlugPistol,
                SlugRifle,
            ]
        }
        Gunnery => {
            vec![
                BayWeapons,
                HeavyWeapons,
                Screens,
                SpinalMounts,
                TurretWeapons,
            ]
        }
        MeleeCombat => {
            vec![
                BludgeoningWeapons,
                NaturalWeapons,
                PiercingWeapons,
                SlashingWeapons,
            ]
        }
        Vehicle => vec![Aircraft, Mole, Watercraft, WheeledVehicle],
        Aircraft => vec![GravVehicle, RotorAircraft, WingedAircraft],
        Watercraft => vec![Motorboats, OceanShips, SailingShips, Submarine],
        _ => Vec::new(),
    }
}

struct CharacterSkills {
    pub entity_id: u32,
    pub skills: HashMap<Skill, u8>,
}

pub enum Skill {
    Admin,
    Advocate,
    Animals,
    Farming,
    Riding,
    Survival,
    VeterinaryMedicine,
    Athletics,
    BattleDress,
    Bribery,
    Broker,
    Carousing,
    Comms,
    Computer,
    Demolitions,
    Electronics,
    Engineering,
    Gambling,
    Gravitics,
    JackOfAllTrades,
    Leadership,
    Linguistics,
    Liaison,
    Mechanics,
    Medicine,
    Navigation,
    Piloting,
    Recon,
    Sciences,
    LifeSciences,
    PhysicalSciences,
    SocialSciences,
    SpaceSciences,
    Steward,
    Streetwise,
    Tactics,
    ZeroG,
    GunCombat,
    Archery,
    EnergyPistol,
    EnergyRifle,
    Shotgun,
    SlugPistol,
    SlugRifle,
    Gunnery,
    BayWeapons,
    HeavyWeapons,
    Screens,
    SpinalMounts,
    TurretWeapons,
    MeleeCombat,
    BludgeoningWeapons,
    NaturalWeapons,
    PiercingWeapons,
    SlashingWeapons,
    Vehicle,
    Aircraft,
    GravVehicle,
    RotorAircraft,
    WingedAircraft,
    Mole,
    TrackedVehicle,
    Watercraft,
    Motorboats,
    OceanShips,
    SailingShips,
    Submarine,
    WheeledVehicle,
}
