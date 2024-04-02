use std::fmt;

use rand::prelude::*;

use super::Card;

#[derive(PartialEq, Eq)]
pub struct DeckEmptyError;

impl fmt::Display for DeckEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Deck is out of cards. You all lost!")
    }
}

impl fmt::Debug for DeckEmptyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn card_iter(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter()
    }

    #[must_use]
    pub fn contains(&self, c: &Card) -> bool {
        self.cards.contains(c)
    }

    /// # Panics
    /// If card is not found
    pub fn use_card(&mut self, c: &Card) {
        let pos = self
            .cards
            .iter()
            .position(|hc| hc == c)
            .expect("INTERNAL ERR: Card used should be in hand");
        self.cards.remove(pos);
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

    /// Draw a card from the deck
    ///
    /// # Errors
    /// If there are no more cards in the deck, returns a [`DeckEmptyError`]
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
            cards.push(Card::Adversary {
                title: format!("Card_{i}"),
                effect: None,
                description: "".to_string(),
                strenght: 0,
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
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn cards() -> Vec<Card> {
        let mut cards = vec![];

        for i in 1..=60 {
            cards.push(Card::Adversary {
                title: format!("Card_{i}"),
                effect: None,
                description: "".to_string(),
                strenght: 0,
            });
        }
        cards
    }

    #[test]
    fn hand_created_empty() {
        let h = Hand::new();

        assert!(h.is_empty());
    }

    #[rstest]
    fn can_add_cards(cards: Vec<Card>) {
        let mut h = Hand::new();

        for (i, card) in cards.into_iter().enumerate() {
            h.add(card);

            assert_eq!(h.len(), i + 1);
        }
    }

    #[rstest]
    fn can_add_multiple_cards(cards: Vec<Card>) {
        let mut h = Hand::new();
        let len_cards = cards.len();

        // When
        h.add_multiple(cards);
        // Then
        assert_eq!(h.len(), len_cards);
    }

    #[rstest]
    fn use_a_card(cards: Vec<Card>) {
        let mut h = Hand::new();
        let card = cards.first().expect("").clone();
        h.add_multiple(cards.clone());

        h.use_card(&card);

        assert_eq!(h.len(), cards.len() - 1);
    }

    #[rstest]
    fn use_a_card_should_remove_one_copy(cards: Vec<Card>) {
        let mut h = Hand::new();
        let card = cards.first().expect("").clone();
        h.add_multiple(cards.clone());
        h.add(card.clone());

        h.use_card(&card);

        assert_eq!(
            h.len(),
            cards.len() + 1 - 1,
            "more than one copy of the card was removed. Should not happen"
        );
    }

    mod take {
        use super::*;

        #[rstest]
        fn take_one_card(cards: Vec<Card>) {
            let mut h = Hand::new();
            let card = cards.first().expect("").clone();
            h.add(card.clone());
            // When
            let taken_cards = h.take(1);
            // Then
            assert_eq!(taken_cards, vec![card]);
            assert_eq!(h.len(), 0);
        }

        #[rstest]
        fn takes_a_random_card(cards: Vec<Card>) {
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

        #[rstest]
        fn takes_multiple_cards(cards: Vec<Card>) {
            let mut h = Hand::new();
            let card = cards.first().expect("").clone();
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
