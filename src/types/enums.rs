#![allow(dead_code)]

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
