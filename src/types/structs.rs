#![allow(dead_code)]
#![allow(unused)]

use super::enums::{Modifiers, Multipliers, Tags, Vulnerabilities, MINE_RATES, MINE_DROP_RATES, RATES_FROM_STANDARD};


#[derive(Debug, Clone)]
pub struct Mine {
    drop_rate: f32,
    value: f32,
    adds: Vec<Multipliers>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    adds_immunities: Vec<Tags>,
    modifiers: Modifiers
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

        Ore::new(value,
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
    pub fn new(multiplier:f32, modifiers: Modifiers, adds: Option<Vec<Tags>>, adds_vulnerabilities: Option<Vec<Vulnerabilities>>, removes: Option<Vec<Tags>>, destroys: Option<Vec<Tags>>) -> Self {
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
    pub fn new(multiplier: f32, modifiers: Modifiers, multiplies: bool, extra: Option<Vec<Multipliers>>, refuses: Option<Vec<Tags>>) -> Self {
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
                self.from_standard(&modifier);
            }
            _ => (),
        }

        println!("second match: {:?}", self);
    }
}
impl ModifyStandard for Furnace {
    fn to_standard(&mut self){
        let overclocked_rate = RATES_FROM_STANDARD.get(&Modifiers::Overclocked).unwrap();
        let golden_rate = RATES_FROM_STANDARD.get(&Modifiers::Golden).unwrap();
        let negative_rate = RATES_FROM_STANDARD.get(&Modifiers::Negative).unwrap();
        let overclocked_golden_rate = RATES_FROM_STANDARD.get(&Modifiers::OverclockedGolden).unwrap();
        let overclocked_negative_rate = RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegative).unwrap();
        let negative_golden_rate = RATES_FROM_STANDARD.get(&Modifiers::NegativeGolden).unwrap();
        let overclocked_negative_golden_rate = RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegativeGolden).unwrap();
        match self.modifiers {
            Modifiers::Overclocked => {
                self.multiplier = (self.multiplier+overclocked_rate[1])/overclocked_rate[0];
            }
            Modifiers::Golden => {
                self.multiplier = (self.multiplier+golden_rate[1])/golden_rate[0];
            }
            Modifiers::Negative => {
                self.multiplier = (self.multiplier+negative_rate[1])/negative_rate[0];
            }
            Modifiers::OverclockedGolden => {
                self.multiplier = (self.multiplier+overclocked_golden_rate[1])/overclocked_golden_rate[0];
            }
            Modifiers::OverclockedNegative => {
                self.multiplier = (self.multiplier+overclocked_negative_rate[1])/overclocked_negative_rate[0];
            }
            Modifiers::NegativeGolden => {
                self.multiplier = (self.multiplier+negative_golden_rate[1])/negative_golden_rate[0];
            }
            Modifiers::OverclockedNegativeGolden => {
                self.multiplier = (self.multiplier+overclocked_negative_golden_rate[1])/overclocked_negative_golden_rate[0];
            }
            _ => (),
        }
        self.modifiers = Modifiers::Standard;
    }
    
    fn from_standard(&mut self, to_modifier: &Modifiers) {
        let rates = match to_modifier {
            Modifiers::Overclocked => RATES_FROM_STANDARD.get(&Modifiers::Overclocked).unwrap(),
            Modifiers::Golden => RATES_FROM_STANDARD.get(&Modifiers::Golden).unwrap(),
            Modifiers::Negative => RATES_FROM_STANDARD.get(&Modifiers::Negative).unwrap(),
            Modifiers::OverclockedGolden => RATES_FROM_STANDARD.get(&Modifiers::OverclockedGolden).unwrap(),
            Modifiers::OverclockedNegative => RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegative).unwrap(),
            Modifiers::NegativeGolden => RATES_FROM_STANDARD.get(&Modifiers::NegativeGolden).unwrap(),
            Modifiers::OverclockedNegativeGolden => RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegativeGolden).unwrap(),
            _ => return, // Exit early if `to_modifier` does not match any known variant
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
    pub fn new(value: f32,
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
    fn from_standard(&mut self, to_modifier: &Modifiers);
}
trait ModifyMine {
    fn to_standard(&mut self);
    fn to_standard_drop_rate(&mut self);
    fn from_standard(&mut self, to_modifier: &Modifiers);
    fn from_standard_drop_rate(&mut self, to_modifier: &Modifiers);
}

