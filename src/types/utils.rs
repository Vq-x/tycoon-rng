use super::enums::Modifiers;

pub trait Modify {
    fn modify(&mut self, modifier: Modifiers);
}

pub trait ModifyStandard {
    fn to_standard(&mut self);
    fn modify_from_standard(&mut self, to_modifier: &Modifiers);
}
