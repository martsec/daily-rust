use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

mod cards;
mod errors;
mod player;
mod round;
use crate::game::cards::{get_cards_available, Deck, DeckEmptyError};
pub use crate::game::cards::{Card, Hand};
pub use crate::game::player::Player;
use crate::game::player::PlayerState;
use crate::game::round::Round;

pub use crate::game::errors::{Error, Result};

///Represents a game of PLAI, containing all the items and logic
///to play the game until the end.
#[derive(Clone, Debug)]
pub struct Game {
    pub players: Vec<Player>,
    deck: Box<Deck>,
    pub round: Round,
}

// Setup
impl Game {
    const INITIAL_CARDS: u8 = 6;

    #[must_use]
    pub fn new(players: &[(Uuid, String)]) -> Self {
        let cards = get_cards_available();
        let deck = Deck::new(cards);
        let players: Vec<Player> = players.iter().map(|(id, n)| Player::new(*id, n)).collect();
        let players_id: Vec<Uuid> = players.iter().map(|p| p.id).collect();

        let mut game = Self {
            players,
            deck: Box::new(deck),
            round: Round::new(0, players_id),
        };

        game.initial_deal();

        game
    }

    fn initial_deal(&mut self) {
        self.players.iter_mut().for_each(|p| {
            p.hand.add_multiple(
                self.deck
                    .draw(Self::INITIAL_CARDS.into())
                    .expect("INTERNAL ERROR dealing cards"),
            );
        });
    }
}

/// Player options
impl Game {
    /// Ends the turn and computes logic for startup elimination
    /// and new round if needed
    pub fn end_turn(&mut self) {
        self.round.next_player();
    }

    /// # Panics
    /// If user id does not exist
    pub fn get_player(&self, id: Uuid) -> &Player {
        self.players
            .iter()
            .find(|p| p.id == id)
            .expect("INNER ERROR: Player not found.")
    }

    /// # Panics
    /// If user id does not exist
    pub fn get_player_mut(&mut self, id: Uuid) -> &mut Player {
        self.players
            .iter_mut()
            .find(|p| p.id == id)
            .expect("INNER ERROR: Player not found.")
    }
    /// # Panics
    /// If user id does not exist
    #[must_use]
    pub fn active_player(&self) -> &Player {
        let pid = self.round.active_player();
        self.players
            .iter()
            .find(|p| p.id == *pid)
            .expect("INNER ERROR: Player not found.")
    }

    /// # Panics
    /// If user id does not exist
    #[must_use]
    fn active_player_mut(&mut self) -> &mut Player {
        let pid = self.round.active_player();
        self.players
            .iter_mut()
            .find(|p| p.id == *pid)
            .expect("INNER ERROR: Player not found.")
    }

    fn ensure_player_can_act(&self, player_id: Uuid) -> Result<()> {
        if self.has_ended() {
            return Err(Error::GameEnded);
        }
        let player = self.active_player();
        if player.id != player_id {
            return Err(Error::NotYourTurn);
        }
        Ok(())
    }

    /// Execute an action for a given player and ends the turn
    ///
    /// ### Errors
    ///
    /// * ``EmptyDeck``
    /// * ``GameEnded``
    /// * ``NotYourTurn``
    pub fn turn_action(&mut self, player_id: Uuid, action: TurnAction) -> Result<()> {
        self.ensure_player_can_act(player_id)?;

        match action {
            TurnAction::Funding(f) => self.do_funding(f),
            TurnAction::HostileTakeover(target) => todo!(),
            TurnAction::SpecialCard(c) => self.do_special(c),
        }?;

        // TODO implement battle logic when needed
        self.end_turn();
        Ok(())
    }

    fn do_funding(&mut self, f: Funding) -> Result<()> {
        let deck = &mut self.deck;
        match f {
            Funding::Family => {
                let cards = deck.draw(1)?;
                self.active_player_mut().hand.add_multiple(cards);
                Ok(())
            }
            Funding::Regional => todo!(),
            Funding::VC => todo!(),
        }
    }

