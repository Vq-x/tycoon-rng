#![allow(dead_code)]
#[derive(Debug)]
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
#[derive(Debug)]
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
 
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Multipliers {
    Fire(u8),
    Polished(u8),
    Wet(u8),
    Putrid(u8),
    Fueled(u8),
    Acid(u8),
    Magnetic(u8),
    Air(u8),
    Time(u8)
}
