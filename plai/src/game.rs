use std::fmt;

use crate::cards::{Card, Deck, DeckEmptyError, Hand};
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
        let cards = vec![
            Card {
                title: "Card1".into(),
                effect: None,
            },
            Card {
                title: "Card2".into(),
                effect: None,
            },
            Card {
                title: "Card3".into(),
                effect: None,
            },
            Card {
                title: "Card4".into(),
                effect: None,
            },
        ];
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

pub enum TurnAction<'a> {
    Funding(Funding),
    SpecialCard(Card),
    HostileTakeover(&'a Player),
}

/// Player options
impl Game {
    pub fn active_player(&self) -> &Player {
        let pid = self.round.active_player();
        self.players
            .get(*pid)
            .expect("INNER ERROR: Player not found.")
    }

    fn active_player_mut(&mut self) -> &mut Player {
        let pid = self.round.active_player();
        self.players
            .get_mut(*pid)
            .expect("INNER ERROR: Player not found.")
    }

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
    fn family_funding(mut game: Game) {
        let original_deck_size = game.deck.len();
        let active_p = game.active_player();
        let original_card_num = active_p.hand.len();
        let active_pid = active_p.id;

        let action = TurnAction::Funding(Funding::Family);
        let _ = game.turn_action(active_pid, action);

        assert_eq!(game.active_player().hand.len(), original_card_num + 1);
        assert_eq!(game.deck.len(), original_deck_size - 1)
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
#[derive(Debug, Clone, PartialEq)]
pub struct Round {
    pub number: u32,
    players: Vec<usize>,
    remaining_players: Vec<usize>,
}

impl Round {
    fn new(number: u32, players: Vec<usize>) -> Self {
        let mut player_order = players.clone();
        player_order.reverse();
        Self {
            number,
            players,
            remaining_players: player_order,
        }
    }

    fn next_round(self) -> Result<Self, RoundNotEnded> {
        match self.remaining_players.len() {
            0 => Ok(Self::new(self.number + 1, self.players)),
            _ => Err(RoundNotEnded),
        }
    }

    fn active_player(&self) -> &usize {
        self.remaining_players
            .last()
            .expect("A round should have no empty player list")
    }

    fn next_player(&mut self) -> Result<usize, RoundEnded> {
        match self.remaining_players.pop() {
            Some(p) => Ok(p),
            None => Err(RoundEnded),
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

        let previous_player = r.next_player();

        assert_eq!(previous_player, Ok(0));
        assert_eq!(r.remaining_players.len(), 3);
    }

    #[test]
    fn error_when_no_new_player() {
        let ids = vec![0];
        let mut r = Round::new(0, ids);

        let previous_player = r.next_player();

        assert_eq!(previous_player, Ok(0));

        let err = r.next_player();
        assert_eq!(err, Err(RoundEnded));
    }

    #[test]
    fn cannot_end_round_if_players_remaining() {
        let ids = vec![0, 1, 2, 3];
        let r = Round::new(1, ids);

        assert_eq!(r.next_round(), Err(RoundNotEnded));
    }

    #[test]
    fn new_round() {
        let ids = vec![0, 1, 2, 3];
        let mut r = Round::new(1, ids);
        let _: Result<Vec<usize>, RoundEnded> = (0..4).map(|_| r.next_player()).collect();

        assert_eq!(r.remaining_players.len(), 0);

        let next_round = r.next_round().expect("Error in round");

        assert_eq!(next_round.number, 2);
        assert_eq!(next_round.remaining_players.len(), 4);
    }
}

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

#[derive(Debug, Clone)]
pub struct Player {
    id: usize,
    name: String,
    state: PlayerState,
    hand: Hand,
}

impl Player {
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

    pub fn state(&self) -> &PlayerState {
        &self.state
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
