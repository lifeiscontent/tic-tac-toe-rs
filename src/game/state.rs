use std::fmt::{Display, Formatter, Result};

use crate::game::{board::Board, player::Player};

#[derive(Debug)]
pub struct State {
    board: Board,
    player: Player,
}

impl State {
    pub fn next(&self, move_index: usize) -> Option<Self> {
        if !self.board.is_legal_move(move_index) {
            println!("Invalid move");
            return None;
        }
        Some(State {
            board: self.board.next(move_index, self.player),
            player: self.player.next(),
        })
    }

    pub fn is_over(&self) -> bool {
        self.board.is_won_by_player(Player::X)
            || self.board.is_won_by_player(Player::O)
            || self.board.is_tie()
    }

    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player: Player::X,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "-------------")?;
        writeln!(f, "{}", self.status())?;
        writeln!(f, "-------------\n\n{}", self.board)?;
        writeln!(f, "-------------")
    }
}

impl State {
    fn status(&self) -> String {
        match (
            self.board.is_won_by_player(Player::X),
            self.board.is_won_by_player(Player::O),
            self.board.is_tie(),
        ) {
            (true, _, _) => String::from("Player X Won!"),
            (_, true, _) => String::from("Player O Won!"),
            (_, _, true) => String::from("Tie Game!"),
            _ => format!("Player {} Turn", self.player),
        }
    }
}
