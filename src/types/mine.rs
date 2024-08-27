use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, Error};

use crate::json_files::file::get_json_text;

use super::{
    enums::{
        Immunities, MineTypes, Mines, Modifiers, Multipliers, Tags, Vulnerabilities,
        MINE_DROP_RATES, MINE_RATES, RARITY_MULTIPLIERS,
    },
    ore::{Ore, Ores},
    utils::Modify,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mine {
    pub drop_rate: f32,
    pub value: f64,
    pub effects: Vec<MineTypes>,
    pub modifiers: Modifiers,
    pub rarity: u64,
}

impl Default for Mine {
    fn default() -> Self {
        Self {
            drop_rate: 1.0,
            value: 1.0,
            effects: vec![],
            modifiers: Modifiers::Standard,
            rarity: 1000,
        }
    }
}
impl Mine {
    pub fn get_mine(mine_name: Mines, modifier: Modifiers) -> Result<Mine, Error> {
        // Read the JSON file content
        let file_text = get_json_text("src/json_files/mines.json").expect("could not find file");

        // Deserialize the file into a HashMap<String, Mine>
        let json_map: HashMap<String, Mine> = from_str(&file_text).expect("problem with json");

        // Convert the enum variant to a string
        let key = mine_name.get_string();

        // Get the Mine corresponding to the mine_name
        let mut mine = json_map
            .get(&key)
            .expect(&format!("Could not find mine of type {}", key))
            .clone();
        mine.modify(modifier);
        // Return the mine
        Ok(mine)
    }

    pub fn spawn_ore(&self) -> Ore {
        let value = self.value;
        // let multipliers = &self.adds;
        // let immunities = &self.adds_immunities;
        // let vulnerabilities = &self.adds_vulnerabilities;
        let mut multipliers = Vec::new();
        let mut tags = Vec::new();
        let mut immunities = Vec::new();
        let mut vulnerabilities = Vec::new();
        for effect in self.effects.iter() {
            match effect {
                MineTypes::Tag(tag, amount) => {
                    for _ in 0..*amount {
                        tags.push(tag.clone());
                    }
                }
                MineTypes::Multiplier(mult) => {
                    multipliers.push(mult.clone());
                }
                MineTypes::Immunity(immunity) => {
                    immunities.push(immunity.clone());
                }
                MineTypes::Vulnerability(vulnerability) => {
                    vulnerabilities.push(vulnerability.clone());
                }
            }
        }
        // Ore::new(
        //     value,
        //     Some(multipliers.clone()),
        //     None,
        //     Some(immunities.clone()),
        //     Some(vulnerabilities.clone()),
        // )

        Ore {
            value,
            tags,
            multipliers,
            immunities,
            vulnerabilities,
            ..Default::default()
        }
    }

    pub fn spawn_ores(&self, seconds: u16) -> Ores {
        let amount = (seconds as f32 * self.drop_rate).floor();
        let group: Vec<Ore> = (0..amount as u16).map(|_| self.spawn_ore()).collect();
        Ores { ores: group }
    }
}

trait ModifyMine {
    fn to_standard(&mut self);
    fn to_standard_drop_rate(&mut self);
    fn modify_from_standard(&mut self, to_modifier: &Modifiers);
    fn apply_standard_drop_rate(&mut self, to_modifier: &Modifiers);
}

impl Modify for Mine {
    fn modify(&mut self, modifier: Modifiers) {
        // println!("before match:{:?}", self);

        // Consolidate the drop rate and standardization steps
        if matches!(
            self.modifiers,
            Modifiers::Overclocked
                | Modifiers::OverclockedGolden
                | Modifiers::OverclockedNegative
                | Modifiers::NegativeGolden
                | Modifiers::OverclockedNegativeGolden
        ) {
            self.to_standard_drop_rate();
        }

        if !matches!(self.modifiers, Modifiers::Standard) {
            self.to_standard();
        }

        // println!("first match:{:?}", self);
        assert!(self.modifiers == Modifiers::Standard);

        // Consolidate the application of new modifiers
        if matches!(
            modifier,
            Modifiers::Overclocked
                | Modifiers::OverclockedGolden
                | Modifiers::OverclockedNegative
                | Modifiers::OverclockedNegativeGolden
        ) {
            self.apply_standard_drop_rate(&modifier);
        }

        if matches!(
            modifier,
            Modifiers::Golden
                | Modifiers::Negative
                | Modifiers::OverclockedGolden
                | Modifiers::OverclockedNegative
                | Modifiers::NegativeGolden
                | Modifiers::OverclockedNegativeGolden
                | Modifiers::Standard
        ) {
            self.modify_from_standard(&modifier);
        }

        // println!("second match:{:?}", self);
    }
}

impl ModifyMine for Mine {
    fn to_standard(&mut self) {
        // get rate to divide current modifier by itself to turn it into standard
        let rate = match self.modifiers {
            Modifiers::Golden | Modifiers::OverclockedGolden => MINE_RATES.get(&Modifiers::Golden),
            Modifiers::Negative | Modifiers::OverclockedNegative => {
                MINE_RATES.get(&Modifiers::Negative)
            }
            Modifiers::NegativeGolden | Modifiers::OverclockedNegativeGolden => {
                MINE_RATES.get(&Modifiers::NegativeGolden)
            }
            _ => None,
        };
        // divide value by rate
        if let Some(rate) = rate {
            self.value /= *rate as f64;
        }

        let rarity = RARITY_MULTIPLIERS.get(&self.modifiers).unwrap();

        self.rarity /= rarity;

        self.modifiers = Modifiers::Standard;
    }

    fn to_standard_drop_rate(&mut self) {
        if let Some(overclocked_rate) = MINE_DROP_RATES.get(&Modifiers::Overclocked) {
            match self.modifiers {
                Modifiers::Overclocked
                | Modifiers::OverclockedGolden
                | Modifiers::OverclockedNegative
                | Modifiers::OverclockedNegativeGolden => {
                    self.drop_rate = (self.drop_rate - overclocked_rate[1]) / overclocked_rate[0];
                }
                _ => (),
            }
        }
    }

    fn modify_from_standard(&mut self, to_modifier: &Modifiers) {
        match to_modifier {
            Modifiers::Standard | Modifiers::Overclocked => {
                self.modifiers = to_modifier.clone();
            }
            Modifiers::Golden | Modifiers::OverclockedGolden => {
                if let Some(golden_rate) = MINE_RATES.get(&Modifiers::Golden) {
                    self.value *= *golden_rate as f64;
                    self.modifiers = to_modifier.clone();
                }
            }
            Modifiers::Negative | Modifiers::OverclockedNegative => {
                if let Some(negative_rate) = MINE_RATES.get(&Modifiers::Negative) {
                    self.value *= *negative_rate as f64;
                    self.modifiers = to_modifier.clone();
                }
            }
            Modifiers::NegativeGolden | Modifiers::OverclockedNegativeGolden => {
                if let Some(negative_golden_rate) = MINE_RATES.get(&Modifiers::NegativeGolden) {
                    self.value *= *negative_golden_rate as f64;
                    self.modifiers = to_modifier.clone();
                }
            }
        }
        let rarity = RARITY_MULTIPLIERS.get(&self.modifiers).unwrap();

        self.rarity *= rarity;
    }

    fn apply_standard_drop_rate(&mut self, to_modifier: &Modifiers) {
        if let Some(overclocked_rate) = MINE_DROP_RATES.get(&Modifiers::Overclocked) {
            match to_modifier {
                Modifiers::Overclocked
                | Modifiers::OverclockedGolden
                | Modifiers::OverclockedNegative
                | Modifiers::OverclockedNegativeGolden => {
                    self.drop_rate = (overclocked_rate[0] * self.drop_rate) + overclocked_rate[1];
                    self.modifiers = to_modifier.clone();
                }
                _ => (),
            }
        }
    }
}
