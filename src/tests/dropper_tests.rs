#[cfg(test)]
mod tests {
    use std::vec;

    use strum::IntoEnumIterator;

    use crate::types::{
        enums::{
            Immunities, MineTypes, Mines, Modifiers, Multipliers, Tags, UpgraderTypes, Upgraders,
            Vulnerabilities,
        },
        mine::Mine,
        upgrader::Upgrader,
    };

    #[test]
    fn test_mine_json() {
        for dropper in Mines::iter() {
            println!("{:?}", dropper);
            let dropper1 = Mine::get_mine(dropper, Modifiers::OverclockedNegativeGolden).unwrap();
            assert!(dropper1.rarity > 0);
            assert!(dropper1.drop_rate > 0.0);
        }
    }
}
