#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    Empty,
    X,
    O,
}

enum Player {
    X,
    O
}

struct BoardState {
    board: [[Piece; 3]; 3],
    turn: Player
}

impl Piece {
    fn to_str(&self) -> &str{
        match self {
            &Piece::Empty => " ",
            &Piece::X => "X",
            &Piece::O => "O"
        }
    }
}

impl Player {
    fn player_to_piece(&self) -> Piece {
        match self {
            &Player::X => Piece::X,
            &Player::O => Piece::O
        }
    }
}

impl std::fmt::Display for BoardState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::from("");
        for (i,row) in self.board.iter().enumerate() {
            if i > 0 {
                //TODO hardocded size assumption
                output.push_str("-----\n");
            }
            for (j,value) in row.iter().enumerate(){
                if j > 0 {
                    output.push_str("|");
                }
                output.push_str(value.to_str());
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}

impl BoardState {
    fn new_board() -> BoardState{
        let mut board = [[Piece::Empty; 3]; 3];
        let mut turn = Player::X;
        BoardState { board, turn }
    }

    fn make_move(&mut self, coord: (usize, usize)) -> Piece{
        let pos = self.board[coord.0][coord.1];
        match pos {
            Piece::Empty => {
                self.board[coord.0][coord.1] = self.turn.player_to_piece();
                match self.turn {
                    Player::X => self.turn = Player::O,
                    Player::O => self.turn = Player::X,
                }
            }
            _ => ()
        };
        self.check_any_victory(&coord)
    }

    fn check_any_victory(&self, coord: &(usize, usize)) -> Piece {
        let dirs = [(1,0),(1,1),(0,1)];
        for dir in dirs.iter(){
            let victor = self.check_victory(coord, dir);
            if let Piece::Empty = victor {
            } else {
                return victor
            }
        }
        Piece::Empty
    }

    fn check_victory(&self, coord: &(usize, usize), dir: &(usize, usize)) -> Piece {
        let mut start = coord.clone();
        while start.0 != 0 && start.1 != 0 {
            start.0 -= dir.0;
            start.1 -= dir.1;
        }
        let start = start;
        let mut start_val = self.board[start.0][start.1];
        for i in 1..3 {
            let new_x = start.0 + dir.0 * i;
            let new_y = start.1 + dir.1 * i;
            if new_x >= 3 || new_y >= 3 {
                return Piece::Empty;
            }
            let new_val = self.board[new_x][new_y];
            if start_val != new_val {
                start_val = Piece::Empty;
            }
        }
        start_val
    }
}

fn main() {
    let mut board = BoardState::new_board();
    println!("{}", board);
    println!("{:?}", board.make_move((1,0)));
    println!("{}", board);
    println!("{:?}", board.make_move((1,1)));
    println!("{}", board);
    println!("{:?}", board.make_move((2,0)));
    println!("{}", board);
    println!("{:?}", board.make_move((1,2)));
    println!("{}", board);
    println!("{:?}", board.make_move((0,0)));
    println!("{}", board);
}
