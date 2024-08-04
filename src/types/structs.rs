#![allow(dead_code)]
#![allow(unused)]

use super::enums::{
    Modifiers, Multipliers, Tags, Vulnerabilities, MINE_DROP_RATES, MINE_RATES, RATES_FROM_STANDARD,
};

#[derive(Debug, Clone)]
pub struct Mine {
    drop_rate: f32,
    value: f32,
    adds: Vec<Multipliers>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    adds_immunities: Vec<Tags>,
    modifiers: Modifiers,
}

impl Mine {
    pub fn new(
        drop_rate: f32,
        value: f32,
        modifiers: Modifiers,
        adds: Option<Vec<Multipliers>>,
        adds_vulnerabilities: Option<Vec<Vulnerabilities>>,
        adds_immunities: Option<Vec<Tags>>,
    ) -> Self {
        Self {
            drop_rate,
            value,
            modifiers,
            adds: adds.unwrap_or_default(),
            adds_vulnerabilities: adds_vulnerabilities.unwrap_or_default(),
            adds_immunities: adds_immunities.unwrap_or_default(),
        }
    }

    pub fn spawn_ore(&self) -> Ore {
        let value: f32 = self.value;
        let multipliers = self.adds.clone();
        let immunities = &self.adds_immunities;
        let vulnerabilities = &self.adds_vulnerabilities;

        Ore::new(
            value,
            Some(multipliers.clone()),
            None,
            Some(immunities.clone()),
            Some(vulnerabilities.clone()),
        )
    }

    pub fn spawn_ores(&self, seconds: u16) -> Vec<Ore> {
        let mut ores = Vec::new();
        let amount = (seconds as f32 * self.drop_rate).floor();
        for i in 0..amount as u16 {
            ores.push(self.spawn_ore());
        }
        ores
    }
}

#[derive(Debug, Clone)]
pub struct Upgrader {
    multiplier: f32,
    modifiers: Modifiers,
    adds: Vec<Tags>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    removes: Vec<Tags>,
    destroys: Vec<Tags>,
}

impl Upgrader {
    pub fn new(
        multiplier: f32,
        modifiers: Modifiers,
        adds: Option<Vec<Tags>>,
        adds_vulnerabilities: Option<Vec<Vulnerabilities>>,
        removes: Option<Vec<Tags>>,
        destroys: Option<Vec<Tags>>,
    ) -> Self {
        Self {
            multiplier,
            modifiers,
            adds: adds.unwrap_or_default(),
            adds_vulnerabilities: adds_vulnerabilities.unwrap_or_default(),
            removes: removes.unwrap_or_default(),
            destroys: destroys.unwrap_or_default(),
        }
    }

    fn upgrade(&self, ore: &mut Ore) {
        todo!("Upgrade ore");
    }
}

#[derive(Debug, Clone)]
pub struct Furnace {
    multiplier: f32,
    modifiers: Modifiers,
    multiplies: bool,
    extra: Vec<Multipliers>,
    refuses: Vec<Tags>,
}
impl Furnace {
    pub fn new(
        multiplier: f32,
        modifiers: Modifiers,
        multiplies: bool,
        extra: Option<Vec<Multipliers>>,
        refuses: Option<Vec<Tags>>,
    ) -> Self {
        Self {
            multiplier,
            modifiers,
            multiplies,
            extra: extra.unwrap_or_default(),
            refuses: refuses.unwrap_or_default(),
        }
    }
}
impl Modify for Furnace {
    fn modify(&mut self, modifier: Modifiers) {
        println!("before match: {:?}", self);
        self.to_standard();
        println!("first match: {:?}", self);

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
            _ => (),
        }

        println!("second match: {:?}", self);
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

