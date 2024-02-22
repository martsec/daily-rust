mod containers;

pub use containers::{Deck, DeckEmptyError, Hand};

mod sealed {
    /// Trick to seal the card trait so downstream types cannot implement it
    pub trait Card {}
}

/// Sealed type representing a card
pub trait Card: sealed::Card {
    fn title(&self) -> &String;
    fn description(&self) -> &String;
}

trait BattleCard {
    fn strenght(&self) -> u32;
}

trait Attack: BattleCard {}

trait Defense: BattleCard {}

trait Effect: Card {
    fn effect(&self) -> CardEffect;
}

trait Special: Effect {}

trait Immediate: Effect {}

macro_rules! card_struct {
    ($struct_name:ident { $($field_name:ident : $field_type:ty),* }) => {
        #[derive(Debug, PartialEq)]
        pub struct $struct_name {
            $($field_name : $field_type,)*
            title: String,
            description: String,
        }

        impl sealed::Card for $struct_name {}
        impl Card for $struct_name {
            fn title(&self) -> &String {
                &self.title
            }
            fn description(&self) -> &String {
                &self.description
            }
        }
    }
}

/// This enum contains all possible effects in the game
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CardEffect {
    PlusOneData,
    PlusTwoAgainstData,
    DiscardOnePlaied,
    DiscardTwoPlaied,
    Antitrust,
    StealTwo,
    StopEffect,
    StopAttack,
    ChangeHands,
    AllChangeHands,
    FourCardVC,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BasicCard {
    pub title: String,
    pub effect: Option<CardEffect>,
}
impl sealed::Card for BasicCard {}
impl Card for BasicCard {
    fn title(&self) -> &String {
        &self.title
    }
    fn description(&self) -> &String {
        &self.title
    }
}

card_struct!(Adversary { strenght: u32 });
impl BattleCard for Adversary {
    fn strenght(&self) -> u32 {
        self.strenght
    }
}
impl Attack for Adversary {}

pub fn get_cards() -> Vec<impl Card> {
    vec![
        Adversary {
            title: "nyob (NGO)".to_string(),
            description: "Fighting against giants with GDPR.".to_string(),
            strenght: 1,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
        Adversary {
            title: "".to_string(),
            description: "".to_string(),
            strenght: 0,
        },
    ]
}
