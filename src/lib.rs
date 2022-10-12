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
    pub fn get_constraints<'a>(&self, board: &'a Sudoku) -> impl Iterator<Item = u8> + 'a {
        board
            .iter()
            .filter(|c| if let Some(_) = c.value { true } else { false })
            .map(|c| c.value.unwrap())
    }
}

pub struct Sudoku {
    cells: Vec<Cell>,
}

impl Sudoku {
    fn from_str(str: &str) -> Self {
        if str.trim().chars().count() != 81 {
            panic!("Sudoku str size was not 81.");
        }
        if str.trim().contains(|c: char| {
            if c.is_digit(10) {
                let c = c.to_digit(10).unwrap();
                if c == 0 || c > 9 {
                    true
                } else {
                    false
                }
            } else if c != '.' {
                true
            } else {
                false
            }
        }) {
            panic!("Sudoku str contains invalid characters.");
        }
        Self {
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
        }
    }
    fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_converts_to_index() {
        let p = Pos::new(5, 3);
        assert_eq!(p.to_index(), 32);
    }
    fn pos_from_index_is_correct() {
        let p = Pos::from_index(32);
        let p2 = Pos::new(5, 3);
        assert_eq!(p, p2);
    }
}
