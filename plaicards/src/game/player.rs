use uuid::Uuid;

use crate::game::cards::{Card, Hand};
use crate::game::Funding::{Family, Regional, VC};
use crate::game::TurnAction;

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
    pub id: Uuid,
    name: String,
    state: PlayerState,
    pub hand: Hand,
}

impl Player {
    #[must_use]
    pub fn new(id: Uuid, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
            state: PlayerState::default(),
            hand: Hand::new(),
        }
    }

    pub fn update_state(&mut self) {
        use PlayerState::{Eliminated, OpenSource, Startup};
        if self.hand.is_empty() {
            match self.state {
                Startup => self.state = OpenSource,
                OpenSource | Eliminated => (),
            }
        }
    }

    #[must_use]
    pub fn possible_actions(&self) -> Vec<TurnAction> {
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
    use crate::game::cards::CardEffect;
    use uuid::Uuid;

    use super::*;
    use rstest::{fixture, rstest};

    #[fixture]
    fn cards() -> Vec<Card> {
        let mut cards = vec![];

        for i in 1..=60 {
            cards.push(Card::Adversary {
                title: format!("Card_{i}"),
                effect: None,
                description: String::new(),
                strenght: 0,
            });
        }
        cards
    }

    #[fixture]
    fn player() -> Player {
        Player::new(Uuid::new_v4(), "Test")
    }

    #[rstest]
    fn next_turn_changes_player(mut cards: Vec<Card>, mut player: Player) {
        player.hand.add(cards.pop().expect(""));
    }

    #[rstest]
    fn possible_actions_startup_funding(player: Player) {
        use crate::game::Funding::{Family, Regional, VC};

        for f_type in [Family, Regional, VC] {
            assert!(player
                .possible_actions()
                .contains(&TurnAction::Funding(f_type)));
        }
    }

    #[rstest]
    fn possible_actions_special_card(mut player: Player) {
        let c = Card::Special {
            title: "c".into(),
            description: "c".into(),
            effect: CardEffect::DrawTwo,
        };
        player.hand.add(c.clone());

        dbg!(player.possible_actions());
        assert!(player
            .possible_actions()
            .contains(&TurnAction::SpecialCard(&c)));
    }

    #[rstest]
    fn possible_actions_no_special_card(player: Player) {
        dbg!(player.possible_actions());
        for a in player.possible_actions() {
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
