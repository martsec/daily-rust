use std::sync::Arc;
use crate::dices::DiceType;

#[derive(Debug)]
pub struct Weapon<'a> {
    pub name : Arc<&'a str>,
    pub damage_dice : DiceType,
}
