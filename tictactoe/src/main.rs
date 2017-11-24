extern crate tictactoe;

fn main() {
    let mut board = tictactoe::BoardState::new_board();
    println!("{}", board);
    while board.legal_moves().len() > 0 {
        match board.victor {
            None => (),
            _ => break
        };
        let mut tree = tictactoe::GameTree::new(board.clone(), None);
        let play = tree.determine_move();
        match play {
            None => {
                println!("AI chose no play");
                println!("{}", board);
                println!("{:?}", board.victor);
                break
            }
            Some(p) => {
                board = board.make_move(&p).unwrap();
                println!("{}", board);
                println!("{:?}", board.victor);
            }
        }
    }
    println!("{}", board);
    println!("{:?}", board.victor);
}