        self.modifiers = Modifiers::Standard;
    }

    fn modify_from_standard(&mut self, to_modifier: &Modifiers) {
        let rates = match RATES_FROM_STANDARD.get(to_modifier) {
            Some(rate) => rate,
            None => return, // Exit early if `to_modifier` does not match any known variant
        };

        self.multiplier = (rates[0] * self.multiplier) - rates[1];
        self.modifiers = to_modifier.clone();
    }
}
#[derive(Debug, Clone)]
pub struct Ore {
    value: f32,
    multipliers: Vec<Multipliers>,
    tags: Vec<Tags>,
    immunities: Vec<Tags>,
    vulnerabilities: Vec<Vulnerabilities>,
}

impl Ore {
    pub fn new(
        value: f32,
        multipliers: Option<Vec<Multipliers>>,
        tags: Option<Vec<Tags>>,
        immunities: Option<Vec<Tags>>,
        vulnerabilities: Option<Vec<Vulnerabilities>>,
    ) -> Self {
        Self {
            value,
            multipliers: multipliers.unwrap_or_default(),
            tags: tags.unwrap_or_default(),
            immunities: immunities.unwrap_or_default(),
            vulnerabilities: vulnerabilities.unwrap_or_default(),
        }
    }
}
pub trait Modify {
    fn modify(&mut self, modifier: Modifiers);
}

trait ModifyStandard {
    fn to_standard(&mut self);
    fn modify_from_standard(&mut self, to_modifier: &Modifiers);
}
trait ModifyMine {
    fn to_standard(&mut self);
    fn to_standard_drop_rate(&mut self);
    fn modify_from_standard(&mut self, to_modifier: &Modifiers);
    fn apply_standard_drop_rate(&mut self, to_modifier: &Modifiers);
}

impl Modify for Mine {
    fn modify(&mut self, modifier: Modifiers) {
        println!("before match:{:?}", self);

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

        println!("first match:{:?}", self);
        assert!(self.modifiers == Modifiers::Standard);

        println!("first match:{:?}", self);
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

        println!("second match:{:?}", self);
    }
}

impl ModifyMine for Mine {
    fn to_standard(&mut self) {
        let rates = match self.modifiers {
            Modifiers::Golden | Modifiers::OverclockedGolden => MINE_RATES.get(&Modifiers::Golden),
            Modifiers::Negative | Modifiers::OverclockedNegative => {
                MINE_RATES.get(&Modifiers::Negative)
            }
            Modifiers::NegativeGolden | Modifiers::OverclockedNegativeGolden => {
                MINE_RATES.get(&Modifiers::NegativeGolden)
            }
            _ => None,
        };

        if let Some(rate) = rates {
            self.value /= rate;
        }

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
                    self.value *= golden_rate;
                    self.modifiers = to_modifier.clone();
                }
            }
            Modifiers::Negative | Modifiers::OverclockedNegative => {
                if let Some(negative_rate) = MINE_RATES.get(&Modifiers::Negative) {
                    self.value *= negative_rate;
                    self.modifiers = to_modifier.clone();
                }
            }
            Modifiers::NegativeGolden | Modifiers::OverclockedNegativeGolden => {
                if let Some(negative_golden_rate) = MINE_RATES.get(&Modifiers::NegativeGolden) {
                    self.value *= negative_golden_rate;
                    self.modifiers = to_modifier.clone();
                }
            }
        }
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

impl Modify for Upgrader {
    fn modify(&mut self, modifier: Modifiers) {
        println!("before match:{:?}", self);
        self.to_standard();
        println!("{:?}", self.modifiers);
        assert!(self.modifiers == Modifiers::Standard);
        println!("first match:{:?}", self);
        self.modify_from_standard(&modifier);
        println!("second match:{:?}", self);
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

        self.modifiers = Modifiers::Standard;
    }

    fn modify_from_standard(&mut self, to_modifier: &Modifiers) {
        let rates = match RATES_FROM_STANDARD.get(to_modifier) {
            Some(rate) => rate,
            None => return, // Exit early if `to_modifier` does not match any known variant
        };

        self.multiplier = (rates[0] * self.multiplier) - rates[1];
        self.modifiers = to_modifier.clone();
    }
}
