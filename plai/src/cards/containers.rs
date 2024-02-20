use std::fmt;

use rand::prelude::*;

use crate::cards::Card;

#[derive(PartialEq)]
pub struct DeckEmptyError;

impl fmt::Display for DeckEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck is out of cards. You all lost!")
    }
}

impl fmt::Debug for DeckEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    #[must_use]
    pub const fn new() -> Self {
        Self { cards: vec![] }
    }

    /// Add a card to the hand
    pub fn add(&mut self, c: Card) {
        self.cards.push(c);
    }

    pub fn add_multiple(&mut self, cs: Vec<Card>) {
        self.cards.extend(cs);
    }

    pub fn take(&mut self, num: u32) -> Vec<Card> {
        let mut returned: Vec<Card> = vec![];
        let mut rng = rand::thread_rng();

        for _ in 1..=num {
            if self.is_empty() {
                return returned;
            }
            let idx = rng.gen_range(0..self.len());

            returned.push(self.cards.remove(idx));
        }
        returned
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    #[must_use]
    pub fn new(cards: Vec<Card>) -> Self {
        let mut deck = Self { cards };
        deck.shuffle();
        deck
    }

    pub fn draw(&mut self, num: usize) -> Result<Vec<Card>, DeckEmptyError> {
        let remaining = self.len();

        if remaining <= num {
            return Err(DeckEmptyError);
        }

        Ok(self.cards.drain((remaining - num)..remaining).collect())
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn cards() -> Vec<Card> {
        let mut cards = vec![];

        for i in 1..=5 {
            cards.push(Card {
                title: format!("Card_{i}"),
                effect: None,
            });
        }
        cards
    }

    #[rstest]
    fn new_deck_is_shufled(cards: Vec<Card>) {
        let deck = Deck::new(cards.clone());
        let deck2 = Deck::new(cards.clone());

        assert_ne!(deck.cards, deck2.cards);
        assert_ne!(deck.cards, cards);
        assert_ne!(deck2.cards, cards);
    }

    #[rstest]
    fn draw_decreases_remaining_cards(cards: Vec<Card>) {
        let mut deck = Deck::new(cards);

        let original_size = deck.len();
        let _ = deck.draw(1);
        let after_draw_size = deck.len();
        assert_ne!(original_size, after_draw_size);
        assert_eq!(original_size, after_draw_size + 1);
    }

    #[rstest]
    fn draw_empty_deck_returns_err() {
        let mut deck = Deck::new(vec![]);

        let drawn = deck.draw(1);

        assert_eq!(drawn, Err(DeckEmptyError));
    }

    #[rstest]
    fn draw_more_or_equal_cards_than_available(cards: Vec<Card>) {
        let num_cards = cards.len();
        let mut deck = Deck::new(cards);

        assert_eq!(deck.draw(num_cards), Err(DeckEmptyError));
        assert_eq!(deck.draw(num_cards + 1), Err(DeckEmptyError));
    }
}

#[cfg(test)]
mod testhand {
    #[allow(clippy::wildcard_imports)]
    use rstest::*;

    #[fixture]
    fn setup() {
        #[allow(clippy::unwrap_used)]
        color_eyre::install().unwrap();
    }

    use super::*;

    fn get_cards(num: u32) -> Vec<Card> {
        let mut cards = vec![];

        for i in 1..=num {
            cards.push(Card {
                title: format!("Card_{i}"),
                effect: None,
            });
        }
        cards
    }

    #[test]
    fn hand_created_empty() {
        let h = Hand::new();

        assert!(h.is_empty());
    }

    #[test]
    fn can_add_cards() {
        let mut h = Hand::new();

        for i in 1..10 {
            h.add(Card {
                title: "MyCard".into(),
                effect: None,
            });

            assert_eq!(h.len(), i);
        }
    }

    #[test]
    fn can_add_multiple_cards() {
        let mut h = Hand::new();
        let mut to_add = vec![];

        for _ in 1..10 {
            to_add.push(Card {
                title: "MyCard".into(),
                effect: None,
            });
        }
        // When
        h.add_multiple(to_add);
        // Then
        assert_eq!(h.len(), (1..10).len());
    }

    mod take {
        use super::*;

        #[test]
        fn take_one_card() {
            let mut h = Hand::new();
            let card = get_cards(1).first().expect("").clone();
            h.add(card.clone());
            // When
            let taken_cards = h.take(1);
            // Then
            assert_eq!(taken_cards, vec![card]);
            assert_eq!(h.len(), 0);
        }

        #[test]
        fn takes_a_random_card() {
            let cards = get_cards(20);
            let first_card = cards.first();
            let last_card = cards.last();

            let mut first_last_matches = 0;

            for _ in 1..100 {
                let mut h = Hand::new();
                h.add_multiple(cards.clone());
                let taken_cards = h.take(1);
                let taken_card = taken_cards.first();

                if taken_card == last_card || taken_card == first_card {
                    first_last_matches += 1;
                }
            }
            assert!(first_last_matches < 80);
        }

        #[test]
        fn takes_multiple_cards() {
            let mut h = Hand::new();
            let card = get_cards(1).first().expect("").clone();
            h.add(card.clone());
            h.add(card.clone());
            // When
            let taken_cards = h.take(2);
            // Then
            assert_eq!(taken_cards, vec![card.clone(), card]);
            assert_eq!(h.len(), 0);
        }

        #[test]
        fn does_not_take_any_if_there_are_none() {
            let mut h = Hand::new();

            assert_eq!(h.len(), 0);
            assert!(h.take(100).is_empty());
        }
    }
}
