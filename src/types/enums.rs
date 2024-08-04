#![allow(dead_code)]

extern crate lazy_static;

use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
    Time(f32)
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
}


