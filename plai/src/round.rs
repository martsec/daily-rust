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
    pub fn new(number: u32, players: Vec<usize>) -> Self {
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

    pub fn next_player(&mut self) {
        let _ = self.remaining_players.pop().map_or(0, |p| p);
        if self.remaining_players.is_empty() {
            self.next_round();
        }
    }
}

#[cfg(test)]
mod test_round {
    use crate::round::Round;

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
