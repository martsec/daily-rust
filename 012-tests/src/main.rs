use crate::character::Character;
use crate::dices::do_roll;
use crate::dices::DiceType;
use crate::weapons::handheld::Weapon;
use rand::prelude::*;
use std::sync::Arc;

mod character;
mod dices;
mod weapons;

fn main() {
    use DiceType::*;
    do_roll(D20);

    let mut cornelius = Character::new(
        String::from("Cornelius"),
        20,
        12,
        Weapon {
            name: Arc::new("Gladius"),
            damage_dice: D10,
        },
    );
    let mut barca = Character::new(
        String::from("Barca"),
        25,
        13,
        Weapon {
            name: Arc::new("ShortSword"),
            damage_dice: D8,
        },
    );

    println!("{:?}", cornelius);
    println!("{:?}", barca);

    let mut num_rounds = 0;
    while cornelius.is_alive() && barca.is_alive() {
        num_rounds += 1;
        println!("====== A new round starts! ======");
        let chosen = rand::thread_rng().gen_range(1..=2);
        match chosen {
            1 => cornelius.hit(&mut barca),
            2 => barca.hit(&mut cornelius),
            _ => {}
        };
        //println!("{:?}", cornelius);
        //println!("{:?}", barca);
    }
    let winner = if cornelius.is_alive() {
        cornelius
    } else {
        barca
    };
    println!(
        "\n\nAfter {} rounds the winner is {}!",
        num_rounds, winner.name
    );

    let new_char = Character::new(
        "myname".to_string(),
        10,
        15,
        Weapon {
            name: Arc::new("Fist"),
            damage_dice: D20,
        },
    );
}
