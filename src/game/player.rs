use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