impl Modify for Mine {
    fn modify(&mut self, modifier: Modifiers) {
        println!("before match:{:?}", self);
        match self.modifiers {
            Modifiers::Overclocked => {
                self.to_standard_drop_rate();
            }
            Modifiers::OverclockedGolden => {
                self.to_standard_drop_rate();
                self.to_standard();
            }
            Modifiers::OverclockedNegative => {
                self.to_standard_drop_rate();
                self.to_standard();
            }
            Modifiers::NegativeGolden => {
                self.to_standard_drop_rate();
            }
            Modifiers::OverclockedNegativeGolden => {
                self.to_standard_drop_rate();
                self.to_standard();
            }
            _ => {
                self.to_standard();
            }
        }
        println!("first match:{:?}", self);
        assert!(self.modifiers == Modifiers::Standard);
        match modifier {
            Modifiers::Standard => {
                self.from_standard(&modifier);
            }
            Modifiers::Overclocked => {
                self.from_standard_drop_rate(&modifier);
            }
            Modifiers::Golden => {
                self.from_standard(&modifier);
            }
            Modifiers::Negative => {
                self.from_standard(&modifier);
            }
            Modifiers::OverclockedGolden => {
                self.from_standard_drop_rate(&modifier);
                self.from_standard(&modifier);
            },
            Modifiers::OverclockedNegative => {
                self.from_standard_drop_rate(&modifier);
                self.from_standard(&modifier);
            }
            Modifiers::NegativeGolden => {
                self.from_standard(&modifier);
            }
            Modifiers::OverclockedNegativeGolden => {
                self.from_standard_drop_rate(&modifier);
                self.from_standard(&modifier);
            }
        }
        println!("second match:{:?}", self);
    }
}

impl ModifyMine for Mine {
    fn to_standard(&mut self){
        let golden_rate = MINE_RATES.get(&Modifiers::Golden).unwrap();
        let negative_rate = MINE_RATES.get(&Modifiers::Negative).unwrap();
        let negative_golden_rate = MINE_RATES.get(&Modifiers::NegativeGolden).unwrap();
        match self.modifiers {
            Modifiers::Golden => {
                self.value /= golden_rate;
            }
            Modifiers::Negative => {
                self.value /= negative_rate;
            }
            Modifiers::OverclockedGolden => {
                self.value /= golden_rate;
            }
            Modifiers::OverclockedNegative => {
                self.value /= negative_rate;
            }
            Modifiers::NegativeGolden => {
                self.value /= negative_golden_rate;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.value /= negative_golden_rate;
            }
            _ => ()
            // Modifiers::OverclockedNegative => self.value = (self.value + 15.05)/16.05,
            // Modifiers::NegativeGolden => self.value = (self.value + 25.25)/26.25,
            // Modifiers::OverclockedNegativeGolden => self.value = (self.value + 55.175)/56.175,
        }
        self.modifiers = Modifiers::Standard;
    }
    
    fn to_standard_drop_rate(&mut self){
        let overclocked_rate = MINE_DROP_RATES.get(&Modifiers::Overclocked).unwrap();
        match self.modifiers {
            Modifiers::Overclocked => {
                self.drop_rate = (self.drop_rate-overclocked_rate[1])/overclocked_rate[0];
            }
            Modifiers::OverclockedGolden => {
                self.drop_rate = (self.drop_rate-overclocked_rate[1])/overclocked_rate[0];
            }
            Modifiers::OverclockedNegative => {
                self.drop_rate = (self.drop_rate-overclocked_rate[1])/overclocked_rate[0];
            }
            Modifiers::OverclockedNegativeGolden => {
                self.drop_rate = (self.drop_rate-overclocked_rate[1])/overclocked_rate[0];
            }
            _ => (),
        }
    }
    
    fn from_standard(&mut self, to_modifier: &Modifiers) {
        let golden_rate = MINE_RATES.get(&Modifiers::Golden).unwrap();
        let negative_rate = MINE_RATES.get(&Modifiers::Negative).unwrap();
        let negative_golden_rate = MINE_RATES.get(&Modifiers::NegativeGolden).unwrap();
        match to_modifier {
            Modifiers::Standard => {
                self.modifiers = Modifiers::Standard;
            },
            Modifiers::Overclocked => {
                self.modifiers = Modifiers::Overclocked;
            }
            Modifiers::Golden => {
                self.value *= golden_rate;
                self.modifiers = Modifiers::Golden;
            }
            Modifiers::Negative => {
                self.value *= negative_rate;
                self.modifiers = Modifiers::Negative;
            }
            Modifiers::OverclockedGolden => {
                self.value *= golden_rate;
                self.modifiers = Modifiers::OverclockedGolden;
            }
            Modifiers::OverclockedNegative => {
                self.value *= negative_rate;
                self.modifiers = Modifiers::OverclockedNegative;
            }
            Modifiers::NegativeGolden => {
                self.value *= negative_golden_rate;
                self.modifiers = Modifiers::NegativeGolden;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.value *= negative_golden_rate;
                self.modifiers = Modifiers::OverclockedNegativeGolden;
            }
        }
    }
    
