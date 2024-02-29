use std::fmt;

use crate::cards::{get_cards_available, Card, Deck, DeckEmptyError, Hand};
///Represents a game of PLAI, containing all the items and logic
///to play the game until the end.
#[derive(Clone)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum TurnAction<'a> {
    Funding(Funding),
    SpecialCard(&'a Card),
    HostileTakeover(&'a Player),
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

    /// Execute an action for a given player
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
            TurnAction::SpecialCard(c) => todo!(),
        }
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
}

#[cfg(test)]
mod test_game_actions {
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
    fn family_funding(mut game: Game) {
        let original_deck_size = game.deck.len();
        let active_p = game.active_player();
        let original_card_num = active_p.hand.len();
        let active_pid = active_p.id;

        let action = TurnAction::Funding(Funding::Family);
        let _ = game.turn_action(active_pid, action);

        assert_eq!(game.active_player().hand.len(), original_card_num + 1);
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
}

/// End conditions
impl Game {
    fn has_ended_as_monopoly(&self) -> bool {
        let num_startups = self
            .players
            .iter()
            .filter(|p| p.state == PlayerState::Startup)
            .count();

        let num_eliminated = self
            .players
            .iter()
            .filter(|p| p.state == PlayerState::Eliminated)
            .count();

        num_startups == 1 && (num_startups + num_eliminated) == self.players.len()
    }

    fn has_ended_as_open_source(&self) -> bool {
        let num_os = self
            .players
            .iter()
            .filter(|p| p.state == PlayerState::OpenSource)
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

/// Keeps information on the round.
///
/// * Round number
/// * Player order
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round {
    pub number: u32,
    players: Vec<usize>,
    remaining_players: Vec<usize>,
}

impl Round {
    #[must_use]
    fn new(number: u32, players: Vec<usize>) -> Self {
        let mut player_order = players.clone();
        player_order.reverse();
        Self {
            number,
            players,
            remaining_players: player_order,
        }
    }

    fn next_round(&mut self) -> () {
        if self.remaining_players.len() == 0 {
            self.number += 1;
            self.remaining_players = self.players.clone();
            self.remaining_players.reverse();
        }
    }

    /// # Panics
    /// If there are no remaining players. This is something that should
    /// not happen.
    ///
    /// TODO see if there is a way to avoid representing the empty group
    /// at compile time
    #[must_use]
    pub fn active_player(&self) -> &usize {
        self.remaining_players
            .last()
            .expect("INTERNAL ERROR: A round should have no empty player list")
    }

    #[must_use]
    pub fn next_player(&mut self) {
        let _ = self.remaining_players.pop().map_or(0, |p| p);
        if self.remaining_players.is_empty() {
            self.next_round();
        }
    }
}

#[cfg(test)]
mod test_round {
    use super::*;

    #[test]
    fn creation() {
        let ids = vec![0, 1, 2, 3];
        let r = Round::new(0, ids);

        assert_eq!(r.number, 0);
        assert_eq!(r.players, vec![0, 1, 2, 3]);
    }

    #[test]
    fn next_player() {
        let ids = vec![0, 1, 2, 3];
        let mut r = Round::new(0, ids);

        let previous_player = *r.active_player();
        r.next_player();

        assert_eq!(previous_player, 0);
        assert_eq!(r.remaining_players.len(), 3);
    }

    #[test]
    fn when_ending_round_starts_a_new() {
        let ids = vec![0, 1];
        let mut r = Round::new(0, ids);

        let round_num = r.number;
        let first = *r.active_player();
        r.next_player();
        r.next_player();

        assert_eq!(*r.active_player(), first, "Should return first player");
        assert_eq!(r.number, round_num + 1);
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq)]
pub enum Funding {
    Family,
    Regional,
    VC,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    Startup,
    OpenSource,
    Eliminated,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Startup
    }
}

impl std::fmt::Display for PlayerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Self::Startup => "Startup",
            Self::OpenSource => "Open Source",
            Self::Eliminated => "Eliminated",
        };
        write!(f, "{text}")
    }
}

/// Represnts a player and its elements:
///
/// * [`PlayerState`]
/// * [`Hand`] representing the cards the player has
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    id: usize,
    name: String,
    state: PlayerState,
    hand: Hand,
}

impl Player {
    #[must_use]
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
            state: PlayerState::default(),
            hand: Hand::new(),
        }
    }

    fn update_state(&mut self) {
        use PlayerState::{Eliminated, OpenSource, Startup};
        if self.hand.is_empty() {
            match self.state {
                Startup => self.state = OpenSource,
                OpenSource | Eliminated => (),
            }
        }
    }

    #[must_use]
    fn possible_actions(&self) -> Vec<TurnAction> {
        use Funding::{Family, Regional, VC};
        let mut actions = vec![
            TurnAction::Funding(Family),
            TurnAction::Funding(Regional),
            TurnAction::Funding(VC),
        ];

        for c in self.hand.card_iter() {
            match c {
                Card::Special { .. } => actions.push(TurnAction::SpecialCard(c)),
                _ => (),
            }
        }

        actions
    }

    #[must_use]
    pub const fn state(&self) -> &PlayerState {
        &self.state
    }

    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod test_player {
    use crate::cards::CardEffect;

    use super::*;
    use rstest::{fixture, rstest};

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

    #[rstest]
    fn next_turn_changes_player(mut cards: Vec<Card>) {
        let mut p = Player::new(0, "Test");

        p.hand.add(cards.pop().expect(""));
    }

    #[rstest]
    fn possible_actions_startup_funding() {
        use Funding::{Family, Regional, VC};
        let p = Player::new(0, "Test");

        for f_type in [Family, Regional, VC] {
            assert!(p.possible_actions().contains(&TurnAction::Funding(f_type)));
        }
    }

    #[rstest]
    fn possible_actions_special_card() {
        let mut p = Player::new(0, "Test");
        let c = Card::Special {
            title: "c".into(),
            description: "c".into(),
            effect: CardEffect::FourCardVC,
        };
        p.hand.add(c.clone());

        dbg!(p.possible_actions());
        assert!(p.possible_actions().contains(&TurnAction::SpecialCard(&c)));
    }

    #[rstest]
    fn possible_actions_no_special_card() {
        let p = Player::new(0, "Test");

        dbg!(p.possible_actions());
        for a in p.possible_actions() {
            assert!(
                match a {
                    TurnAction::SpecialCard(_) => false,
                    _ => true,
                },
                "Found SpecialCard action when there are no special cards in the hand"
            );
        }
    }
}
