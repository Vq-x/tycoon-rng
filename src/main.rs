mod types;
use std::vec;

use types::{
    enums::{Modifiers, Multipliers, Tags, Vulnerabilities},
    structs::{Furnace, Mine, Modify, Upgrader},
};

fn main() {
    let mut gut_dripper = Mine::new(
        2.2,
        238.0,
        1_150_000,
        Modifiers::Golden,
        Some(vec![Multipliers::Wet(2.0)]),
        Some(vec![Vulnerabilities::Acid]),
        None,
    );
    gut_dripper.modify(Modifiers::OverclockedNegativeGolden);
    let mut surge_dropper = Mine::new(
        2.0,
        6.5,
        1000,
        Modifiers::Standard,
        Some(vec![Multipliers::Wet(1.8)]),
        None,
        None,
    );
    surge_dropper.modify(Modifiers::OverclockedNegativeGolden);

    let mut upgrader = Upgrader::new(
        67.5,
        Modifiers::Golden,
        1000,
        Some(vec![Tags::Wet, Tags::Wet]),
        None,
        None,
        None,
    );
    upgrader.modify(Modifiers::Negative);

    let mut furnace = Furnace::new(20.0, Modifiers::Standard, 1000, true, None, None);
    furnace.modify(Modifiers::OverclockedNegativeGolden);
    // println!("{:?}", mine);
}
