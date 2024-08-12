use super::{
    enums::{
        Modifiers, Multipliers, Tags, Vulnerabilities, RARITY_MULTIPLIERS, RATES_FROM_STANDARD,
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
    pub adds: Vec<Tags>,
    pub adds_vulnerabilities: Vec<Vulnerabilities>,
    pub removes: Vec<Tags>,
    pub destroys: Vec<Tags>,
}

impl Upgrader {
    pub fn upgrade(&self, ore: &mut Ore) {
        // multiply ore by multiplier
        ore.multiply_by(self.multiplier);

        // add tags to ore if any of the adds tags are present
        self.adds.iter().for_each(|tag| ore.add_tag(tag.clone()));

        // add vulnerabilities to ore if any of the adds_vulnerabilities tags are present
        self.adds_vulnerabilities
            .iter()
            .for_each(|vulnerability| ore.add_vulnerability(vulnerability.clone()));

        // remove tags from ore if any of the removes tags are present
        self.removes
            .iter()
            .for_each(|tag| ore.remove_tag(tag.clone()));

        // destroy ore if any of the destroys tags are present
        self.destroys.iter().for_each(|tag| {
            if ore.tags.contains(tag) {
                ore.destroy();
            }
        });
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