    fn do_special(&mut self, c: &Card) -> Result<()> {
        use crate::game::cards::CardEffect::*;
        self.active_player_mut().hand.use_card(c);
        let deck = &mut self.deck;
        if let Card::Special { effect, .. } = c {
            match effect {
                DrawTwo => {
                    let cards = deck.draw(2)?;
                    self.active_player_mut().hand.add_multiple(cards);
                }
                DrawThree => {
                    let cards = deck.draw(3)?;
                    self.active_player_mut().hand.add_multiple(cards);
                }

                AllDrawFour => {
                    self.players.iter_mut().try_for_each(|p| {
                        let cards = deck.draw(4)?;
                        p.hand.add_multiple(cards);
                        Ok(())
                    })?;
                }
                AllDiscardFour => {
                    // TODO players should be able to choose discarded cards
                    // TODO discarded cards should go to the discard pile
                    self.players.iter_mut().try_for_each(|p| {
                        p.hand.take(4);
                        Ok(())
                    })?;
                }
                AllDiscardOne => {
                    self.players.iter_mut().try_for_each(|p| {
                        p.hand.take(1);
                        Ok(())
                    })?;
                }
                Antitrust => {
                    // TODO players should be able to choose discarded cards
                    // TODO discarded cards should go to the discard pile
                    self.players
                        .iter_mut()
                        .filter(|p| p.hand.len() > 9)
                        .try_for_each(|p| {
                            p.hand.take(10);
                            Ok(())
                        })?;
                }
                CardsToNextPlayer => {
                    // Shifht hands
                    let mut hands = self
                        .players
                        .iter()
                        .map(|p| p.hand.clone())
                        .collect::<Vec<Hand>>();
                    hands.rotate_right(1);
                    // Assign shifted hands to players
                    self.players
                        .iter_mut()
                        .zip(hands)
                        .for_each(|(p, h)| p.hand = h);
                }
                ChangeHands => {
                    //let hand = self.active_player().hand.clone();
                    //let other_player_hand = self.get_player(rival_id).hand.clone();
                    //self.active_player_mut().hand = other_player_hand;
                    //self.get_player_mut(rival_id).hand = hand;
                    todo!();
                }
                FourCardVc => todo!(),
                ReviveCard => todo!(),
                SpyPlayer => todo!(),
                StealCat => todo!(),
                Steal2Cards => todo!(),
                DiscardAttack => {
                    self.active_player_mut()
                        .hand
                        .remove(|c| matches!(c, Card::Adversary { .. }));
                }

                DiscardBuzzwords
                | PlusTwoVsData
                | PlusTwoVsDeceptive
                | PlusFourVsData
                | PlusOneData
                | PlusOnePython
                | PlusTwoVsManagers
                | PlusTwoBuzzwords
                | PlusThreeCEOs
                | RemovesEffect
                | DiscardOne
                | DiscardBuzzwordsRival
                | DiscardTwo
                | DiscardOneEach
                | CannotDiscard
                | DiscardThreeDrawTwo
                | DiscardThree => return Err(Error::RuleBreak),

                NoEffect => {}
                StopEffect | StopAttack => return Err(Error::RuleBreak),
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod test_game_actions {
    use crate::game::cards::{Card::Special, CardEffect};

    use super::*;
    use rstest::*;

    #[fixture]
    fn game() -> Game {
        let players = [
            (Uuid::new_v4(), "P1".to_string()),
            (Uuid::new_v4(), "P2".to_string()),
            (Uuid::new_v4(), "P3".to_string()),
        ];
        Game::new(&players)
    }

    #[rstest]
    fn next_turn_changes_player(mut game: Game) {
        let p1 = game.active_player().id;

        game.end_turn();

        assert_ne!(game.active_player().id, p1);
    }

    #[rstest]
    fn next_turn_advances_round(mut game: Game) {
        let p1 = game.active_player().id;
        let round_num = game.round.number;
        for _ in 0..game.players.len() {
            game.end_turn();
        }
        assert_eq!(game.round.number, round_num + 1);
        assert_eq!(game.active_player().id, p1);
    }

    #[rstest]
    fn after_an_action_turn_is_ended(mut game: Game) {
        let first_player = game.active_player().id;

        let action = TurnAction::Funding(Funding::Family);
        let _ = game.turn_action(first_player, action);

        assert_ne!(
            game.active_player().id,
            first_player,
            "After taking an action (and thus the turn), active player should be different."
        );
    }

    #[rstest]
    fn family_funding(mut game: Game) {
        let original_deck_size = game.deck.len();
        let active_p = game.active_player();
        let original_card_num = active_p.hand.len();
        let active_pid = active_p.id;

        let action = TurnAction::Funding(Funding::Family);
        let _ = game.turn_action(active_pid, action);

        assert_eq!(
            game.get_player(active_pid).hand.len(),
            original_card_num + 1
        );
        assert_eq!(game.deck.len(), original_deck_size - 1);
    }

    // #[rstest]
    // fn regional_funding(mut game: Game) {
    //     let original_deck_size = game.deck.len();
    //     let active_p = game.active_player();
    //     let original_card_num = active_p.hand.len();
    //     let active_pid = active_p.id;

    //     let action = TurnAction::Funding(Funding::Regional);
    //     let _ = game.turn_action(active_pid, action);

    //     assert_eq!(game.active_player().hand.len(), original_card_num + 2);
    //     assert_eq!(game.deck.len(), original_deck_size - 2)
    // }

    // #[rstest]
    // fn regional_funding_next_player_can_attak(mut game: Game) {
    //     let original_deck_size = game.deck.len();
    //     let active_p = game.active_player();
    //     let original_card_num = active_p.hand.len();
    //     let active_pid = active_p.id;

    //     let action = TurnAction::Funding(Funding::Regional);
    //     let _ = game.turn_action(active_pid, action);
    //     let _ = game.end_turn();

    //     todo!();
    // }

    // #[rstest]
    // fn vc_funding(mut game: Game) {
    //     let original_deck_size = game.deck.len();
    //     let active_p = game.active_player();
    //     let original_card_num = active_p.hand.len();
    //     let active_pid = active_p.id;

    //     let action = TurnAction::Funding(Funding::VC);
    //     let _ = game.turn_action(active_pid, action);

    //     assert_eq!(game.active_player().hand.len(), original_card_num + 3);
    //     assert_eq!(game.deck.len(), original_deck_size - 3)
    // }

    fn special_card(effect: CardEffect) -> Card {
        Special {
            title: String::new(),
            description: String::new(),
            effect,
        }
    }

    #[rstest]
    fn special_draw_two(mut game: Game) {
        let active_p = game.active_player_mut();
        let card = special_card(CardEffect::DrawTwo);
        active_p.hand.add_multiple(vec![card.clone()]);
        let active_pid = active_p.id;

        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(active_pid, action);

        assert!(
            !game.active_player().hand.contains(&card),
            "Card used must be removed from hand"
        );
    }

    #[rstest]
    fn special_draw_three(mut game: Game) {
        let original_deck_size = game.deck.len();
        let active_p = game.active_player_mut();
        let card = special_card(CardEffect::DrawThree);
        active_p.hand.add_multiple(vec![card.clone()]);
        // We are using one card, so the hand will have one less
        let original_card_num = active_p.hand.len() - 1;
        let active_pid = active_p.id;

        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(active_pid, action);

        assert_eq!(game.deck.len(), original_deck_size - 3);
        assert_eq!(
            game.get_player(active_pid).hand.len(),
            original_card_num + 3
        );
    }

    #[rstest]
    fn special_all_draw_four(mut game: Game) {
        let should_be_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len() + 4).collect();
        let card = special_card(CardEffect::AllDrawFour);
        {
            game.active_player_mut().hand.add(card.clone());
        }
        let p = game.active_player().id;

        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(p, action);

        let end_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len()).collect();

        assert_eq!(end_cards, should_be_cards);
    }

    #[rstest]
    fn special_all_discard_one(mut game: Game) {
        let should_be_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len() - 1).collect();
        let card = special_card(CardEffect::AllDiscardOne);
        {
            game.active_player_mut().hand.add(card.clone());
        }
        let p = game.active_player().id;

        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(p, action);

        let end_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len()).collect();

        assert_eq!(end_cards, should_be_cards);
    }

