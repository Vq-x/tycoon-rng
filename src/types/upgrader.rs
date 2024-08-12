use super::{
    enums::{
        Modifiers, Multipliers, Tags, UpgraderTypes, Vulnerabilities, RARITY_MULTIPLIERS,
        RATES_FROM_STANDARD,
    },
    ore::Ore,
    utils::{Modify, ModifyStandard},
};

#[derive(Debug, Clone)]
pub struct Upgrader {
    pub multiplier: f32,
    pub multipliers: Vec<Multipliers>,
    pub modifiers: Modifiers,
    pub rarity: u64,
    pub effects: Vec<UpgraderTypes>,
    pub adds: Vec<Tags>,
    pub adds_vulnerabilities: Vec<Vulnerabilities>,
    pub removes: Vec<Tags>,
    pub destroys: Vec<Tags>,
}

impl Upgrader {
    pub fn upgrade(&self, ore: &mut Ore) {
        // multiply ore by multiplier
        let mut multiplier = self.multiplier;

        // // add tags to ore if any of the adds tags are present
        // self.adds.iter().for_each(|tag| ore.add_tag(tag.clone()));

        // // add vulnerabilities to ore if any of the adds_vulnerabilities tags are present
        // self.adds_vulnerabilities
        //     .iter()
        //     .for_each(|vulnerability| ore.add_vulnerability(vulnerability.clone()));

        // // remove tags from ore if any of the removes tags are present
        // self.removes
        //     .iter()
        //     .for_each(|tag| ore.remove_tag(tag.clone()));

        // // destroy ore if any of the destroys tags are present
        // self.destroys.iter().for_each(|tag| {
        //     if ore.tags.contains(tag) {
        //         ore.destroy();
        //     }
        // });
        for effect in self.effects.iter() {
            println!("effect: {:?}", effect);
            match effect {
                // adds Wet 1x if Fire 2x if None
                UpgraderTypes::AddsIfThen(adder, count, tag_if, count2) => {
                    // if contains tag fire
                    let times_to_add = if ore.tags.contains(tag_if) {
                        count
                    } else {
                        count2
                    };

                    for _ in 0..*times_to_add {
                        ore.add_tag(adder.clone());
                    }
                }
                // adds Ice if Wet
                UpgraderTypes::AddsIf(adder, tag) => {
                    if ore.tags.contains(tag) {
                        ore.add_tag(adder.clone());
                    }
                }
                // Adds Tag
                UpgraderTypes::Adds(tag, amount) => {
                    for _ in 0..*amount {
                        ore.add_tag(tag.clone());
                    }
                }
                // Adds Wet If not on Fire
                UpgraderTypes::AddsIfNot(adds, not_tag) => {
                    if !ore.tags.contains(not_tag) {
                        ore.add_tag(adds.clone());
                    }
                }
                // replaces Fire with Wet
                UpgraderTypes::Replaces(replacing, replace) => {
                    if ore.tags.contains(replacing) {
                        ore.add_tag(replace.clone());
                        ore.remove_tag(replacing.clone());
                    }
                }
                // extra 1.2x for each fire tag
                UpgraderTypes::ExtraForEach(num, tag) => {
                    for iter_tag in ore.tags.iter() {
                        if iter_tag == tag {
                            multiplier *= num;
                        }
                    }
                }
                // Multiplies value by 10x if glitch
                UpgraderTypes::MultiplyIf(num, tag) => {
                    if ore.tags.contains(tag) {
                        multiplier *= num;
                    }
                }
                // extra upgrade logarithmically based on ore value
                UpgraderTypes::ExtraLogarithmic => {
                    let eq = log_base(ore.value + 1.0, 1000.0) / 4.0;
                    // ore.multiply_by(eq);
                    multiplier *= eq;
                }
                UpgraderTypes::Removes(tag) => {
                    ore.remove_tag(tag.clone());
                }
                _ => {}
            }
        }
        ore.multiply_by(multiplier);
    }
}

