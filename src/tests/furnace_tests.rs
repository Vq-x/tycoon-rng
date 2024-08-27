#[cfg(test)]
mod tests {
    use std::vec;

    use strum::IntoEnumIterator;

    use crate::types::{
        enums::{
            Furnaces, Immunities, MineTypes, Mines, Modifiers, Multipliers, Tags, UpgraderTypes,
            Upgraders, Vulnerabilities,
        },
        furnace::Furnace,
        mine::Mine,
        upgrader::Upgrader,
    };

    #[test]
    fn test_furnace_json() {
        for furnace in Furnaces::iter() {
            println!("{:?}", furnace);
            let furnace1 =
                Furnace::get_furnace(furnace, Modifiers::OverclockedNegativeGolden).unwrap();
            assert!(furnace1.rarity > 0);
            assert!(furnace1.multiplier > 0.0);
            assert!(furnace1.modifiers == Modifiers::OverclockedNegativeGolden);
        }
    }
}
