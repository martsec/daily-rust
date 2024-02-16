use crate::cards::{Card, Deck, Hand};

///Represents a game of PLAI, containing all the items and logic
///to play the game until the end.
#[derive(Clone)]
pub struct Game {
    pub players: Vec<Player>,
    deck: Box<Deck>,
    pub round: u32,
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
        Self {
            players: player_names.iter().map(|n| Player::new(n)).collect(),
            deck: Box::new(deck),
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

#[derive(Clone)]
pub struct Player {
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
