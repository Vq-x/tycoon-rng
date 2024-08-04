mod types;
use types::{enums::{Modifiers, Multipliers, Vulnerabilities, MINE_RATES}, structs::{Mine, Modify}};

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
    // println!("{:?}", mine);
    
}
