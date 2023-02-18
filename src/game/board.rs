use std::fmt::Display;

use crate::game::player::Player;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    state: [Option<Player>; 9],
}

fn fmt_cell(cell: Option<Player>) -> String {
    match cell {
        Some(Player::X) => "X".to_string(),
        Some(Player::O) => "O".to_string(),
        None => "-".to_string(),
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-------------")?;
        for col in 0..=2 {
            let cell_1 = self.state[col * 3];
            let cell_2 = self.state[col * 3 + 1];
            let cell_3 = self.state[col * 3 + 2];
            writeln!(
                f,
                "|-{}-|-{}-|-{}-|",
                fmt_cell(cell_1),
                fmt_cell(cell_2),
                fmt_cell(cell_3)
            )?;
        }
        writeln!(f, "-------------")?;
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        Self { state: [None; 9] }
    }
    pub fn next(&self, index: usize, player: Player) -> Self {
        Self {
            state: self
                .state
                .iter()
                .enumerate()
                .map(|(i, &cell)| if i == index { Some(player) } else { cell })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
    pub fn is_tie(&self) -> bool {
        self.state.iter().all(Option::is_some)
    }
    pub fn is_won_by_player(&self, player: Player) -> bool {
        (0..3).any(|i| {
            self.is_won_by_player_vertical(i, player)
                || self.is_won_by_player_horizontal(i * 3, player)
                || self.is_won_by_player_diagonal(i, player)
        })
    }

    pub fn is_legal_move(&self, index: usize) -> bool {
        self.state.get(index).map_or(false, |cell| cell.is_none())
    }
    fn is_won_by_player_diagonal(&self, index: usize, player: Player) -> bool {
        let diagonal = match index {
            0 => [0, 4, 8],
            2 => [2, 4, 6],
            _ => return false,
        };

        diagonal.iter().all(|&i| self.state[i] == Some(player))
    }

    fn is_won_by_player_horizontal(&self, index: usize, player: Player) -> bool {
        (0..3).all(|i| {
            self.state
                .get(index + i)
                .map_or(false, |cell| cell == &Some(player))
        })
    }
    fn is_won_by_player_vertical(&self, index: usize, player: Player) -> bool {
        (0..3).all(|i| {
            self.state
                .get(index + i * 3)
                .map_or(false, |cell| cell == &Some(player))
        })
    }
}
