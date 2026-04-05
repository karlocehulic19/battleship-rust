const SIZE: usize = 10;
type ShipSizesArray = [usize; 5];
const SHIP_SIZES: ShipSizesArray = [2, 3, 3, 4, 5];

pub struct Player {
    pub name: String,
    pub board: Board,
}

impl Player {
    // how to check for ships, if any is consumed drop otherwise place
    pub fn place_ship(
        &mut self,
        row: usize,
        col: usize,
        len: usize,
        orientation: ShipOrientation,
    ) -> Result<(), String> {
        if !self.board.ships_needed.contains(&len) {
            return Err("Wrong ship size to place".to_string());
        }

        match orientation {
            ShipOrientation::Horizontal => {
                let end_col = col + len;
                for c in col..end_col {
                    if self.board.cells[row][c] == 1 {
                        return Err("Cell already occuppied".to_string());
                    }
                }
                for c in col..end_col {
                    self.board.cells[row][c] = 1;
                }
                Ok(())
            }
            ShipOrientation::Vertical => {
                let end_row = col + len;
                for r in row..end_row {
                    if self.board.cells[r][col] == 1 {
                        return Err("Cell already occuppied".to_string());
                    }
                }
                for r in row..end_row {
                    self.board.cells[r][col] = 1;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Board {
    cells: [[u32; SIZE]; SIZE],
    ships_needed: ShipSizesArray,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            cells: [[0; SIZE]; SIZE],
            ships_needed: SHIP_SIZES.clone(),
        };
    }
}

pub enum ShipOrientation {
    Horizontal,
    Vertical,
}

fn attack(row: usize, col: usize, board: &mut [[u32; SIZE]; SIZE]) {
    board[row][col] = 1;
}