    fn from_standard_drop_rate(&mut self, to_modifier: &Modifiers) {
        let overclocked_rate = MINE_DROP_RATES.get(&Modifiers::Overclocked).unwrap();
        match to_modifier {
            Modifiers::Overclocked => {
                self.drop_rate = (overclocked_rate[0]*self.drop_rate)+overclocked_rate[1];
                self.modifiers = Modifiers::Overclocked;
            }
            Modifiers::OverclockedGolden => {
                self.drop_rate = (overclocked_rate[0]*self.drop_rate)+overclocked_rate[1];
                self.modifiers = Modifiers::OverclockedGolden;
            }
            Modifiers::OverclockedNegative => {
                self.drop_rate = (overclocked_rate[0]*self.drop_rate)+overclocked_rate[1];
                self.modifiers = Modifiers::OverclockedNegative;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.drop_rate = (overclocked_rate[0]*self.drop_rate)+overclocked_rate[1];
                self.modifiers = Modifiers::OverclockedNegativeGolden;
            }
            _ => (),
        }
    }
}



impl Modify for Upgrader {
    fn modify(&mut self, modifier: Modifiers) {
        println!("before match:{:?}", self);
        self.to_standard();
        println!("{:?}",self.modifiers);
        assert!(self.modifiers == Modifiers::Standard);
        println!("first match:{:?}", self);
        match modifier {
            Modifiers::Standard => {
                self.from_standard(&modifier);
            }
            Modifiers::Overclocked => {
                self.from_standard(&modifier);
            }
            Modifiers::Golden => {
                self.from_standard(&modifier);
            }
            Modifiers::Negative => {
                self.from_standard(&modifier);
            }
            Modifiers::OverclockedGolden => {
                self.from_standard(&modifier);
            }
            Modifiers::OverclockedNegative => {
                self.from_standard(&modifier);
            }
            Modifiers::NegativeGolden => {
                self.from_standard(&modifier);
            }
            Modifiers::OverclockedNegativeGolden => {
                self.from_standard(&modifier);
            }
        }
        println!("second match:{:?}", self);
    }
}

impl ModifyStandard for Upgrader {
    fn to_standard(&mut self){
        let overclocked_rate = RATES_FROM_STANDARD.get(&Modifiers::Overclocked).unwrap();
        let golden_rate = RATES_FROM_STANDARD.get(&Modifiers::Golden).unwrap();
        let negative_rate = RATES_FROM_STANDARD.get(&Modifiers::Negative).unwrap();
        let overclocked_golden_rate = RATES_FROM_STANDARD.get(&Modifiers::OverclockedGolden).unwrap();
        let overclocked_negative_rate = RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegative).unwrap();
        let negative_golden_rate = RATES_FROM_STANDARD.get(&Modifiers::NegativeGolden).unwrap();
        let overclocked_negative_golden_rate = RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegativeGolden).unwrap();
        match self.modifiers {
            Modifiers::Overclocked => {
                self.multiplier = (self.multiplier+overclocked_rate[1])/overclocked_rate[0];
            }
            Modifiers::Golden => {
                self.multiplier = (self.multiplier+golden_rate[1])/golden_rate[0];
            }
            Modifiers::Negative => {
                self.multiplier = (self.multiplier+negative_rate[1])/negative_rate[0];
            }
            Modifiers::OverclockedGolden => {
                self.multiplier = (self.multiplier+overclocked_golden_rate[1])/overclocked_golden_rate[0];
            }
            Modifiers::OverclockedNegative => {
                self.multiplier = (self.multiplier+overclocked_negative_rate[1])/overclocked_negative_rate[0];
            }
            Modifiers::NegativeGolden => {
                self.multiplier = (self.multiplier+negative_golden_rate[1])/negative_golden_rate[0];
            }
            Modifiers::OverclockedNegativeGolden => {
                self.multiplier = (self.multiplier+overclocked_negative_golden_rate[1])/overclocked_negative_golden_rate[0];
            }
            _ => (),
        }
        self.modifiers = Modifiers::Standard;
    }
    
    fn from_standard(&mut self, to_modifier: &Modifiers) {
        let rates = match to_modifier {
            Modifiers::Overclocked => RATES_FROM_STANDARD.get(&Modifiers::Overclocked).unwrap(),
            Modifiers::Golden => RATES_FROM_STANDARD.get(&Modifiers::Golden).unwrap(),
            Modifiers::Negative => RATES_FROM_STANDARD.get(&Modifiers::Negative).unwrap(),
            Modifiers::OverclockedGolden => RATES_FROM_STANDARD.get(&Modifiers::OverclockedGolden).unwrap(),
            Modifiers::OverclockedNegative => RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegative).unwrap(),
            Modifiers::NegativeGolden => RATES_FROM_STANDARD.get(&Modifiers::NegativeGolden).unwrap(),
            Modifiers::OverclockedNegativeGolden => RATES_FROM_STANDARD.get(&Modifiers::OverclockedNegativeGolden).unwrap(),
            _ => return, // Exit early if `to_modifier` does not match any known variant
        };
    
        self.multiplier = (rates[0] * self.multiplier) - rates[1];
        self.modifiers = to_modifier.clone();
    }
}