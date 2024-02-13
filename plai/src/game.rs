use crate::cards::{Deck, Hand};

///Represents a game of PLAI, containing all the items and logic
///to play the game until the end.
struct Game {
    players: Vec<Player>,
    deck: Box<Deck>,
    round: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            players: vec![Player::new("P1")],
            deck: Box::new(Deck::new()),
            round: 0,
        }
    }

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

#[derive(Debug, PartialEq, Eq)]
enum PlayerState {
    Startup,
    OpenSource,
    Eliminated,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::Startup
    }
}

struct Player {
    name: String,
    state: PlayerState,
    hand: Hand,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            state: PlayerState::default(),
            hand: Hand::new(),
        }
    }

    fn update_state(&mut self) {
        use PlayerState::*;
        if self.hand.is_empty() {
            match self.state {
                Startup => self.state = OpenSource,
                OpenSource => (),
                Eliminated => (),
            }
        }
    }
}
