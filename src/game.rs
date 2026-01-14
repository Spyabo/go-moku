#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn symbol(self) -> char {
        match self {
            Player::Black => 'X',
            Player::White => 'O',
        }
    }
}

#[derive(Debug)]
pub struct Game {
    size: usize,
    board: Vec<Vec<Option<Player>>>,
    current: Player,
    winner: Option<Player>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let board = vec![vec![None; size]; size];
        Self {
            size,
            board,
            current: Player::Black,
            winner: None,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn board(&self) -> &Vec<Vec<Option<Player>>> {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current
    }

    pub fn winner(&self) -> Option<Player> {
        self.winner
    }

    pub fn place(&mut self, row: usize, col: usize) -> Result<(), PlaceError> {
        if row >= self.size || col >= self.size {
            return Err(PlaceError::OutOfBounds);
        }
        if self.winner.is_some() {
            return Err(PlaceError::GameOver);
        }
        if self.board[row][col].is_some() {
            return Err(PlaceError::Occupied);
        }

        self.board[row][col] = Some(self.current);

        if self.is_winning_move(row, col, self.current) {
            self.winner = Some(self.current);
        } else {
            self.current = self.current.other();
        }

        Ok(())
    }

    fn is_winning_move(&self, row: usize, col: usize, player: Player) -> bool {
        let directions = [(1isize, 0isize), (0, 1), (1, 1), (1, -1)];

        directions.into_iter().any(|(dr, dc)| {
            let count = 1
                + self.count_dir(row, col, dr, dc, player)
                + self.count_dir(row, col, -dr, -dc, player);
            count >= 5
        })
    }

    fn count_dir(&self, row: usize, col: usize, dr: isize, dc: isize, player: Player) -> usize {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut count = 0usize;

        while r >= 0 && c >= 0 && (r as usize) < self.size && (c as usize) < self.size {
            if self.board[r as usize][c as usize] == Some(player) {
                count += 1;
                r += dr;
                c += dc;
            } else {
                break;
            }
        }

        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceError {
    OutOfBounds,
    Occupied,
    GameOver,
}
