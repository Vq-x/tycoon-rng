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
    let mine = Mine {
        drop_rate: 0.9,
        value: 647.5,
        rarity: 656_000,
        modifiers: Modifiers::Golden,
        adds: vec![Multipliers::Vulnerable(2.2)],
        ..Default::default()
    };
    let mut ores = mine.spawn_ores(100);
    let mine2 = Mine {
        drop_rate: 5.0,
        value: 100.0,
        rarity: 100_000,
        modifiers: Modifiers::Golden,
        adds: vec![Multipliers::Vulnerable(2.2)],
        adds_vulnerabilities: vec![Vulnerabilities::Fire, Vulnerabilities::Acid],
        ..Default::default()
    };
    // ores.combine(&mut mine2.spawn_ores(100));
    let mut upgrader = Upgrader {
        multiplier: 40.0,
        modifiers: Modifiers::Standard,
        rarity: 3660000,
        effects: vec![UpgraderTypes::Adds(Tags::Vulnerable, 1)],
    };
    // ores.upgrade(&upgrader);
    let upgrader2 = Upgrader::get_upgrader(Upgraders::OreHacker, Modifiers::Standard).unwrap();
    println!("{:?}", upgrader2);
    println!("{:?}", human_readable(upgrader2.rarity));
    // upgrader.modify(Modifiers::Standard);
    println!("{}", to_string(&upgrader).unwrap());

    println!("total ores count: {:?}", ores.ores.len());
    println!("{:?}", ores.ores.iter().filter(|ore| ore.destroyed).count());
    ores.upgrade(&upgrader2);
    println!(
        "percent ores destroyed:{:?}",
        ores.ores.iter().filter(|ore| ore.destroyed).count() as f32 / ores.ores.len() as f32
            * 100.0
    );
    // println!("ores: {:?}", ores.ores);
}
