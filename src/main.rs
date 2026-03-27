const SIZE: usize = 10;

fn main() {
    let mut player1 = Player {
        name: String::from("Joe"),
        board: [[0; SIZE]; SIZE],
    };
    let player2 = Player {
        name: String::from("Computer"),
        board: [[0; SIZE]; SIZE],
    };

    player1.place_ship(1, 1, 3);
    println!("Player {}, with board {:?}", player1.name, player1.board);
    println!("Player {}, with board {:?}", player2.name, player2.board);
}

struct Player {
    name: String,
    board: [[u32; SIZE]; SIZE],
}

impl Player {
    fn place_ship(&mut self, row: usize, col: usize, len: usize) {
        let end_col = col + len;
        for c in col..end_col {
            self.board[row][c] = 1;
        }
    }
}

fn attack(row: usize, col: usize, board: &mut [[u32; SIZE]; SIZE]) {
    board[row][col] = 1;
}
