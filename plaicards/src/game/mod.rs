use std::fmt;

mod cards;
mod player;
mod round;
use crate::game::cards::{get_cards_available, Card, Deck, DeckEmptyError};
pub use crate::game::player::Player;
use crate::game::player::PlayerState;
use crate::game::round::Round;

///Represents a game of PLAI, containing all the items and logic
///to play the game until the end.
#[derive(Clone, Debug)]
pub struct Game {
    pub players: Vec<Player>,
    deck: Box<Deck>,
    pub round: Round,
}

impl Game {
    #[must_use]
    pub fn new(player_names: &[String]) -> Self {
        let cards = get_cards_available();
        let deck = Deck::new(cards);
        let players: Vec<Player> = player_names
            .iter()
            .enumerate()
            .map(|(id, n)| Player::new(id, n))
            .collect();
        let players_id: Vec<usize> = players.iter().map(|p| p.id).collect();

        Self {
            players,
            deck: Box::new(deck),
            round: Round::new(0, players_id),
        }
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
    pub fn get_player(&self, id: usize) -> &Player {
        self.players
            .iter()
            .find(|p| p.id == id)
            .expect("INNER ERROR: Player not found.")
    }

    /// # Panics
    /// If user id does not exist
    #[must_use]
    pub fn active_player(&self) -> &Player {
        let pid = self.round.active_player();
        self.players
            .get(*pid)
            .expect("INNER ERROR: Player not found.")
    }

    /// # Panics
    /// If user id does not exist
    #[must_use]
    fn active_player_mut(&mut self) -> &mut Player {
        let pid = self.round.active_player();
        self.players
            .get_mut(*pid)
            .expect("INNER ERROR: Player not found.")
    }

    /// Execute an action for a given player and ends the turn
    ///
    /// # Errors
    /// If deck is empty, returns an error
    pub fn turn_action(
        &mut self,
        player_id: usize,
        action: TurnAction,
    ) -> Result<(), DeckEmptyError> {
        let player = self.active_player();
        if player.id != player_id {
            todo!();
        }

        match action {
            TurnAction::Funding(f) => self.do_funding(f),
            TurnAction::HostileTakeover(target) => todo!(),
            TurnAction::SpecialCard(c) => self.do_special(c),
        }?;

        // TODO implement battle logic when needed
        self.end_turn();
        Ok(())
    }

    fn do_funding(&mut self, f: Funding) -> Result<(), DeckEmptyError> {
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

    fn do_special(&mut self, c: &Card) -> Result<(), DeckEmptyError> {
        use crate::game::cards::CardEffect::*;
        self.active_player_mut().hand.use_card(&c);
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
        let players = ["P1".to_string(), "P2".to_string(), "P3".to_string()];
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
    fn special_removes_from_player_hand(mut game: Game) {
        let active_p = game.active_player_mut();
        let card = special_card(CardEffect::DrawTwo);
        active_p.hand.add_multiple(vec![card.clone()]);
        let active_pid = active_p.id;

        dbg!(game.active_player());
        let action = TurnAction::SpecialCard(&card);
        let _ = game.turn_action(active_pid, action);

        dbg!(game.active_player());
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

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub enum Funding {
    Family,
    Regional,
    VC,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TurnAction<'a> {
    Funding(Funding),
    SpecialCard(&'a Card),
    HostileTakeover(&'a Player),
}
