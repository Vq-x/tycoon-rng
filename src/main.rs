mod json_files;
mod tests;
mod types;
mod utils;
use serde_json::{from_str, to_string, to_string_pretty};
use std::{borrow::BorrowMut, vec};
use types::{
    enums::{
        FurnaceTypes, Furnaces, Immunities, MineTypes, Mines, Modifiers, Multipliers, Tags,
        UpgraderTypes, Upgraders, Vulnerabilities,
    },
    furnace::Furnace,
    mine::Mine,
    ore::{Ore, Ores},
    upgrader::Upgrader,
    utils::Modify,
};
use utils::human_readable;

fn main() {
    let gut = Mine::get_mine(Mines::GuttationDripper, Modifiers::Negative).unwrap();
    let perfect_lawn = Upgrader::get_upgrader(Upgraders::PerfectLawn, Modifiers::Negative).unwrap();
    let hand_of_poseidon =
        Furnace::get_furnace(Furnaces::HandOfPoseidon, Modifiers::Golden).unwrap();
    let mut ores = gut.spawn_ores(100);
    ores.upgrade(&perfect_lawn);
    println!("ores: {:?}", ores.ores);
    let sum = hand_of_poseidon.process_ores(&mut ores);
    let one_ore = ores.ores[0].clone();
    let one = hand_of_poseidon.process_ores(&mut Ores {
        ores: vec![one_ore],
    });
    println!("one: {}", human_readable(one));
    println!("sum: {}", human_readable(sum / 100.0));

    // println!("ores: {:?}", ores.ores);
}