impl Modify for Upgrader {
    fn modify(&mut self, modifier: Modifiers) {
        // println!("before match:{:?}", self);
        self.to_standard();
        // println!("{:?}", self.modifiers);
        assert!(self.modifiers == Modifiers::Standard);
        // println!("first match:{:?}", self);
        self.modify_from_standard(&modifier);
        // println!("second match:{:?}", self);
    }
}

impl Default for Upgrader {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            multipliers: vec![],
            modifiers: Modifiers::Standard,
            rarity: 1000,
            effects: vec![],
            adds: vec![],
            adds_vulnerabilities: vec![],
            removes: vec![],
            destroys: vec![],
        }
    }
}

impl ModifyStandard for Upgrader {
    fn to_standard(&mut self) {
        let rates = match self.modifiers {
            Modifiers::Overclocked => RATES_FROM_STANDARD.get(&Modifiers::Overclocked),
            Modifiers::Golden => RATES_FROM_STANDARD.get(&Modifiers::Golden),
            Modifiers::Negative => RATES_FROM_STANDARD.get(&Modifiers::Negative),
            Modifiers::OverclockedGolden => RATES_FROM_STANDARD.get(&Modifiers::OverclockedGolden),
            Modifiers::OverclockedNegative => {
                RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegative)
            }
            Modifiers::NegativeGolden => RATES_FROM_STANDARD.get(&Modifiers::NegativeGolden),
            Modifiers::OverclockedNegativeGolden => {
                RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegativeGolden)
            }
            _ => None,
        };

        if let Some(rate) = rates {
            self.multiplier = (self.multiplier + rate[1]) / rate[0];
        }

        let rarity = RARITY_MULTIPLIERS.get(&self.modifiers).unwrap();

        self.rarity /= rarity;

        self.modifiers = Modifiers::Standard;
    }

    fn modify_from_standard(&mut self, to_modifier: &Modifiers) {
        let rates = match RATES_FROM_STANDARD.get(to_modifier) {
            Some(rate) => rate,
            None => return, // Exit early if `to_modifier` does not match any known variant
        };

        self.multiplier = (rates[0] * self.multiplier) - rates[1];

        let rarity = RARITY_MULTIPLIERS.get(&self.modifiers).unwrap();

        self.rarity *= rarity;

        self.modifiers = to_modifier.clone();
    }
}

fn log_base(x: f32, base: f32) -> f32 {
    x.log10() / base.log10()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::types::{enums::Immunities, mine::Mine};

    // Import the functions from the parent module
    use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add(2, 3), 5);
    //     assert_eq!(add(-1, 1), 0);
    //     assert_eq!(add(0, 0), 0);
    // }
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
        assert!(ore.value == 2531.56);
        let steampunk_overdrive = Upgrader {
            multiplier: 26.0,
            rarity: 14_800,
            effects: vec![UpgraderTypes::ExtraForEach(1.12, Tags::Fueled)],
            ..Default::default()
        };
        steampunk_overdrive.upgrade(&mut ore);
        assert_eq!(ore.value, 82_565.32);
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
        assert_eq!(ore.value, 146.11276);
        data_encryption.upgrade(&mut ore);
        assert_eq!(
            (100.0 * ore.value).trunc() / 100.0,
            (100.0 * 765.41_f32).trunc() / 100.0
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
            effects: vec![UpgraderTypes::Adds(Tags::Fire(3), 1)],
            ..Default::default()
        };
        let wind_tunnel = Upgrader {
            multiplier: 28.82,
            rarity: 55_000,
            effects: vec![
                UpgraderTypes::Adds(Tags::Aired, 1),
                UpgraderTypes::Removes(Tags::Wet),
                UpgraderTypes::Replaces(Tags::Fire(3), Tags::Fire(3)),
            ],
            ..Default::default()
        };

        let billy_fishtank = Upgrader {
            multiplier: 129.4,
            effects: vec![
                UpgraderTypes::Adds(Tags::Wet, 2),
                UpgraderTypes::Removes(Tags::Fire(1)),
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
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 228_596.64);
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
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 29_580_404.0);
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
        assert_eq!((ore.value * 100.0).trunc() / 100.0, 266_223_630.0);
        data_encryption.upgrade(&mut ore);
    }
}
