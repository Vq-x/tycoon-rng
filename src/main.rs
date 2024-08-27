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
    let furnace = Furnace {
        multiplier: 47.0,
        rarity: 101_100,
        effects: vec![
            FurnaceTypes::MultiplyIf(1.6, Tags::Magnetic),
            FurnaceTypes::MultiplyIf(1.6, Tags::Fire(1.0)),
            FurnaceTypes::MultiplyIf(0.2, Tags::Fueled),
        ],
        ..Default::default()
    };
    println!("{}", to_string(&furnace).unwrap());
    // println!("ores: {:?}", ores.ores);
}
