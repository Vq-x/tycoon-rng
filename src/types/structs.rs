#![allow(dead_code)]
#![allow(unused)]

use super::enums::{Tags, Vulnerabilities, Multipliers};



struct Mine {
    drop_rate: u16,
    value: u16,
    adds: Vec<Multipliers>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    adds_immunities: Vec<Tags>,
}

struct Upgrader {
    multiplier: u16,
    adds: Vec<Tags>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    removes: Vec<Tags>,
    destroys: Vec<Tags>,
}

struct Furnace {
    multiplier: u16,
    extra: Vec<Multipliers>,
    refuses: Vec<Tags>,
}

struct Ore {
    value: u16,
    mutiplier: u16,
    tags: Vec<Tags>,
    immunities: Vec<Tags>,
    vulnerabilities: Vec<Vulnerabilities>,
}