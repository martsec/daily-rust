use crate::dices::DiceType;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Weapon<'a> {
    pub name: Arc<&'a str>,
    pub damage_dice: DiceType,
}
