use super::{
    enums::{
        Immunities, Modifiers, Multipliers, Tags, Vulnerabilities, MINE_DROP_RATES, MINE_RATES,
        RARITY_MULTIPLIERS,
    },
    ore::{Ore, Ores},
    utils::Modify,
};

#[derive(Debug, Clone)]
pub struct Mine {
    pub drop_rate: f32,
    pub value: f64,
    pub adds: Vec<Multipliers>,
    pub adds_vulnerabilities: Vec<Vulnerabilities>,
    pub adds_immunities: Vec<Immunities>,
    pub modifiers: Modifiers,
    pub rarity: u64,
}

impl Default for Mine {
    fn default() -> Self {
        Self {
            drop_rate: 1.0,
            value: 1.0,
            adds: vec![],
            adds_vulnerabilities: vec![],
            adds_immunities: vec![],
            modifiers: Modifiers::Standard,
            rarity: 1000,
        }
    }
}
impl Mine {
    // pub fn new(
    //     drop_rate: f32,
    //     value: f32,
    //     rarity: u64,
    //     modifiers: Modifiers,
    //     adds: Option<Vec<Multipliers>>,
    //     adds_vulnerabilities: Option<Vec<Vulnerabilities>>,
    //     adds_immunities: Option<Vec<Tags>>,
    // ) -> Self {
    //     Self {
    //         drop_rate,
    //         value,
    //         modifiers,
    //         rarity,
    //         adds: adds.unwrap_or_default(),
    //         adds_vulnerabilities: adds_vulnerabilities.unwrap_or_default(),
    //         adds_immunities: adds_immunities.unwrap_or_default(),
    //     }
    // }

    pub fn spawn_ore(&self) -> Ore {
        let value = self.value;
        let multipliers = &self.adds;
        let immunities = &self.adds_immunities;
        let vulnerabilities = &self.adds_vulnerabilities;

        // Ore::new(
        //     value,
        //     Some(multipliers.clone()),
        //     None,
        //     Some(immunities.clone()),
        //     Some(vulnerabilities.clone()),
        // )

        Ore {
            value,
            multipliers: multipliers.clone(),
            immunities: immunities.clone(),
            vulnerabilities: vulnerabilities.clone(),
            ..Default::default()
        }
    }

    pub fn spawn_ores(&self, seconds: u16) -> Ores {
        let amount = (seconds as f32 * self.drop_rate).floor();
        let group: Vec<Ore> = (0..amount as u16).map(|_| self.spawn_ore()).collect();
        let ores = Ores { ores: group };
        ores
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
