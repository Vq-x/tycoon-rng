mod tests;
mod types;
use std::vec;

use types::{
    enums::{FurnaceTypes, Modifiers, Multipliers, Tags, UpgraderTypes, Vulnerabilities},
    furnace::Furnace,
    mine::Mine,
    ore::Ore,
    upgrader::Upgrader,
    utils::Modify,
};

fn main() {
    let gut_dripper = Mine {
        drop_rate: 2.2,
        value: 510.0,
        rarity: 11_500_000,
        modifiers: Modifiers::Negative,
        adds: vec![Multipliers::Wet(2.0)],
        adds_vulnerabilities: vec![Vulnerabilities::Acid],
        ..Default::default()
    };

    let surge_dropper = Mine {
        drop_rate: 2.0,
        value: 6.5,
        modifiers: Modifiers::Standard,
        adds: vec![Multipliers::Wet(1.8)],
        ..Default::default()
    };
    let aurora_tundra = Upgrader {
        multiplier: 342.5,
        effects: vec![
            UpgraderTypes::ExtraForEach(1.2, Tags::Fire(1)),
            UpgraderTypes::Removes(Tags::Fire(1)),
        ],
        modifiers: Modifiers::NegativeGolden,
        ..Default::default()
    };

    let perfect_lawn_negative = Upgrader {
        multiplier: 143.5,
        effects: vec![UpgraderTypes::AddsIfThen(Tags::Wet, 1, Tags::Fire(1), 2)],
        modifiers: Modifiers::Negative,
        ..Default::default()
    };

    let mut perfect_lawn_og = perfect_lawn_negative.clone();
    perfect_lawn_og.modify(Modifiers::OverclockedGolden);

    let hand_of_poseidon = Furnace {
        multiplier: 67.5,
        modifiers: Modifiers::Golden,
        rarity: 9_990_000,
        effects: vec![FurnaceTypes::MultipliesByTag(Tags::Wet, 1.0)],
        ..Default::default()
    };

    // let mut ores = gut_dripper.spawn_ores(50);
    /*
    TODO:
        - make sure the ores that have a 2x multiplier for wet tag
          get applied when those ores go through a upgrader that adds wet.

     */
    let mut ore = gut_dripper.spawn_ore();

    println!("before upgrades: {:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    println!("{:?}", hand_of_poseidon.process_ores(&mut vec![ore]));
}
