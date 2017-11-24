extern crate tictactoe;

fn main() {
    let moves = [(1,0), (1,1), (2,0), (1,2), (0,0)];
    let mut board = tictactoe::BoardState::new_board();
    println!("{}", board);
    for m in moves.iter(){
        board = board.make_move(*m).unwrap();
        println!("{}", board);
        println!("{:?}", board.victor);
    }
}
