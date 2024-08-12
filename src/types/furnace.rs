use super::{
    enums::{Modifiers, Multipliers, Tags, RARITY_MULTIPLIERS, RATES_FROM_STANDARD},
    ore::Ore,
    utils::{Modify, ModifyStandard},
};

#[derive(Debug, Clone)]
pub struct Furnace {
    pub multiplier: f32,
    pub modifiers: Modifiers,
    pub rarity: u64,
    pub multiplies: bool,
    pub extra: Vec<Multipliers>,
    pub refuses: Vec<Tags>,
}

impl Default for Furnace {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            modifiers: Modifiers::Standard,
            rarity: 1000,
            multiplies: false,
            extra: vec![],
            refuses: vec![],
        }
    }
}
impl Furnace {
    // pub fn new(
    //     multiplier: f32,
    //     modifiers: Modifiers,
    //     rarity: u64,
    //     multiplies: bool,
    //     extra: Option<Vec<Multipliers>>,
    //     refuses: Option<Vec<Tags>>,
    // ) -> Self {
    //     Self {
    //         multiplier,
    //         modifiers,
    //         rarity,
    //         multiplies,
    //         extra: extra.unwrap_or_default(),
    //         refuses: refuses.unwrap_or_default(),
    //     }
    // }
    pub fn process_ores(&self, ores: &mut Vec<Ore>) {
        ores.iter_mut()
            .for_each(|o: &mut Ore| o.multiply_by(self.multiplier))
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
