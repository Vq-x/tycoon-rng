#![allow(dead_code)]

extern crate lazy_static;

use lazy_static::lazy_static;
use std::{collections::HashMap, vec};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Tags {
    Fire(u8),
    Time(u8),
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
}
#[derive(Debug, Clone)]
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
    Air(f32),
    Time(f32),
    Perfumed(f32),
    Glitch(f32),
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

pub enum UpgraderTypes {
    // adds 1x if Fire 2x if None
    AddsIf(u8, Tags, u8),
    //Adds Wet If not on Fire
    AddsIfNot(Tags, Tags),
    //replaces Fire with Wet
    Replaces(Tags, Tags),
    // extra 1.2x for each fire tag
    ExtraForEach(f32, Tags),
    // Multiplies value by 10x if glitch
    MultiplyIf(f32, Tags),
    //Extra 3x upgrade after 4.0s
    Overtime(f32, f32),
    //Adds Tag
    Adds(Tags),
    //Removes Tag
    Removes(Tags),
    //Adds Immunity
    AddsImmunity(Tags),
    //Adds Vulnerability
    AddsVulnerability(Vulnerabilities),
    //destroys ore if tag
    Destroys(Tags),
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
