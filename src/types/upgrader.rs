use std::mem;

use serde::{Deserialize, Serialize};

use super::{
    enums::{
        Modifiers, Multipliers, Tags, UpgraderTypes, Vulnerabilities, RARITY_MULTIPLIERS,
        RATES_FROM_STANDARD,
    },
    ore::Ore,
    utils::{Modify, ModifyStandard},
};
/*
NOTES:
Upgraders like the Forceful Blizzard which replaces Fire with Wet doesn't apply the 2x wet multiplier from the guttation dripper
the 2x multiplier is only applied when the tag is explicitly added
For example, the Royal CastleKeep which has 2x fire multiplier, but also is fire immune, applies the 2x fire multiplier when
upgraded by the Gates of Calamity which adds the fire tag. It multiplies it by 2x, but doesn't add the tag(cause of the immunity)

The Overtime Tag automatically adds the 3x multiplier after 4.0s, doesn't need a upgrader, the tag name is
time, if the ore is 1.9x time it will get upgraded by upgraders that have the overtime effect, adding time.

A ore that has 1.6x aired doesn't get the 1.6x multiplier from the Wind Tunnel upgrader if that ore is also on fire.


TODO:
    - make timed tags work
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Upgrader {
    pub multiplier: f32,
    pub modifiers: Modifiers,
    pub rarity: u64,
    pub effects: Vec<UpgraderTypes>,
    // pub adds: Vec<Tags>,
    // pub adds_vulnerabilities: Vec<Vulnerabilities>,
    // pub removes: Vec<Tags>,
    // pub destroys: Vec<Tags>,
}

impl Upgrader {
    pub fn upgrade(&self, ore: &mut Ore) {
        // multiply ore by multiplier
        let mut multiplier = self.multiplier;

        for effect in self.effects.iter() {
            match effect {
                // adds Wet 1x if Fire 2x if None
                UpgraderTypes::AddsIfThen(adder, count, tag_if, count2) => {
                    // if contains tag fire
                    let times_to_add = if ore
                        .tags
                        .iter()
                        .any(|tag| mem::discriminant(tag) == mem::discriminant(tag_if))
                    {
                        count
                    } else {
                        count2
                    };

                    for _ in 0..*times_to_add {
                        ore.add_tag(adder.clone());
                    }
                }
                // adds Ice if Wet
                UpgraderTypes::AddsIf(adder, tag) => {
                    if ore
                        .tags
                        .iter()
                        .any(|t| mem::discriminant(t) == mem::discriminant(tag))
                    {
                        ore.add_tag(adder.clone());
                    }
                }
                // Adds Tag
                UpgraderTypes::Adds(tag, amount) => {
                    for _ in 0..*amount {
                        ore.add_tag(tag.clone());
                    }
                }
                // Adds Wet If not on Fire
                UpgraderTypes::AddsIfNot(adds, not_tag) => {
                    if !ore
                        .tags
                        .iter()
                        .any(|t| mem::discriminant(t) == mem::discriminant(not_tag))
                    {
                        ore.add_tag(adds.clone());
                    }
                }
                // replaces Fire with Wet
                UpgraderTypes::Replaces(replacing, replace) => {
                    // if ore.tags.contains(replacing) {
                    //     ore.add_tag(replace.clone());
                    //     ore.remove_tag(replacing.clone());
                    // }
                    if ore
                        .tags
                        .iter()
                        .any(|t| mem::discriminant(t) == mem::discriminant(replacing))
                    {
                        ore.add_tag(replace.clone());
                        ore.remove_tag(replacing.clone());
                    }
                }
                // extra 1.2x for each fire tag
                UpgraderTypes::ExtraForEach(num, tag) => {
                    for iter_tag in ore.tags.iter() {
                        if iter_tag == tag {
                            multiplier *= num;
                        }
                    }
                }
                // Multiplies value by 10x if glitch
                UpgraderTypes::MultiplyIf(num, tag) => {
                    if ore.tags.contains(tag) {
                        multiplier *= num;
                    }
                }

                UpgraderTypes::MultiplyIfGrouped(num, tags) => {
                    if tags.iter().any(|tag| ore.tags.contains(tag)) {
                        multiplier *= num;
                    }
                }
                // extra upgrade logarithmically based on ore value
                UpgraderTypes::ExtraLogarithmic => {
                    let eq = log_base(ore.value + 1.0, 1000.0) / 4.0;
                    // ore.multiply_by(eq);
                    multiplier *= eq as f32;
                }
                UpgraderTypes::Removes(tag) => {
                    ore.remove_tag(tag.clone());
                }
                UpgraderTypes::Overtime(mult, seconds) => {
                    ore.add_tag(Tags::Time(*seconds as u8, *mult));
                }
                UpgraderTypes::AddsImmunity(immunity) => {
                    ore.add_immunity(immunity.clone());
                }
                UpgraderTypes::AddsVulnerability(vulnerability) => {
                    ore.add_vulnerability(vulnerability.clone());
                }
                UpgraderTypes::Destroys(tag) => {
                    if ore
                        .tags
                        .iter()
                        .any(|t| mem::discriminant(t) == mem::discriminant(tag))
                    {
                        ore.destroy();
                    }
                }
                UpgraderTypes::DestroysVulnerability(vulnerability) => {
                    if ore
                        .vulnerabilities
                        .iter()
                        .any(|v| mem::discriminant(v) == mem::discriminant(vulnerability))
                    {
                        ore.destroy();
                    }
                }
            }
        }

        /*
            check for ore multipliers with upgraders
            current logic checks that if a upgrader adds a tag to the ore, than the upgrader is of that type tag.
            When there is a ore with a wet multiplier for example and the upgrader adds a wet tag to the ore
            the upgrader will automatically apply the multiplier once.
            This logic only applies to upgrader types Adds, AddsIf, AddsIfThen.
            TODO:
                - make sure the upgrader types are sufficient to cover all cases
        */
        for ore_mult in ore.multipliers.iter() {
            if self.effects.iter().any(|effect| match effect {
                UpgraderTypes::Adds(tag, _) if *tag == ore_mult.get_tag() => true,
                UpgraderTypes::AddsIf(tag, _) if *tag == ore_mult.get_tag() => true,
                UpgraderTypes::AddsIfThen(tag, _, _, _) if *tag == ore_mult.get_tag() => true,
                _ => false,
            }) {
                match ore_mult {
                    Multipliers::Fire(mult)
                    | Multipliers::Acid(mult)
                    | Multipliers::Wet(mult)
                    | Multipliers::Putrid(mult)
                    | Multipliers::Fueled(mult)
                    | Multipliers::Magnetic(mult)
                    | Multipliers::Aired(mult)
                    | Multipliers::Time(mult)
                    | Multipliers::Polished(mult)
                    | Multipliers::Perfumed(mult)
                    | Multipliers::Glitch(mult)
                    | Multipliers::Vulnerable(mult) => {
                        multiplier *= mult;
                    }
                }
            }
        }
        ore.multiply_by(multiplier as f64);
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
            modifiers: Modifiers::Standard,
            rarity: 1000,
            effects: vec![],
            // adds: vec![],
            // adds_vulnerabilities: vec![],
            // removes: vec![],
            // destroys: vec![],
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

fn log_base(x: f64, base: f64) -> f64 {
    x.log10() / base.log10()
}
