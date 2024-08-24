#[cfg(test)]
mod tests {
    use std::vec;

    use crate::types::{
        enums::{Immunities, Modifiers, Multipliers, Tags, UpgraderTypes, Vulnerabilities},
        mine::Mine,
        upgrader::Upgrader,
    };

    #[test]
    fn test_upgrader_1() {
        let corrosive_sentinel = Mine {
            drop_rate: 4.0,
            value: 38.0,
            rarity: 12_600,
            modifiers: Modifiers::Golden,
            adds: vec![Multipliers::Acid(2.0), Multipliers::Putrid(1.0)],
            adds_vulnerabilities: vec![Vulnerabilities::Fire],
            adds_immunities: vec![Immunities::Acid],
        };
        let mut ore = corrosive_sentinel.spawn_ore();
        let steam_engine = Upgrader {
            multiplier: 66.62,
            rarity: 450_000,
            effects: vec![UpgraderTypes::Adds(Tags::Fueled, 2)],
            // adds: vec![Tags::Fueled, Tags::Fueled],
            ..Default::default()
        };
        steam_engine.upgrade(&mut ore);
        assert_eq!(
            ore.tags.iter().filter(|&tag| tag == &Tags::Fueled).count(),
            2
        );
        assert_eq!(ore.value, 2531.560104370117);
        let steampunk_overdrive = Upgrader {
            multiplier: 26.0,
            rarity: 14_800,
            effects: vec![UpgraderTypes::ExtraForEach(1.12, Tags::Fueled)],
            ..Default::default()
        };
        steampunk_overdrive.upgrade(&mut ore);
        assert_eq!(ore.value, 82_565.32088291191);
        assert_eq!(
            ore.tags.iter().filter(|&tag| tag == &Tags::Fueled).count(),
            2
        );
    }
    #[test]
    fn test_upgrader_log() {
        let corrosive_sentinel = Mine {
            drop_rate: 4.0,
            value: 38.0,
            rarity: 12_600,
            modifiers: Modifiers::Golden,
            adds: vec![Multipliers::Acid(2.0), Multipliers::Putrid(1.0)],
            adds_vulnerabilities: vec![Vulnerabilities::Fire],
            adds_immunities: vec![Immunities::Acid],
        };
        let mut ore = corrosive_sentinel.spawn_ore();
        let data_encryption = Upgrader {
            multiplier: 29.0,
            rarity: 250_000,
            effects: vec![UpgraderTypes::ExtraLogarithmic],
            // adds: vec![Tags::Fueled, Tags::Fueled],
            ..Default::default()
        };
        data_encryption.upgrade(&mut ore);
        assert_eq!(ore.value, 146.1127643585205);
        data_encryption.upgrade(&mut ore);
        assert_eq!(
            (100.0 * ore.value).trunc() / 100.0,
            (100.0 * 765.4_f64).trunc() / 100.0
        );
    }
    #[test]
    fn test_upgrader_log2() {
        let shadow_veil = Mine {
            drop_rate: 0.9,
            value: 647.5,
            rarity: 656_000,
            modifiers: Modifiers::Golden,
            adds: vec![Multipliers::Vulnerable(2.2)],
            ..Default::default()
        };
        let mut ore = shadow_veil.spawn_ore();
        let flame_blaster = Upgrader {
            multiplier: 12.25,
            effects: vec![UpgraderTypes::Adds(Tags::Fire(3.0), 1)],
            ..Default::default()
        };
        let wind_tunnel = Upgrader {
            multiplier: 28.82,
            rarity: 55_000,
            effects: vec![
                UpgraderTypes::Adds(Tags::Aired, 1),
                UpgraderTypes::Removes(Tags::Wet),
                UpgraderTypes::Replaces(Tags::Fire(3.0), Tags::Fire(3.0)),
            ],
            ..Default::default()
        };

        let billy_fishtank = Upgrader {
            multiplier: 129.4,
            effects: vec![
                UpgraderTypes::Adds(Tags::Wet, 2),
                UpgraderTypes::Removes(Tags::Fire(1.0)),
            ],
            ..Default::default()
        };
        let cold_snap = Upgrader {
            multiplier: 9.0,
            effects: vec![
                UpgraderTypes::AddsIf(Tags::Ice, Tags::Wet),
                UpgraderTypes::AddsIf(Tags::Ice, Tags::Aired),
            ],
            ..Default::default()
        };
        let data_encryption = Upgrader {
            multiplier: 29.0,
            rarity: 250_000,
            effects: vec![UpgraderTypes::ExtraLogarithmic],
            // adds: vec![Tags::Fueled, Tags::Fueled],
            ..Default::default()
        };
        flame_blaster.upgrade(&mut ore);
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Fire(_)))
                .count(),
            1
        );
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 7_931.87);
        wind_tunnel.upgrade(&mut ore);
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Fire(_)))
                .count(),
            1
        );
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Aired))
                .count(),
            1
        );
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 228_596.63);
        billy_fishtank.upgrade(&mut ore);
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Fire(_)))
                .count(),
            0
        );
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Aired))
                .count(),
            1
        );
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Wet))
                .count(),
            2
        );
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 29_580_403.18);
        cold_snap.upgrade(&mut ore);
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Fire(_)))
                .count(),
            0
        );
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Aired))
                .count(),
            1
        );
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Wet))
                .count(),
            2
        );
        assert_eq!(
            ore.tags
                .iter()
                .filter(|&tag| matches!(tag, &Tags::Aired))
                .count(),
            1
        );
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 266_223_628.65);
    }
}
