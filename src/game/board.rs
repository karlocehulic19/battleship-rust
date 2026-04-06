use crate::ColorBox;
use crate::general::{
    colors::Color,
    dimensions::{BOX_HEIGHT, BOX_WIDTH},
};

#[derive(Default, Debug)]
pub struct Board {
    pub blocks: ColorBox,
    pub curr_block: (usize, usize),
}

impl Board {
    pub fn new() -> Self {
        return Self {
            blocks: [[Color::Empty; BOX_WIDTH]; BOX_HEIGHT],
            curr_block: (0, 0),
        };
    }

    pub fn next_move(&mut self) {
        let (r, c) = self.curr_block;
        if r == BOX_HEIGHT || !matches!(self.blocks[r][c], Color::Empty) {
            self.curr_block = (0, 0);
            return;
        }
        self.blocks[r][c] = Color::Red;
        if r > 0 {
            self.blocks[r - 1][c] = Color::Empty;
        }

        self.curr_block = (r + 1, c);
    }
}
