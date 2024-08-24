use std::{any::Any, vec};

use super::{
    enums::{Immunities, Multipliers, Tags, Vulnerabilities},
    upgrader::Upgrader,
};
/*
    TODO:
        - add a struct for ores in a vector
        - implement functions that combine two vectors of ores
        - implement functions to upgrade all ores in a vector with given upgrader
        -
*/
#[derive(Debug, Clone)]
pub struct Ore {
    pub value: f64,
    pub multipliers: Vec<Multipliers>,
    pub tags: Vec<Tags>,
    pub immunities: Vec<Immunities>,
    pub vulnerabilities: Vec<Vulnerabilities>,
    pub destroyed: bool,
}

impl Default for Ore {
    fn default() -> Self {
        Self {
            value: 1.0,
            multipliers: vec![],
            tags: vec![],
            immunities: vec![],
            vulnerabilities: vec![],
            destroyed: false,
        }
    }
}

impl Ore {
    pub fn multiply_by(&mut self, multiplier: f64) {
        self.value *= multiplier;
    }

    pub fn add_tag(&mut self, tag: Tags) {
        if tag
            .get_immunity()
            .is_some_and(|immunity| self.immunities.contains(&immunity))
        {
            return;
        }

        if tag
            .get_vulnerability()
            .is_some_and(|vulnerability| self.vulnerabilities.contains(&vulnerability))
        {
            self.destroy();
        }
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: Tags) {
        if let Some(index) = self
            .tags
            .iter()
            .position(|x| std::mem::discriminant(x) == std::mem::discriminant(&tag))
        {
            self.tags.remove(index);
        }
    }

    pub fn remove_tags(&mut self, tag: Tags) {
        self.tags.retain(|x| x != &tag);
    }

    pub fn add_immunity(&mut self, immunity: Immunities) {
        self.immunities.push(immunity);
    }

    pub fn add_vulnerability(&mut self, vulnerability: Vulnerabilities) {
        self.vulnerabilities.push(vulnerability);
    }

    pub fn add_multiplier(&mut self, multiplier: Multipliers) {
        self.multipliers.push(multiplier);
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
    }
}

#[derive(Debug, Clone)]
pub struct Ores {
    pub ores: Vec<Ore>,
}
impl Ores {
    pub fn upgrade(&mut self, upgrader: &Upgrader) {
        self.ores.iter_mut().for_each(|ore| upgrader.upgrade(ore));
    }
    pub fn combine(&mut self, other: &mut Ores) {
        self.ores.append(&mut other.ores);
    }
    pub fn add(&mut self, other: Ore) {
        self.ores.push(other);
    }
}
