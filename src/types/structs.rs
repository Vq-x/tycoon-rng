#![allow(dead_code)]
#![allow(unused)]

use super::enums::{Modifiers, Multipliers, Tags, Vulnerabilities};


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


pub struct Upgrader {
    multiplier: u16,
    adds: Vec<Tags>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    removes: Vec<Tags>,
    destroys: Vec<Tags>,
}

impl Upgrader {
    pub fn new(multiplier:u16, adds: Option<Vec<Tags>>, adds_vulnerabilities: Option<Vec<Vulnerabilities>>, removes: Option<Vec<Tags>>, destroys: Option<Vec<Tags>>) -> Self {
        Self {
            multiplier,
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

struct Furnace {
    multiplier: u16,
    extra: Vec<Multipliers>,
    refuses: Vec<Tags>,
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

    fn to_standard(&mut self){
        match self.modifiers {
            Modifiers::Golden => {
                self.value /= 3.5;
            }
            Modifiers::Negative => {
                self.value /=7.5;
            }
            Modifiers::OverclockedGolden => {
                self.value /= 3.5;
            }
            Modifiers::OverclockedNegative => {
                self.value /= 7.5;
            }
            Modifiers::NegativeGolden => {
                self.value /= 26.25;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.value /= 26.25;
            }
            _ => ()
            // Modifiers::OverclockedNegative => self.value = (self.value + 15.05)/16.05,
            // Modifiers::NegativeGolden => self.value = (self.value + 25.25)/26.25,
            // Modifiers::OverclockedNegativeGolden => self.value = (self.value + 55.175)/56.175,
        }
        self.modifiers = Modifiers::Standard;
    }
    
    fn to_standard_drop_rate(&mut self){
        match self.modifiers {
            Modifiers::Overclocked => {
                self.drop_rate = (self.drop_rate-0.025)/2.125;
            }
            Modifiers::OverclockedGolden => {
                self.drop_rate = (self.drop_rate-0.025)/2.125;
            }
            Modifiers::OverclockedNegative => {
                self.drop_rate = (self.drop_rate-0.025)/2.125;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.drop_rate = (self.drop_rate-0.025)/2.125;
            }
            _ => (),
        }
    }
    
    fn from_standard(&mut self, to_modifier: &Modifiers) {
        match to_modifier {
            Modifiers::Standard => {
                self.modifiers = Modifiers::Standard;
            },
            Modifiers::Overclocked => {
                self.modifiers = Modifiers::Overclocked;
            }
            Modifiers::Golden => {
                self.value *= 3.5;
                self.modifiers = Modifiers::Golden;
            }
            Modifiers::Negative => {
                self.value *= 7.5;
                self.modifiers = Modifiers::Negative;
            }
            Modifiers::OverclockedGolden => {
                self.value *= 3.5;
                self.modifiers = Modifiers::OverclockedGolden;
            }
            Modifiers::OverclockedNegative => {
                self.value *= 7.5;
                self.modifiers = Modifiers::OverclockedNegative;
            }
            Modifiers::NegativeGolden => {
                self.value *= 26.25;
                self.modifiers = Modifiers::NegativeGolden;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.value *= 26.25;
                self.modifiers = Modifiers::OverclockedNegativeGolden;
            }
            // Modifiers::Golden => (value * 3.5) - 2.5,
            // Modifiers::Negative => (value * 7.5) - 6.5,
            // Modifiers::OverclockedGolden => (value * 7.49) - 6.49,
            // Modifiers::OverclockedNegative => (value * 16.05) - 15.05,
            // Modifiers::NegativeGolden => (value * 26.25) - 25.25,
            // Modifiers::OverclockedNegativeGolden => (value * 56.175) - 55.175,
        }
    }
    
    fn from_standard_drop_rate(&mut self, to_modifier: &Modifiers) {
        match to_modifier {
            Modifiers::Overclocked => {
                self.drop_rate = (2.125*self.drop_rate)+0.025;
                self.modifiers = Modifiers::Overclocked;
            }
            Modifiers::OverclockedGolden => {
                self.drop_rate = (2.125*self.drop_rate)+0.025;
                self.modifiers = Modifiers::OverclockedGolden;
            }
            Modifiers::OverclockedNegative => {
                self.drop_rate = (2.125*self.drop_rate)+0.025;
                self.modifiers = Modifiers::OverclockedNegative;
            }
            Modifiers::OverclockedNegativeGolden => {
                self.drop_rate = (2.125*self.drop_rate)+0.025;
                self.modifiers = Modifiers::OverclockedNegativeGolden;
            }
            // Modifiers::OverclockedNegative => (2.125*value)+0.025,
            // Modifiers::OverclockedNegativeGolden => (2.125*value)+0.025,
            _ => (),
        }
    }
    
}