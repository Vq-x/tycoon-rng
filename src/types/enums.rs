#![allow(dead_code)]

extern crate lazy_static;

use lazy_static::lazy_static;
use std::{collections::HashMap, vec};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Tags {
    Fire(u8),
    // time in seconds and multiplier after those seconds.
    Time(u8, u32),
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

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Immunities {
    Fire,
    Acid,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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
}

#[derive(Debug, Clone)]
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
            Multipliers::Fire(_) => Tags::Fire(1),
            Multipliers::Acid(_) => Tags::Acid(1),
            Multipliers::Wet(_) => Tags::Wet,
            Multipliers::Putrid(_) => Tags::Putrid,
            Multipliers::Fueled(_) => Tags::Fueled,
            Multipliers::Magnetic(_) => Tags::Magnetic,
            Multipliers::Aired(_) => Tags::Air,
            Multipliers::Time(_) => Tags::Time(1, 0),
            Multipliers::Polished(_) => Tags::Polished,
            Multipliers::Perfumed(_) => Tags::Perfumed,
            Multipliers::Glitch(_) => Tags::Glitch,
            Multipliers::Vulnerable(_) => Tags::Vulnerable,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
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

#[derive(Debug, Clone)]
pub enum UpgraderTypes {
    // adds Wet 1x if Fire 2x if None
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

    //destroys ore if tag
    Destroys(Tags),
}

#[derive(Debug, Clone)]
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
