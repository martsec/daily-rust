use std::sync::Arc;
use crate::dices::DiceType;
use crate::weapons::handheld::Weapon;

#[derive(Debug)]
pub struct Character<'a> {
    pub(crate) name: String,
    hp: u16,
    ac: u16,
    weapon: Weapon<'a>
}

impl <'a>Default for Character<'a> {
    fn default() -> Self {
        let weapon = Weapon {
            name: Arc::new("Empty hand"),
            damage_dice: DiceType::D4,
        };
        Self {
            name: "Noname".to_string(),
            hp: 5,
            ac: 10,
            weapon,
        }
    }
}

impl <'a>Character<'a> {
    pub fn new(name: String, hp: u16, ac: u16, weapon: Weapon<'a>) -> Self {
        Self {
            name, hp, ac, weapon
        }
    }

    pub fn hit(&mut self, other: &mut Self) {
        let attack_roll = DiceType::D20.roll();
        if attack_roll == 20 {
            println!("\t critical hit!!!");
            other.take_damage(2 * self.weapon.damage_dice.roll())
        } else if attack_roll >= other.ac.into() {
            other.take_damage(self.weapon.damage_dice.roll())
        } else {
            println!("Attack from {} missed.", self.name)
        }
    }

    fn take_damage(&mut self, damage: u16) {
        // NOTE! Can cause integer overflow if not handled
        if self.hp > damage {
            println!("{} was hit for {} hp", self.name, damage);
            self.hp -= damage
        } else {
            println!("{} reached 0 hp", self.name);
            self.hp = 0
        }
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }
}
