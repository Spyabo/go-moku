use crossterm::event::KeyCode;

use crate::game::{Game, Player};

#[derive(Debug)]
pub struct App {
    pub game: Game,
    pub cursor_row: usize,
    pub cursor_col: usize,
}

impl App {
    pub fn new(board_size: usize) -> Self {
        Self {
            game: Game::new(board_size),
            cursor_row: board_size / 2,
            cursor_col: board_size / 2,
        }
    }

    pub fn handle_key(&mut self, code: KeyCode) {
        match code {
            KeyCode::Left | KeyCode::Char('h') => {
                self.cursor_col = self.cursor_col.saturating_sub(1);
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.cursor_col = (self.cursor_col + 1).min(self.game.size() - 1);
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.cursor_row = self.cursor_row.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.cursor_row = (self.cursor_row + 1).min(self.game.size() - 1);
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if self.game.winner().is_none() {
                    let _ = self.game.place(self.cursor_row, self.cursor_col);
                }
            }
            KeyCode::Char('r') => {
                let size = self.game.size();
                self.game = Game::new(size);
                self.cursor_row = size / 2;
                self.cursor_col = size / 2;
            }
            _ => {}
        }
    }

    pub fn current_player(&self) -> Player {
        self.game.current_player()
    }
}
