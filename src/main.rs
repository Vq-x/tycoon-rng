mod json_files;
mod tests;
mod types;
mod utils;
use serde_json::{from_str, to_string, to_string_pretty};
use std::{borrow::BorrowMut, vec};
use types::{
    enums::{
        FurnaceTypes, Immunities, MineTypes, Modifiers, Multipliers, Tags, UpgraderTypes,
        Upgraders, Vulnerabilities,
    },
    furnace::Furnace,
    mine::Mine,
    ore::{Ore, Ores},
    upgrader::Upgrader,
    utils::Modify,
};
use utils::human_readable;

fn main() {
    let mine = Mine {
        value: 75.0,
        drop_rate: 2.3,
        rarity: 6_617_000,
        effects: vec![
            // MineTypes::Vulnerability(Vulnerabilities::Fire),
            // MineTypes::Vulnerability(Vulnerabilities::Acid),
            // MineTypes::Vulnerability(Vulnerabilities::Putrid),
            // MineTypes::Vulnerability(Vulnerabilities::Magnetic),
            MineTypes::Tag(Tags::Glitch, 1),
            // MineTypes::Multiplier(Multipliers::Vulnerable(0.0)),
            // MineTypes::Multiplier(Multipliers::Perfumed(3.6)),
            // MineTypes::Immunity(Immunities::Acid),
        ],
        ..Default::default()
    };
    println!("{}", to_string(&mine).unwrap());
    // println!("ores: {:?}", ores.ores);
}
