use ratatui::symbols::block;

use crate::general::colors::Color;

pub struct Board {
    pub blocks: [[Color; 10]; 10],
}

impl Board {
    pub fn new() -> Self {
        return Self {
            blocks: [[Color::Empty; 10]; 10],
        };
    }

    pub fn place_block(&mut self) {
        self.blocks[3][3] = Color::Green;
        self.blocks[4][3] = Color::Red;
        self.blocks[5][5] = Color::Blue;
        self.blocks[6][5] = Color::Yellow;
    }
}
