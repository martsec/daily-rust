use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
pub enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D14,
    D20,
}

impl DiceType {
    fn max_value(&self) -> u16 {
        match self {
            DiceType::D4 => 4,
            DiceType::D6 => 6,
            DiceType::D8 => 8,
            DiceType::D10 => 10,
            DiceType::D14 => 14,
            DiceType::D20 => 20,
        }
    }

    pub(crate) fn roll(&self) -> u16 {
        let mut rng = thread_rng();
        let max_num = self.max_value();
        rng.gen_range(1..=max_num)
    }
}

fn print_roll(roll: u16, dice: &DiceType) {
    println!("\tYou rolled a {roll}");
    match (roll, dice) {
        (1, DiceType::D20) => println!("\tCritical failure 😞"),
        (20, DiceType::D20) => println!("\tCritical Success! 🙌"),
        (r, dice) => {
            if r == dice.max_value() {
                println!("\t nice roll!")
            }
        }
    };
}

pub fn do_roll(dice: DiceType) -> u16 {
    let roll = dice.roll();
    println!("Rolling a {:?}", dice);
    print_roll(roll, &dice);
    roll
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roll_cannot_be_bigger_than_max() {
        for dice in DiceType::iter() {
            for _ in 0..100 {
                assert!(
                    dice.roll() <= dice.max_value(),
                    "Roll should not be bigger than dice value"
                );
            }
        }
    }
}
