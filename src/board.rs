enum GameError {
    InvalidInput,
}

struct Board {
    row: Vec<char>,
    column: Vec<char>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            row: vec!['_'; size],
            column: vec!['_'; size],
            size,
        }
    }

    pub fn set_symbol(&mut self, row: usize, column: usize, symbol: char) -> Result<(), GameError> {
        if row > self.size || column > self.size {
            return Err(GameError::InvalidInput);
        }

        self.row[row - 1] = symbol;
        self.column[column - 1] = symbol;

        Ok(())
    }
}
