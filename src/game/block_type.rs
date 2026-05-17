use crate::general::colors::Color;

pub trait BlockType {
    fn get_offset_position(&self) -> Vec<(isize, isize)>;
    fn get_color(&self) -> Color;
}

pub struct LBlock {}

impl BlockType for LBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(0, -1), (1, -1), (0, 1)];
    }
    fn get_color(&self) -> Color {
        return Color::Green;
    }
}

pub struct JBlock {}

impl BlockType for JBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(0, -1), (1, 1), (0, 1)];
    }
    fn get_color(&self) -> Color {
        return Color::Red;
    }
}

pub struct SquareBlock {}

impl BlockType for SquareBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(0, 1), (1, 0), (1, 1)];
    }
    fn get_color(&self) -> Color {
        return Color::Yellow;
    }
}

pub struct TBlock {}

impl BlockType for TBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(1, 0), (0, -1), (0, 1)];
    }
    fn get_color(&self) -> Color {
        return Color::Blue;
    }
}

pub struct ZBlock {}

impl BlockType for ZBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(0, -1), (1, 0), (1, 1)];
    }
    fn get_color(&self) -> Color {
        return Color::Pink;
    }
}

pub struct SBlock {}

impl BlockType for SBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(0, 1), (1, 0), (1, -1)];
    }
    fn get_color(&self) -> Color {
        return Color::Orange;
    }
}

pub struct IBlock {}

impl BlockType for IBlock {
    fn get_offset_position(&self) -> Vec<(isize, isize)> {
        return vec![(-1, 0), (1, 0), (2, 0)];
    }
    fn get_color(&self) -> Color {
        return Color::Purple;
    }
}

pub struct BlockTypeRandomizer {}

impl BlockTypeRandomizer {
    pub fn get_random_block_type() -> Box<dyn BlockType> {
        let block_types: [fn() -> Box<dyn BlockType>; 7] = [
            || Box::new(LBlock {}),
            || Box::new(JBlock {}),
            || Box::new(TBlock {}),
            || Box::new(ZBlock {}),
            || Box::new(SBlock {}),
            || Box::new(SquareBlock {}),
            || Box::new(IBlock {}),
        ];

        let picked_index = rand::random_range(..block_types.len());
        return block_types[picked_index]();
    }
}
