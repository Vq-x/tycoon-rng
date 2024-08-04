mod types;
use std::vec;

use types::{enums::{Modifiers, Multipliers, Tags, Vulnerabilities}, structs::{Upgrader, Mine, Modify}};

fn main() {
    let mut gut_dripper = Mine::new(
        2.2,
        238.0,
        Modifiers::Golden,
        Some(vec![Multipliers::Wet(2.0)]),
        Some(vec![Vulnerabilities::Acid]),
        None);
    gut_dripper.modify(Modifiers::OverclockedGolden);
    gut_dripper.modify(Modifiers::Negative);

    let mut surge_dropper = Mine::new(
        2.0,
        6.5,
        Modifiers::Standard,
        Some(vec![Multipliers::Wet(1.8)]),
        None,
        None
    );

    surge_dropper.modify(Modifiers::OverclockedNegativeGolden);

    let mut upgrader = Upgrader::new(
        67.5,
        Modifiers::Golden,
        Some(vec![Tags::Wet, Tags::Wet]),
        None,
        None,
        None
    );
    upgrader.modify(Modifiers::Negative);
    
    // println!("{:?}", mine);
    
}
