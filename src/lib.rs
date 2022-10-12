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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_converts_to_index() {
        let p = Pos::new(5, 3);
        assert_eq!(p.to_index(), 32);
    }
}
