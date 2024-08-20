use core::num;
use std::mem;

use super::{
    enums::{FurnaceTypes, Modifiers, Multipliers, Tags, RARITY_MULTIPLIERS, RATES_FROM_STANDARD},
    ore::{Ore, Ores},
    utils::{Modify, ModifyStandard},
};

/*
TODO:
The furnace doesn't multiply the exact multiplier when processing ores,
it has a little more which could make the program inconsistent when
introducing bigger numbers.
Need to figure out how to fix this.
Whole numbers are fine, but decimals are not.
*/

#[derive(Debug, Clone)]
pub struct Furnace {
    pub multiplier: f32,
    pub modifiers: Modifiers,
    pub rarity: u64,
    pub effects: Vec<FurnaceTypes>,
}

impl Default for Furnace {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            modifiers: Modifiers::Standard,
            rarity: 1000,
            effects: vec![],
        }
    }
}
impl Furnace {
    pub fn process_ores(&self, ores: &mut Ores) -> f64 {
        for ore in ores.ores.iter_mut() {
            let mut multiplier = self.multiplier as f64;
            for effect in self.effects.iter() {
                match effect {
                    FurnaceTypes::AddForEach(num, tag) => {
                        let amount = ore
                            .tags
                            .iter()
                            .filter(|t| mem::discriminant(*t) == mem::discriminant(tag))
                            .count();
                        multiplier += (amount as f64) * (*num as f64);
                    }
                    FurnaceTypes::MultipliesByTag(tag, if_none) => {
                        let mut amount = ore
                            .tags
                            .iter()
                            .filter(|t| mem::discriminant(*t) == mem::discriminant(tag))
                            .count();
                        multiplier *= (amount as f64) + *if_none as f64;
                    }
                    FurnaceTypes::Refuses(tag) => {
                        if ore
                            .tags
                            .iter()
                            .any(|t| mem::discriminant(t) == mem::discriminant(tag))
                        {
                            ore.destroy();
                        }
                    }
                    FurnaceTypes::MultiplyIf(num, tag) => {
                        if ore
                            .tags
                            .iter()
                            .any(|t| mem::discriminant(t) == mem::discriminant(tag))
                        {
                            multiplier *= *num as f64;
                        }
                    }
                    FurnaceTypes::ExtraMultiplierEvery(num) => {
                        multiplier += (*num as f64) * (ore.tags.len() as f64);
                    }
                }
            }
            ore.multiply_by(multiplier);
        }
        let sum: f64 = ores
            .ores
            .iter()
            .filter(|o| !o.destroyed)
            .map(|o| o.value)
            .sum();
        sum
    }
}
impl Modify for Furnace {
    fn modify(&mut self, modifier: Modifiers) {
        // println!("before match: {:?}", self);
        self.to_standard();
        // println!("first match: {:?}", self);

        match modifier {
            Modifiers::Standard
            | Modifiers::Overclocked
            | Modifiers::Golden
            | Modifiers::Negative
            | Modifiers::OverclockedGolden
            | Modifiers::OverclockedNegative
            | Modifiers::NegativeGolden
            | Modifiers::OverclockedNegativeGolden => {
                self.modify_from_standard(&modifier);
            }
        }

        // println!("second match: {:?}", self);
    }
}
impl ModifyStandard for Furnace {
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
