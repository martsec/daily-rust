mod containers;

pub use containers::{Deck, Hand};

/// This enum contains all possible effects in the game
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Effect {
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
pub struct Card {
    pub title: String,
    pub effect: Option<Effect>,
}
