pub enum GameError {
    InvalidInput,
}

pub struct Board {
    fields: Vec<Vec<char>>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            fields: vec![vec!['_'; size]; size],
            size,
        }
    }

    pub fn set_symbol(&mut self, row: usize, column: usize, symbol: char) -> Result<(), GameError> {
        if row >= self.size || column >= self.size {
            return Err(GameError::InvalidInput);
        }

        self.fields[row][column] = symbol;

        Ok(())
    }

    pub fn is_win(&self) -> bool {
        if self.get_win_char() != '_' {
            return true;
        }
        false
    }

    pub fn get_win_char(&self) -> char {
        if let Some(symbol) = self.is_win_row() {
            return symbol;
        }

        if let Some(symbol) = self.is_win_column() {
            return symbol;
        }

        if let Some(symbol) = self.is_win_cross() {
            return symbol;
        }

        '_'
    }

    pub fn get_size(&self) -> usize {
        return self.size;
    }

    pub fn get_fields(&self) -> Vec<Vec<char>> {
        return self.fields.clone();
    }

    fn is_win_row(&self) -> Option<char> {
        for row in &self.fields {
            if row.iter().all(|&symbol| symbol != '_' && symbol == row[0]) {
                return Some(row[0]);
            }
        }
        None
    }

    fn is_win_column(&self) -> Option<char> {
        for column in 0..self.size {
            let symbol = self.fields[0][column];
            if symbol != '_' && (0..self.size).all(|row| self.fields[row][column] == symbol) {
                return Some(symbol);
            }
        }
        None
    }

    fn is_win_cross(&self) -> Option<char> {
        let symbol_main = self.fields[0][0];

        if symbol_main != '_' && (0..self.size).all(|i| self.fields[i][i] == symbol_main) {
            return Some(symbol_main);
        }

        let symbol_secondary = self.fields[0][self.size - 1];
        if symbol_secondary != '_'
            && (0..self.size).all(|i| self.fields[i][self.size - 1 - i] == symbol_secondary)
        {
            return Some(symbol_secondary);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = Board::new(3);
        assert_eq!(board.get_size(), 3);

        let board_2 = Board::new(10);
        assert_eq!(board_2.get_size(), 10);
    }

    #[test]
    fn test_set_symbol_valid() {
        let mut board = Board::new(3);
        let result = board.set_symbol(0, 0, 'X');
        assert!(result.is_ok());
        assert_eq!(board.fields[0][0], 'X');
    }

    #[test]
    fn test_win_row() {
        let mut board = Board::new(3);
        let _ = board.set_symbol(0, 0, 'X');
        let _ = board.set_symbol(0, 1, 'X');
        let _ = board.set_symbol(0, 2, 'X');

        assert_eq!(board.get_win_char(), 'X')
    }

    #[test]
    fn test_win_column() {
        let mut board = Board::new(3);
        let _ = board.set_symbol(0, 0, 'X');
        let _ = board.set_symbol(1, 0, 'X');
        let _ = board.set_symbol(2, 0, 'X');

        assert_eq!(board.get_win_char(), 'X')
    }

    #[test]
    fn test_win_cross_1() {
        let mut board = Board::new(3);
        let _ = board.set_symbol(0, 0, 'X');
        let _ = board.set_symbol(1, 1, 'X');
        let _ = board.set_symbol(2, 2, 'X');

        assert_eq!(board.get_win_char(), 'X')
    }

    #[test]
    fn test_win_cross_2() {
        let mut board = Board::new(3);
        let _ = board.set_symbol(0, 2, 'X');
        let _ = board.set_symbol(1, 1, 'X');
        let _ = board.set_symbol(2, 0, 'X');

        assert_eq!(board.get_win_char(), 'X')
    }

    #[test]
    fn test_get_win_char() {
        let board = Board::new(3);
        assert_eq!(board.get_win_char(), '_');
    }

    #[test]
    fn test_is_win() {
        let mut board = Board::new(3);
        let _ = board.set_symbol(0, 0, 'X');
        let _ = board.set_symbol(0, 1, 'X');
        let _ = board.set_symbol(0, 2, 'X');

        assert!(board.is_win());
    }
}
