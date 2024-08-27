#![allow(dead_code)]

extern crate lazy_static;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, vec};
use strum_macros::EnumIter;

use super::upgrader::Upgrader;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Tags {
    Fire(f32),
    // time in seconds and multiplier after those seconds.
    Time(u8, f32),
    Acid(u8),
    Wet,
    Air,
    Nebula,
    Ice,
    Shield,
    Aired,
    Putrid,
    Magnetic,
    Fueled,
    Polished,
    Perfumed,
    Glitch,
    Vulnerable,
}
impl Tags {
    pub fn get_immunity(&self) -> Option<Immunities> {
        match self {
            Tags::Fire(_) => Some(Immunities::Fire),
            Tags::Acid(_) => Some(Immunities::Acid),
            _ => None,
        }
    }
    pub fn get_vulnerability(&self) -> Option<Vulnerabilities> {
        match self {
            Tags::Fire(_) => Some(Vulnerabilities::Fire),
            Tags::Polished => Some(Vulnerabilities::Polished),
            Tags::Wet => Some(Vulnerabilities::Wet),
            Tags::Putrid => Some(Vulnerabilities::Putrid),
            Tags::Fueled => Some(Vulnerabilities::Fueled),
            Tags::Acid(_) => Some(Vulnerabilities::Acid),
            Tags::Magnetic => Some(Vulnerabilities::Magnetic),
            Tags::Air => Some(Vulnerabilities::Air),
            Tags::Time(_, _) => Some(Vulnerabilities::Time),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Immunities {
    Fire,
    Acid,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Vulnerabilities {
    Fire,
    Polished,
    Wet,
    Putrid,
    Fueled,
    Acid,
    Magnetic,
    Air,
    Time,
    Aired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Multipliers {
    Fire(f32),
    Polished(f32),
    Wet(f32),
    Putrid(f32),
    Fueled(f32),
    Acid(f32),
    Magnetic(f32),
    Aired(f32),
    Time(f32),
    Perfumed(f32),
    Glitch(f32),
    Vulnerable(f32),
}
impl Multipliers {
    pub fn get_tag(&self) -> Tags {
        match self {
            Multipliers::Fire(_) => Tags::Fire(1.0),
            Multipliers::Acid(_) => Tags::Acid(1),
            Multipliers::Wet(_) => Tags::Wet,
            Multipliers::Putrid(_) => Tags::Putrid,
            Multipliers::Fueled(_) => Tags::Fueled,
            Multipliers::Magnetic(_) => Tags::Magnetic,
            Multipliers::Aired(_) => Tags::Air,
            Multipliers::Time(_) => Tags::Time(1, 1.0),
            Multipliers::Polished(_) => Tags::Polished,
            Multipliers::Perfumed(_) => Tags::Perfumed,
            Multipliers::Glitch(_) => Tags::Glitch,
            Multipliers::Vulnerable(_) => Tags::Vulnerable,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Hash, Eq, Serialize, Deserialize)]
pub enum Modifiers {
    Standard,
    Overclocked,
    Golden,
    Negative,
    OverclockedGolden,
    OverclockedNegative,
    NegativeGolden,
    OverclockedNegativeGolden,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MineTypes {
    // adds tag
    Tag(Tags, u32),

    // adds multiplier
    Multiplier(Multipliers),

    // adds immunity
    Immunity(Immunities),

    // adds vulnerability
    Vulnerability(Vulnerabilities),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgraderTypes {
    // adds Wet 1x if Fire 2x if no Fire
    AddsIfThen(Tags, u8, Tags, u8),

    // adds Ice if Wet
    AddsIf(Tags, Tags),

    //Adds Wet If not on Fire
    AddsIfNot(Tags, Tags),

    //replaces Fire with Wet
    Replaces(Tags, Tags),

    // extra 1.2x for each fire tag
    ExtraForEach(f32, Tags),

    //extra upgrade logarithmically based on ore value
    ExtraLogarithmic,

    // Multiplies value by 10x if glitch
    MultiplyIf(f32, Tags),

    // Multiplies one multiplier if in group
    MultiplyIfGrouped(f32, Vec<Tags>),

    //Extra 3x upgrade after 4.0s
    Overtime(f32, f32),

    //Adds Tag
    Adds(Tags, u32),

    //Removes Tag
    Removes(Tags),

    //Adds Immunity
    AddsImmunity(Immunities),

    //Adds Vulnerability
    AddsVulnerability(Vulnerabilities),

    //Removes Random Vulnerability
    RemovesRandomVulnerability,

    //Percentage chance to destroy ores
    DestroysPercent(f64),

    //Percentage to add tag then amount of tags.
    AddsPercent(f64, Tags, u32),

    //Percentage chance to multiply ores
    MultipliesPercent(f64, f64),

    //destroys ore if tag
    Destroys(Tags),

    //destroys ore if vulnerability
    DestroysVulnerability(Vulnerabilities),

    //destroys ore if not tag
    DestroysIfNotPercent(f64, Tags),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FurnaceTypes {
    // Refuses wet ores
    Refuses(Tags),

    // Extra 1.35x if Fueled
    MultiplyIf(f32, Tags),

    // Extra Multiplier for every Tag: 0.1x
    ExtraMultiplierEvery(f32),

    //Multiplies ores based on how many wet tags they have, if there are no wet tags then it will be processed at 1
    MultipliesByTag(Tags, f32),

    // +/- 0.2x value for each Glitch (tag) (up to 6)

    // +0.5x for each tag
    AddForEach(f32, Tags),
}

#[derive(Debug, Deserialize, EnumIter)]
pub enum Upgraders {
    OreSoaker,
    OrePolisher,
    FlameBlaster,
    HydrogenTank,
    SteamEngine,
    Magnetron,
    AcidPools,
    Hourglass,
    Microscope,
    Macroscope,
    BubblegumStockpile,
    ExtremeHeat,
    BillyFishtank,
    PerfectLawn,
    ColdSnap,
    EntityZero,
    ForbiddenBreeze,
    CausticLab,
    SunstonePylon,
    WindTunnel,
    GatesOfCalamity,
    AuroralTundra,
    DataEncryption,
    OreHacker,
    HeavenlyRuins,
    SteampunkOverdrive,
    CreatorsPlayground,
    PizzaParadise,
    StormSurge,
    SlimeRefinery,
    ParadisiacFlower,
    MetropunkClockwork,
    MoonlightTemple,
    AbyssalChains,
    DraconicNecromancy,
    BloodstruckRitual,
    CursedSiege,
    ElectricalAnnihilation,
    SkeletalWarlock,
    DysonSphere,
    ForcefulBlizzard,
    SimulationError,
    ResonanceSynthesia,
}
impl Upgraders {
    pub fn get_string(&self) -> String {
        match self {
            Upgraders::OreSoaker => "ore_soaker".to_string(),
            Upgraders::OrePolisher => "ore_polisher".to_string(),
            Upgraders::FlameBlaster => "flame_blaster".to_string(),
            Upgraders::HydrogenTank => "hydrogen_tank".to_string(),
            Upgraders::SteamEngine => "steam_engine".to_string(),
            Upgraders::Magnetron => "magnetron".to_string(),
            Upgraders::AcidPools => "acid_pools".to_string(),
            Upgraders::Hourglass => "hourglass".to_string(),
            Upgraders::Microscope => "microscope".to_string(),
            Upgraders::Macroscope => "macroscope".to_string(),
            Upgraders::BubblegumStockpile => "bubblegum_stockpile".to_string(),
            Upgraders::ExtremeHeat => "extreme_heat".to_string(),
            Upgraders::BillyFishtank => "billy_fishtank".to_string(),
            Upgraders::PerfectLawn => "perfect_lawn".to_string(),
            Upgraders::ColdSnap => "cold_snap".to_string(),
            Upgraders::EntityZero => "entity_zero".to_string(),
            Upgraders::ForbiddenBreeze => "forbidden_breeze".to_string(),
            Upgraders::CausticLab => "caustic_lab".to_string(),
            Upgraders::SunstonePylon => "sunstone_pylon".to_string(),
            Upgraders::WindTunnel => "wind_tunnel".to_string(),
            Upgraders::GatesOfCalamity => "gates_of_calamity".to_string(),
            Upgraders::AuroralTundra => "auroral_tundra".to_string(),
            Upgraders::DataEncryption => "data_encryption".to_string(),
            Upgraders::OreHacker => "ore_hacker".to_string(),
            Upgraders::HeavenlyRuins => "heavenly_ruins".to_string(),
            Upgraders::SteampunkOverdrive => "steampunk_overdrive".to_string(),
            Upgraders::CreatorsPlayground => "creators_playground".to_string(),
            Upgraders::PizzaParadise => "pizza_paradise".to_string(),
            Upgraders::StormSurge => "storm_surge".to_string(),
            Upgraders::SlimeRefinery => "slime_refinery".to_string(),
            Upgraders::ParadisiacFlower => "paradisiac_flower".to_string(),
            Upgraders::MetropunkClockwork => "metropunk_clockwork".to_string(),
            Upgraders::MoonlightTemple => "moonlight_temple".to_string(),
            Upgraders::AbyssalChains => "abyssal_chains".to_string(),
            Upgraders::DraconicNecromancy => "draconic_necromancy".to_string(),
            Upgraders::BloodstruckRitual => "bloodstruck_ritual".to_string(),
            Upgraders::CursedSiege => "cursed_siege".to_string(),
            Upgraders::ElectricalAnnihilation => "electrical_annihilation".to_string(),
            Upgraders::SkeletalWarlock => "skeletal_warlock".to_string(),
            Upgraders::DysonSphere => "dyson_sphere".to_string(),
            Upgraders::ForcefulBlizzard => "forceful_blizzard".to_string(),
            Upgraders::SimulationError => "simulation_error".to_string(),
            Upgraders::ResonanceSynthesia => "resonance_synthesia".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, EnumIter)]
pub enum Mines {
    IronMine,
    CopperMine,
    SilverMine,
    HeatedDropper,
    SurgeDropper,
    Contamination,
    NaturesEmbrace,
    WindTurbine,
    ArcaneRune,
    RadiantStar,
    CelestialLight,
    TheIcicle,
    CrystalMist,
    StellarOccultation,
    RoyalCastlekeep,
    DigitalWorld,
    CorrosiveSentinel,
    FloralObliterator,
    TimekeepersClocktower,
    ShadowVeil,
    GuttationDripper,
    GalvanicSurge,
    UtopicFlower,
    GlitchkingsDomain,
}
impl Mines {
    pub fn get_string(&self) -> String {
        match self {
            Mines::IronMine => "iron_mine".to_string(),
            Mines::CopperMine => "copper_mine".to_string(),
            Mines::SilverMine => "silver_mine".to_string(),
            Mines::HeatedDropper => "heated_dropper".to_string(),
            Mines::SurgeDropper => "surge_dropper".to_string(),
            Mines::Contamination => "contamination".to_string(),
            Mines::NaturesEmbrace => "natures_embrace".to_string(),
            Mines::WindTurbine => "wind_turbine".to_string(),
            Mines::ArcaneRune => "arcane_rune".to_string(),
            Mines::RadiantStar => "radiant_star".to_string(),
            Mines::CelestialLight => "celestial_light".to_string(),
            Mines::TheIcicle => "the_icicle".to_string(),
            Mines::CrystalMist => "crystal_mist".to_string(),
            Mines::StellarOccultation => "stellar_occultation".to_string(),
            Mines::RoyalCastlekeep => "royal_castlekeep".to_string(),
            Mines::DigitalWorld => "digital_world".to_string(),
            Mines::CorrosiveSentinel => "corrosive_sentinel".to_string(),
            Mines::FloralObliterator => "floral_obliterator".to_string(),
            Mines::TimekeepersClocktower => "timekeepers_clocktower".to_string(),
            Mines::ShadowVeil => "shadow_veil".to_string(),
            Mines::GuttationDripper => "guttation_dripper".to_string(),
            Mines::GalvanicSurge => "galvanic_surge".to_string(),
            Mines::UtopicFlower => "utopic_flower".to_string(),
            Mines::GlitchkingsDomain => "glitchkings_domain".to_string(),
        }
    }
}
lazy_static! {
    #[derive(Debug)]
    pub static ref MINE_RATES: HashMap<&'static Modifiers, f32> = {
        let mut m = HashMap::new();
        m.insert(&Modifiers::Golden, 3.5);
        m.insert(&Modifiers::Negative, 7.5);
        m.insert(&Modifiers::NegativeGolden, 26.25);
        m
    };
    pub static ref MINE_DROP_RATES: HashMap<&'static Modifiers, Vec<f32>> = {
        let mut m = HashMap::new();
        m.insert(&Modifiers::Overclocked, vec![2.125, 0.025]);
        m
    };
    pub static ref RATES_FROM_STANDARD: HashMap<&'static Modifiers, Vec<f32>> = {
        let mut m = HashMap::new();
        m.insert(&Modifiers::Overclocked, vec![2.14, 1.14]);
        m.insert(&Modifiers::Golden, vec![3.5, 2.5]);
        m.insert(&Modifiers::Negative, vec![7.5, 6.5]);
        m.insert(&Modifiers::OverclockedGolden, vec![7.49, 6.49]);
        m.insert(&Modifiers::OverclockedNegative, vec![16.05, 15.05]);
        m.insert(&Modifiers::NegativeGolden, vec![26.25, 25.25]);
        m.insert(&Modifiers::OverclockedNegativeGolden, vec![56.175, 55.175]);

        m
    };

    pub static ref RARITY_MULTIPLIERS: HashMap<&'static Modifiers, u64> = {
        let mut m = HashMap::new();
        m.insert(&Modifiers::Standard, 1);
        m.insert(&Modifiers::Overclocked, 10);
        m.insert(&Modifiers::Golden, 25);
        m.insert(&Modifiers::Negative, 250);
        m.insert(&Modifiers::OverclockedGolden, 250);
        m.insert(&Modifiers::OverclockedNegative, 2500);
        m.insert(&Modifiers::NegativeGolden, 6250);
        m.insert(&Modifiers::OverclockedNegativeGolden, 62500);
        m
    };
}
