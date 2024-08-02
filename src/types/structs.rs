#![allow(dead_code)]
#![allow(unused)]

use super::enums::{Tags, Vulnerabilities, Multipliers};


#[derive(Debug, Clone)]
pub struct Mine {
    drop_rate: f32,
    value: u16,
    adds: Vec<Multipliers>,
    adds_vulnerabilities: Vec<Vulnerabilities>,
    adds_immunities: Vec<Tags>,
}


impl Mine {
    pub fn new(
        drop_rate: f32,
        value: u16,
        adds: Option<Vec<Multipliers>>,              // Make the arguments optional
        adds_vulnerabilities: Option<Vec<Vulnerabilities>>,
        adds_immunities: Option<Vec<Tags>>,
    ) -> Self {
        Self {
            drop_rate,
            value,
            adds: adds.unwrap_or_default(),                  // Provide default values if None
            adds_vulnerabilities: adds_vulnerabilities.unwrap_or_default(),
            adds_immunities: adds_immunities.unwrap_or_default(),
        }
    }
    
    pub fn spawn_ore(&self) -> Ore {
        let value = self.value;
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
    value: u16,
    multipliers: Vec<Multipliers>,
    tags: Vec<Tags>,
    immunities: Vec<Tags>,
    vulnerabilities: Vec<Vulnerabilities>,
}

impl Ore {
    pub fn new(value: u16,
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