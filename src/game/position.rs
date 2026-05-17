use std::num::TryFromIntError;

use crate::general::{
    constants::{BOX_HEIGHT, BOX_WIDTH},
    movements::Movement,
};

use faer::{Mat, col, mat};

pub type CellPosition = (usize, usize);
pub type CellPositions = Vec<CellPosition>;

#[derive(Debug)]
pub enum PositionError {
    InvalidOffset,
    MainOutOfBounds,
}

impl From<TryFromIntError> for PositionError {
    fn from(_: TryFromIntError) -> Self {
        PositionError::InvalidOffset
    }
}

#[derive(Debug, Clone)]
pub struct BlockRelativePosition {
    main_cell: CellPosition,
    other_cells: CellPositions,
    offset_cells: Vec<(isize, isize)>,
}

impl BlockRelativePosition {
    pub fn new(
        main_cell: CellPosition,
        offset_cells: Vec<(isize, isize)>,
    ) -> Result<Self, PositionError> {
        let (main_row, main_col) = main_cell;
        let mut other_cells: CellPositions = vec![main_cell.clone()];
        for (off_row, off_col) in offset_cells.clone() {
            let row_isize = off_row + isize::try_from(main_row)?;
            let col_isize = off_col + isize::try_from(main_col)?;
            let col_usize: usize = col_isize.try_into()?;
            // TODO: implement custom type that only allows integers between board range
            if col_usize == BOX_WIDTH {
                return Err(PositionError::InvalidOffset);
            }

            other_cells.push((row_isize.try_into()?, col_usize));
        }

        return Ok(Self {
            main_cell,
            other_cells,
            offset_cells,
        });
    }

    pub fn move_block(&mut self, movement: &Movement) -> Result<Self, PositionError> {
        match movement {
            Movement::Left => {
                let (main_row, main_col) = &self.main_cell;
                if *main_col == 0 {
                    return Err(PositionError::MainOutOfBounds);
                }

                let new_main_cell = (main_row.clone(), main_col.clone() - 1);
                return Self::new(new_main_cell, self.offset_cells.clone());
            }

            Movement::Right => {
                let (main_row, main_col) = &self.main_cell;
                if *main_col == BOX_WIDTH - 1 {
                    return Err(PositionError::MainOutOfBounds);
                }

                let new_main_cell = (main_row.clone(), main_col.clone() + 1);
                return Self::new(new_main_cell, self.offset_cells.clone());
            }

            Movement::Down => {
                let (main_row, main_col) = &self.main_cell;
                if *main_row == BOX_HEIGHT - 1 {
                    return Err(PositionError::MainOutOfBounds);
                }

                let new_main_cell = (main_row.clone() + 1, main_col.clone());
                return Self::new(new_main_cell, self.offset_cells.clone());
            }
            Movement::ClockwiseRotation => {
                let clockwise_mat = mat![[0.0, 1.0], [-1.0, 0.0]];
                let rotated_oddset_cells: Vec<(isize, isize)> = self
                    .offset_cells
                    .iter()
                    .map(|cell| {
                        return Self::rotate_offset(
                            cell.0.clone() as f64,
                            cell.1.clone() as f64,
                            &clockwise_mat,
                        );
                    })
                    .collect();

                return Self::new(self.main_cell.clone(), rotated_oddset_cells.into());
            }
            Movement::AntiClockwiseRotation => {
                let anticlockwise_mat = mat![[0.0, -1.0], [1.0, 0.0]];
                let rotated_oddset_cells: Vec<(isize, isize)> = self
                    .offset_cells
                    .iter()
                    .map(|cell| {
                        return Self::rotate_offset(
                            cell.0.clone() as f64,
                            cell.1.clone() as f64,
                            &anticlockwise_mat,
                        );
                    })
                    .collect();

                return Self::new(self.main_cell.clone(), rotated_oddset_cells.into());
            }
        }
    }

    fn rotate_offset(o_row: f64, o_col: f64, rotation_matrix: &Mat<f64>) -> (isize, isize) {
        let offset_vec = col![o_row, o_col];
        let rotated_offset_vec = rotation_matrix * &offset_vec;
        return (
            rotated_offset_vec[0] as isize,
            rotated_offset_vec[1] as isize,
        );
    }
}

impl From<BlockRelativePosition> for CellPositions {
    fn from(value: BlockRelativePosition) -> Self {
        return value.other_cells.clone();
    }
}
