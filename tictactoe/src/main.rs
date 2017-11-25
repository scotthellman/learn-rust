extern crate tictactoe;

fn main() {
    let mut board = tictactoe::BoardState::new_board();
    println!("{}", board);
    board = board.make_move(&(0,0)).unwrap();
    board = board.make_move(&(1,0)).unwrap();
    board = board.make_move(&(2,0)).unwrap();
    board = board.make_move(&(0,1)).unwrap();
    board = board.make_move(&(1,2)).unwrap();
    board = board.make_move(&(2,1)).unwrap();
    while board.legal_moves().len() > 0 {
        if board.ended {
            break
        }
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
        break;
    }
    println!("{}", board);
    println!("{:?}", board.victor);
}
