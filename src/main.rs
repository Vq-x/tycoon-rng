mod json_files;
mod tests;
mod types;
mod utils;
use serde_json::{from_str, to_string, to_string_pretty};
use std::{borrow::BorrowMut, vec};
use types::{
    enums::{
        FurnaceTypes, Immunities, Modifiers, Multipliers, Tags, UpgraderTypes, Upgraders,
        Vulnerabilities,
    },
    furnace::Furnace,
    mine::Mine,
    ore::{Ore, Ores},
    upgrader::Upgrader,
    utils::Modify,
};
use utils::human_readable;

fn main() {
    let mut upgrader = Upgrader {
        multiplier: 80.0,
        effects: vec![
            UpgraderTypes::AddsIf(Tags::Shield, Tags::Glitch),
            UpgraderTypes::MultiplyIf(0.75, Tags::Putrid),
        ],
        rarity: 7_500_000,
        // modifiers: Modifiers::Negative,
        ..Default::default()
    };
    let upgrader2 =
        Upgrader::get_upgrader(Upgraders::PerfectLawn, Modifiers::OverclockedNegativeGolden)
            .unwrap();
    println!("{:?}", upgrader2);
    println!("{:?}", human_readable(upgrader2.rarity));
    // upgrader.modify(Modifiers::Standard);
    println!("{}", to_string(&upgrader).unwrap());
}
