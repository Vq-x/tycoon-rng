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
#[derive(Debug, Clone, PartialEq)]
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


// lazy_static! {
//     static ref MINE_RATES: HashMap<&'static Modifiers, Vec<f32>> = {
//         let mut m = HashMap::new();
//         m.insert(Modifiers::Overclocked, vec![]);
//         m.insert("banana", 1.30);
//         m.insert("cherry", 3.00);
//         m
//     };
// }
