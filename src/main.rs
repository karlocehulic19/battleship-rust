mod game;
use game::{Board, Player, ShipOrientation};

fn main() {
    let mut player1 = Player {
        name: String::from("Joe"),
        board: Board::new(),
    };
    let player2 = Player {
        name: String::from("Computer"),
        board: Board::new(),
    };

    let place1 = player1.place_ship(1, 1, 6, ShipOrientation::Vertical);
    let place2 = player1.place_ship(2, 2, 3, ShipOrientation::Horizontal);
    let place3 = player1.place_ship(1, 1, 3, ShipOrientation::Vertical);
    match place1 {
        Ok(()) => {
            println!("NO!, should fail!!");
        }
        Err(msg) => {
            println!("ERROR: {:?}", msg);
        }
    }
    // player1.place_ship(1, 1, 3, ShipOrientation::Vertical);
    println!("Player {}, with board {:?}", player1.name, player1.board);
    println!("Player {}, with board {:?}", player2.name, player2.board);
}
