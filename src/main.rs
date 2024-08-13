mod types;
use std::vec;

use types::{
    enums::{Modifiers, Multipliers, Tags, UpgraderTypes, Vulnerabilities},
    furnace::Furnace,
    mine::Mine,
    ore::Ore,
    upgrader::Upgrader,
    utils::Modify,
};

fn main() {
    let mut gut_dripper = Mine {
        drop_rate: 2.2,
        value: 510.0,
        rarity: 11_50_0000,
        modifiers: Modifiers::Negative,
        adds: vec![Multipliers::Wet(2.0)],
        adds_vulnerabilities: vec![Vulnerabilities::Acid],
        ..Default::default()
    };

    let mut surge_dropper = Mine {
        drop_rate: 2.0,
        value: 6.5,
        modifiers: Modifiers::Standard,
        adds: vec![Multipliers::Wet(1.8)],
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
    let mut furnace = Furnace {
        multiplier: 20.0,
        modifiers: Modifiers::Standard,
        rarity: 1000,
        multiplies: true,
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
    perfect_lawn_og.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_og.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_og.upgrade(&mut ore);
    println!("{:?}", ore);
    perfect_lawn_negative.upgrade(&mut ore);
    println!("{:?}", ore);
    // gut_dripper.modify(Modifiers::OverclockedNegativeGolden);

    // surge_dropper.modify(Modifiers::OverclockedNegativeGolden);

    // upgrader.modify(Modifiers::Negative);

    // furnace.modify(Modifiers::OverclockedNegativeGolden);

    // ores.iter_mut().for_each(|o: &mut Ore| upgrader.upgrade(o));
    // println!("----------------------------------");
    // println!("upgraded ores: {:#?}", ores);
    // furnace.process_ores(&mut ores);
    // println!("furnace ores: {:#?}", ores);
    // println!("{:?}", mine);
}
