mod types;
use std::vec;

use types::{
    enums::{Modifiers, Multipliers, Tags, Vulnerabilities},
    furnace::Furnace,
    mine::Mine,
    ore::Ore,
    upgrader::Upgrader,
    utils::Modify,
};

fn main() {
    let mut gut_dripper = Mine {
        drop_rate: 2.2,
        value: 238.0,
        rarity: 1_150_000,
        modifiers: Modifiers::Golden,
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

    let mut upgrader = Upgrader {
        multiplier: 67.5,
        ..Default::default()
    };

    let mut furnace = Furnace {
        multiplier: 20.0,
        modifiers: Modifiers::Standard,
        rarity: 1000,
        multiplies: true,
        ..Default::default()
    };

    let mut ores = gut_dripper.spawn_ores(50);

    gut_dripper.modify(Modifiers::OverclockedNegativeGolden);

    surge_dropper.modify(Modifiers::OverclockedNegativeGolden);

    upgrader.modify(Modifiers::Negative);

    furnace.modify(Modifiers::OverclockedNegativeGolden);

    // let sum = ores
    //     .iter_mut()
    //     .map(|o: &mut Ore| {
    //         upgrader.upgrade(o);
    //         o.value
    //     })
    //     .sum::<f32>();

    ores.iter_mut().for_each(|o: &mut Ore| upgrader.upgrade(o));
    println!("----------------------------------");
    println!("upgraded ores: {:#?}", ores);
    furnace.process_ores(&mut ores);
    println!("furnace ores: {:#?}", ores);
    // println!("{:?}", mine);
}
