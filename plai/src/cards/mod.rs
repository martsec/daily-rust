mod containers;

pub use containers::{Deck, DeckEmptyError, Hand};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Card {
    Adversary {
        title: String,
        description: String,
        strenght: u8,
        effect: Option<CardEffect>,
    },
    Buzzword {
        title: String,
        description: String,
        strenght: u8,
        effect: Option<CardEffect>,
    },
    UseCase {
        title: String,
        description: String,
        strenght: u8,
        effect: Option<CardEffect>,
    },
    Special {
        title: String,
        description: String,
        effect: CardEffect,
    },
    MarketEvent {
        title: String,
        description: String,
        effect: CardEffect,
    },
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

#[must_use]
pub fn get_cards_available() -> Vec<Card> {
    use Card::Adversary;
    vec![
        Adversary {
            title: "nyob (NGO)".to_string(),
            description: "Fighting against giants with GDPR.".to_string(),
            strenght: 1,
            effect: None,
        },
        Adversary {
            title: "C1".to_string(),
            description: "D1".to_string(),
            strenght: 0,
            effect: None,
        },
    ]
}
