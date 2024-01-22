use rand::prelude::*;

#[derive(Debug)]
struct Character {
    name: String,
    hp: u16,
    ac: u16,
}

impl Character {
    fn hit(&mut self, hit_roll: u16, damage: u16) {
        if hit_roll > self.ac.into() {
            println!("{} was hit for {} hp", self.name, damage);
            // NOTE! Can cause integer overflow if not handled
            if self.hp > damage {
                self.hp -= damage
            } else {
                self.hp = 0
            }
        } else {
            println!("Hit failed");
        }
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Debug)]
enum DiceType {
    D4,
    D6,
    D8,
    D10,
    D14,
    D20,
}

fn max_for_dice(dice: &DiceType) -> u16 {
    // NOTE: use enum name too because if not it can be detected as a variable
    // and will produce errors later
    match dice {
        DiceType::D4 => 4,
        DiceType::D6 => 6,
        DiceType::D8 => 8,
        DiceType::D10 => 10,
        DiceType::D14 => 14,
        DiceType::D20 => 20,
    }
}

fn roll_dice(dice: &DiceType) -> u16 {
    let mut rng = thread_rng();
    let max_num = max_for_dice(&dice);
    rng.gen_range(1..=max_num)
}

fn print_roll(roll: u16, dice: &DiceType) {
    println!("\tYou rolled a {roll}");
    match (roll, dice) {
        (1, DiceType::D20) => println!("\tCritical failure 😞"),
        (20, DiceType::D20)  => println!("\tCritical Success! 🙌"),
        (r, dice) => if r == max_for_dice(dice) {println!("\t nice roll!")},
    };
}

fn do_roll(dice: DiceType) {
    let roll = roll_dice(&dice);
    println!("Rolling a {:?}", dice);
    print_roll(roll, &dice);
}


fn main() {
    use DiceType::*;
    do_roll(D4);
    do_roll(D6);
    do_roll(D8);
    do_roll(D10);
    do_roll(D14);
    do_roll(D20);
    
    let mut cornelius = Character {
        name: String::from("Cornelius"),
        hp: 20,
        ac: 12,
    };
    println!("We have {:?}", cornelius);
    cornelius.hit(roll_dice(&D20) + 7, roll_dice(&D8) + 2);
    cornelius.hit(roll_dice(&D20) + 7, roll_dice(&D8) + 2);
    cornelius.hit(roll_dice(&D20) + 7, roll_dice(&D8) + 2);
    cornelius.hit(roll_dice(&D20) + 7, roll_dice(&D8) + 2);
    cornelius.hit(roll_dice(&D20) + 7, roll_dice(&D8) + 2);
    cornelius.hit(roll_dice(&D20) + 7, roll_dice(&D8) + 2);

    println!("\tAfter those hits {} has {} hp left", cornelius.name, cornelius.hp);
    println!("Did {} ended up alive? {}", cornelius.name, cornelius.is_alive());
}
