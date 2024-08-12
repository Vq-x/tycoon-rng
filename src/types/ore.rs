use super::enums::{Multipliers, Tags, Vulnerabilities};

#[derive(Debug, Clone)]
pub struct Ore {
    pub value: f32,
    pub multipliers: Vec<Multipliers>,
    pub tags: Vec<Tags>,
    pub immunities: Vec<Tags>,
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
    pub fn multiply_by(&mut self, multiplier: f32) {
        self.value *= multiplier;
    }

    pub fn add_tag(&mut self, tag: Tags) {
        self.tags.push(tag);
    }

    pub fn remove_tag(&mut self, tag: Tags) {
        self.tags.retain(|x| x != &tag);
    }

    pub fn add_immunity(&mut self, immunity: Tags) {
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
