pub mod character;
pub mod dices;
pub mod weapons;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::character::Character;
    use crate::dices::DiceType::*;
    use crate::weapons::handheld::Weapon;

    use std::sync::Arc;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn character_cannot_have_negative_hp() {
        let weapon: Weapon = Weapon {
            name: Arc::new("W"),
            damage_dice: D20,
        };
        let mut c = Character::new(String::from("C"), 20, 0, weapon.clone());
        let c2 = Character::new("C2".to_string(), 10, 0, weapon.clone());
        // When
        for _ in 0..30 {
            c2.hit(&mut c);
        }
        // Then
        assert_eq!(c.hp(), 0, "After a lot of hits, Character stays at 0");
    }

    #[test]
    fn at_zero_hp_char_not_alive() {
        let weapon: Weapon = Weapon {
            name: Arc::new("W"),
            damage_dice: D20,
        };
        let mut c = Character::new(String::from("C"), 20, 0, weapon.clone());
        let c2 = Character::new("C2".to_string(), 10, 0, weapon.clone());
        // When
        for _ in 0..30 {
            match c.hp() {
                // Then
                0 => assert!(!c.is_alive(), "Char with 0 hp should not be alive"),
                _ => assert!(c.is_alive(), "Char with >0 hp should be alive."),
            }
            c2.hit(&mut c);
        }
    }
}
