use std::io;

mod board;
mod player;
mod state;

use self::state::State;

#[derive(Debug)]
pub struct Game {
    history: Vec<State>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            history: vec![State::new()],
        }
    }

    fn current_state(&self) -> &State {
        self.history.last().unwrap()
    }

    fn parse_input(&self) -> Option<usize> {
        println!("Please input your guess. e.g. 0,1): ");

        let mut input_buffer = String::new();
        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read input");

        let mut iter = input_buffer
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap());

        match (iter.next(), iter.next(), iter.next()) {
            (Some(x), Some(y), None) if (y * 3 + x) <= 8 => Some(y * 3 + x),
            _ => None,
        }
    }

    fn update(&mut self) -> Option<State> {
        match self.parse_input() {
            Some(move_index) => self.current_state().next(move_index),
            None => {
                println!("Invalid input");
                None
            }
        }
    }

    pub fn render(&mut self) -> () {
        loop {
            let current_state = self.current_state();

            println!("{current_state}");

            if current_state.is_over() {
                break;
            }

            match self.update() {
                Some(next_state) => self.history.push(next_state),
                None => continue,
            };
        }
    }
}
