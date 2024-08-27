use core::num;
use std::{any::Any, collections::HashMap, mem};

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Error};

use crate::json_files::file::get_json_text;

use super::{
    enums::{
        FurnaceTypes, Furnaces, Modifiers, Multipliers, Tags, RARITY_MULTIPLIERS,
        RATES_FROM_STANDARD,
    },
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn get_furnace(furnace_name: Furnaces, modifier: Modifiers) -> Result<Furnace, Error> {
        // Read the JSON file content
        let file_text = get_json_text("src/json_files/furnaces.json").expect("could not find file");

        // Deserialize the file into a HashMap<String, Furnace>
        let json_map: HashMap<String, Furnace> = from_str(&file_text).expect("problem with json");

        // Convert the enum variant to a string
        let key = furnace_name.get_string();

        // Get the Furnace corresponding to the furnace_name
        let mut furnace = json_map
            .get(&key)
            .expect(&format!("Could not find furnace of type {}", key))
            .clone();
        furnace.modify(modifier);
        Ok(furnace)
    }
    pub fn process_ores(&self, ores: &mut Ores) -> f64 {
        let mut ores = ores.clone();
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
                    FurnaceTypes::ExtraMultiplierIfMoreThanAmount(num, amount) => {
                        if ore.tags.len() > *amount as usize {
                            multiplier *= *num as f64;
                        }
                    }
                    FurnaceTypes::ExtraMultiplierIfUpgradedBy(num, upgrader) => {
                        if ore
                            .upgraded_by
                            .iter()
                            .any(|up2| up2.type_id() == upgrader.type_id())
                        {
                            multiplier *= *num as f64;
                        }
                    }
                    FurnaceTypes::OnlyAccepts(tag) => {
                        if !ore
                            .tags
                            .iter()
                            .any(|t| mem::discriminant(t) == mem::discriminant(tag))
                        {
                            ore.destroy();
                        }
                    }
                    FurnaceTypes::ChanceForEach(value, tag, num) => {
                        let mut total_value = 0.0;
                        let mut i = 0;
                        for t in ore.tags.iter() {
                            if mem::discriminant(t) == mem::discriminant(tag) {
                                if rand::thread_rng().gen_ratio(1, 2) {
                                    total_value += value;
                                } else {
                                    total_value -= value;
                                }
                                i += 1;
                            }
                            if i == *num {
                                break;
                            }
                        }
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
