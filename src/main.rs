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
    let mut digital_anomaly = Mine {
        drop_rate: 1.0,
        value: 400.0,
        rarity: 2_500_000,
        modifiers: Modifiers::Standard,
        ..Default::default()
    };
    let mut gut_dripper = Mine {
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
    let cursed_siege = Upgrader {
        multiplier: 300.6,
        modifiers: Modifiers::OverclockedGolden,
        rarity: 22_700_000,
        effects: vec![
            UpgraderTypes::Adds(Tags::Fueled, 1),
            UpgraderTypes::MultiplyIf(2.0, Tags::Aired),
            UpgraderTypes::Removes(Tags::Aired),
        ],
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

    let wind_tunnel = Upgrader {
        multiplier: 98.5,
        modifiers: Modifiers::Negative,
        ..Default::default()
    };

    let mut perfect_lawn_negative = Upgrader {
        multiplier: 143.5,
        effects: vec![UpgraderTypes::AddsIfThen(Tags::Wet, 1, Tags::Fire(1), 2)],
        modifiers: Modifiers::Negative,
        ..Default::default()
    };

    let mut perfect_lawn_og = perfect_lawn_negative.clone();
    perfect_lawn_og.modify(Modifiers::OverclockedGolden);

    let mut hand_of_poseidon = Furnace {
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

    // gut_dripper.modify(Modifiers::OverclockedGolden);
    // let mut ores = gut_dripper.spawn_ores(10);
    // gut_dripper.modify(Modifiers::Negative);
    // let ores2 = gut_dripper.spawn_ores(10);
    // ores.extend(ores2.iter().cloned());
    let mut ores = digital_anomaly.spawn_ores(10);
    println!("ores amount: {:?}", ores.ores.len());
    println!("before upgrades: {:?}", ores.ores);
    // ores.iter_mut().for_each(|ore| cursed_siege.upgrade(ore));
    ores.upgrade(&cursed_siege);
    println!("{:?}", ores.ores);

    // ores.iter_mut()
    //     .for_each(|ore| perfect_lawn_negative.upgrade(ore));
    ores.upgrade(&perfect_lawn_negative);
    println!("{:?}", ores.ores);

    // ores.iter_mut()
    //     .for_each(|ore| perfect_lawn_negative.upgrade(ore));
    ores.upgrade(&perfect_lawn_negative);
    println!("{:?}", ores.ores);

    // ores.iter_mut()
    //     .for_each(|ore| perfect_lawn_negative.upgrade(ore));
    ores.upgrade(&perfect_lawn_negative);
    println!("{:?}", ores.ores);
    // println!(
    //     "one ore: {:?} one ore times total amount: {:?}",
    //     hand_of_poseidon.process_ores(&mut vec![ores[0].clone()]),
    //     hand_of_poseidon.process_ores(&mut vec![ores[0].clone()]) * ores.len() as f64
    // );
    println!("ores amount: {:?}", ores.ores.len());
    println!("{:?}", hand_of_poseidon.process_ores(&mut ores));
}
