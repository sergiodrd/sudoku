use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    pub fn new(x: u8, y: u8) -> Self {
        if x > 8 || y > 8 {
            panic!("Position out of bounds.");
        }
        Self { x, y }
    }
    pub fn from_index(i: usize) -> Self {
        if i > 80 {
            panic!("Position index out of bounds.");
        }
        Self {
            x: (i % 9) as u8,
            y: (i / 9) as u8,
        }
    }
    pub fn to_index(&self) -> usize {
        (self.y * 9 + self.x) as usize
    }
    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
}

#[derive(Debug)]
pub struct Cell {
    value: Option<u8>,
    position: Pos,
}

impl Cell {
    pub fn new(value: Option<u8>, position: Pos) -> Self {
        if let Some(x) = value {
            if x == 0 || x > 9 {
                panic!("Cell number is invalid.");
            }
        }
        Self { value, position }
    }
    pub fn value(&self) -> Option<u8> {
        self.value
    }
    pub fn get_constraints<'a>(&self, board: &'a Sudoku) -> impl Iterator<Item = u8> + 'a {
        board
            .get_rest_of_row(self.position)
            .chain(board.get_rest_of_column(self.position))
            .chain(board.get_rest_of_box(self.position))
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

#[derive(Debug)]
pub struct Sudoku {
    cells: Vec<Cell>,
}

impl Sudoku {
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut()
    }
    pub fn get_rest_of_row(&'_ self, pos: Pos) -> impl Iterator<Item = u8> + '_ {
        self.iter()
            .filter(|&c| matches!(c.value, Some(_)))
            .filter(move |&c| c.position.y == pos.y && c.position.x != pos.x)
            .map(|c| c.value.unwrap())
    }
    pub fn get_rest_of_column(&'_ self, pos: Pos) -> impl Iterator<Item = u8> + '_ {
        self.iter()
            .filter(|&c| matches!(c.value, Some(_)))
            .filter(move |&c| c.position.x == pos.x && c.position.y != pos.y)
            .map(|c| c.value.unwrap())
    }
    pub fn get_rest_of_box(&'_ self, pos: Pos) -> impl Iterator<Item = u8> + '_ {
        let x = match pos.x {
            1..=2 => 0u8,
            3..=5 => 3,
            _ => 6,
        };
        let y = match pos.y {
            1..=2 => 0u8,
            3..=5 => 3,
            _ => 6,
        };
        self.iter()
            .filter(|&c| matches!(c.value, Some(_)))
            .filter(move |&c| {
                (x..=x + 2).contains(&c.position.x)
                    && (y..=y + 2).contains(&c.position.y)
                    && c.position != pos
            })
            .map(|c| c.value.unwrap())
    }
    pub fn get_cell_at_pos(&self, pos: Pos) -> &Cell {
        self
            .iter()
            .find(|c| c.position == pos)
            .unwrap()
    }
}

impl std::str::FromStr for Sudoku {
    type Err = &'static str;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.trim().chars().count() != 81 {
            return Err("Sudoku str size was not 81.");
        }
        if str.trim().contains(|c: char| {
            if c.is_ascii_digit() {
                let c = c.to_digit(10).unwrap();
                c == 0 || c > 9
            } else {
                c != '.'
            }
        }) {
            return Err("Sudoku str contains invalid characters.");
        }
        Ok(Self {
            cells: str
                .trim()
                .chars()
                .enumerate()
                .map(|c| match c {
                    (i, '1') => Cell::new(Some(1u8), Pos::from_index(i)),
                    (i, '2') => Cell::new(Some(2), Pos::from_index(i)),
                    (i, '3') => Cell::new(Some(3), Pos::from_index(i)),
                    (i, '4') => Cell::new(Some(4), Pos::from_index(i)),
                    (i, '5') => Cell::new(Some(5), Pos::from_index(i)),
                    (i, '6') => Cell::new(Some(6), Pos::from_index(i)),
                    (i, '7') => Cell::new(Some(7), Pos::from_index(i)),
                    (i, '8') => Cell::new(Some(8), Pos::from_index(i)),
                    (i, '9') => Cell::new(Some(9), Pos::from_index(i)),
                    (i, '.') => Cell::new(None, Pos::from_index(i)),
                    _ => unreachable!(),
                })
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn pos_converts_to_index() {
        assert_eq!(Pos::new(5, 3).to_index(), 32);
    }

    #[test]
    fn pos_from_index_is_correct() {
        assert_eq!(Pos::from_index(32), Pos::new(5, 3));
    }

    #[test]
    fn sudoku_can_get_rest_of_row() {
        let s = Sudoku::from_str(
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
        )
        .unwrap();
        assert_eq!(
            s.get_rest_of_row(Pos::new(5, 4)).collect::<Vec<_>>(),
            vec![9u8, 8, 2, 5]
        );
    }

    #[test]
    fn sudoku_can_get_rest_of_column() {
        let s = Sudoku::from_str(
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
        )
        .unwrap();
        assert_eq!(
            s.get_rest_of_column(Pos::new(5, 2)).collect::<Vec<_>>(),
            vec![3u8, 4, 7]
        );
    }

    #[test]
    fn sudoku_can_get_rest_of_box() {
        let s = Sudoku::from_str(
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
        )
        .unwrap();
        assert_eq!(
            s.get_rest_of_box(Pos::new(7, 1)).collect::<Vec<_>>(),
            vec![1u8, 7, 4, 6, 8]
        );
    }

    #[test]
    fn correct_constraints_from_position() {
        let s = Sudoku::from_str(
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
        )
        .unwrap();
        let mut constraints = s
            .get_cell_at_pos(Pos::new(7, 1))
            .get_constraints(&s)
            .collect::<Vec<_>>();
        constraints.sort();
        assert_eq!(constraints, vec![1u8, 4, 5, 6, 7, 8]);
    }
}