    #[rstest]
    fn special_all_discard_four(mut game: Game) {
        let should_be_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len() - 4).collect();
        let card = special_card(CardEffect::AllDiscardFour);
        {
            game.active_player_mut().hand.add(card.clone());
        }
        let p = game.active_player().id;

        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(p, action);

        let end_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len()).collect();

        assert_eq!(end_cards, should_be_cards);
    }

    #[rstest]
    fn special_all_discard_four_players_with_low_card_num(mut game: Game) {
        // Set active player with few cards
        {
            let hand = &mut game.active_player_mut().hand;
            let num: u32 = hand.len() as u32;
            hand.take(num - 1);
        }
        let should_be_cards: Vec<usize> = game
            .players
            .iter()
            .map(|p| p.hand.len().saturating_sub(4)) // Saturating at 0
            .collect();
        let card = special_card(CardEffect::AllDiscardFour);
        {
            game.active_player_mut().hand.add(card.clone());
        }
        dbg!(&game.active_player().hand);
        let p = game.active_player().id;

        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(p, action);

        let end_cards: Vec<usize> = game.players.iter().map(|p| p.hand.len()).collect();

        assert_eq!(end_cards, should_be_cards);
    }

    #[rstest]
    fn special_discard_attack(mut game: Game) {
        // Given
        let card = special_card(CardEffect::DiscardAttack);
        {
            let p = game.active_player_mut();
            p.hand.add(card.clone());
            p.hand.add(Card::Adversary {
                title: "test".to_string(),
                description: "test".to_string(),
                strength: 0,
                effect: CardEffect::NoEffect,
            })
        }
        let original_attack_cards = game
            .active_player()
            .hand
            .card_iter()
            .filter(|c| matches!(c, Card::Adversary { .. }))
            .count();
        assert!(original_attack_cards > 0, "Test setup failure");
        let p = game.active_player().id;

        // When
        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(p, action);

        // Then
        let num_attack_cards = game
            .get_player(p)
            .hand
            .card_iter()
            .filter(|c| matches!(c, Card::Adversary { .. }))
            .count();

        assert_eq!(num_attack_cards, 0);
    }

    #[rstest]
    fn special_change_all_hands(mut game: Game) {
        use crate::game::cards::Hand;
        let mut should_be_hands = game
            .players
            .iter()
            .map(|p| p.hand.clone())
            .collect::<Vec<Hand>>();
        should_be_hands.rotate_right(1);

        let card = special_card(CardEffect::CardsToNextPlayer);
        {
            game.active_player_mut().hand.add(card.clone());
        }
        // When
        let p = game.active_player().id;
        let _ = game.turn_action(p, TurnAction::SpecialCard(&card));

        let result_hands = game
            .players
            .iter()
            .map(|p| p.hand.clone())
            .collect::<Vec<Hand>>();
        // Then
        assert_eq!(should_be_hands, result_hands);
    }
}

/// End conditions
impl Game {
    fn has_ended_as_monopoly(&self) -> bool {
        let num_startups = self
            .players
            .iter()
            .filter(|p| *p.state() == PlayerState::Startup)
            .count();

        let num_eliminated = self
            .players
            .iter()
            .filter(|p| *p.state() == PlayerState::Eliminated)
            .count();

        num_startups == 1 && (num_startups + num_eliminated) == self.players.len()
    }

    fn has_ended_as_open_source(&self) -> bool {
        let num_os = self
            .players
            .iter()
            .filter(|p| *p.state() == PlayerState::OpenSource)
            .count();
        num_os == self.players.len()
    }

    fn has_ended(&self) -> bool {
        self.has_ended_as_open_source() || self.has_ended_as_monopoly()
    }
}

#[derive(Debug, PartialEq)]
struct RoundEnded;

impl fmt::Display for RoundEnded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Round ended")
    }
}

#[derive(Debug, PartialEq)]
struct RoundNotEnded;

impl fmt::Display for RoundNotEnded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Round ended")
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum Funding {
    Family,
    Regional,
    VC,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurnAction<'a> {
    Funding(Funding),
    SpecialCard(&'a Card),
    HostileTakeover(&'a Player),
}
