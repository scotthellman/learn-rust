#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Empty,
    X,
    O,
}

#[derive(Debug)]
pub enum Player {
    X,
    O
}

pub struct BoardState {
    board: [[Piece; 3]; 3],
    pub turn: Player,
    pub victor: Option<Player>
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
    pub fn new_board() -> BoardState{
        let board = [[Piece::Empty; 3]; 3];
        let turn = Player::X;
        let victor = None;
        BoardState { board, turn, victor }
    }

    pub fn legal_moves(&self) -> Vec<(usize, usize)> {
        let mut legal = Vec::with_capacity(9);
        for i in 0..3 {
            for j in 0..3 {
                if let Piece::Empty = self.board[i][j] {
                    legal.push((i,j));
                }
            }
        }
        legal
    }

    pub fn make_move(&self, coord: (usize, usize)) -> Option<BoardState>{
        let pos = &self.board[coord.0][coord.1];
        if let Piece::Empty = self.board[coord.0][coord.1] {
            let mut new_board = self.board.clone();
            new_board[coord.0][coord.1] = self.turn.player_to_piece();
            let new_turn = match self.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            //TODO: this is a bit awkward, calling a method on a struct that
            //isn't really initialized
            let mut new_state = BoardState { board: new_board, turn: new_turn, victor: None };
            new_state.victor = new_state.check_any_victory(&coord);
            Some(new_state)
        } else {
            None
        }
    }

    pub fn check_any_victory(&self, coord: &(usize, usize)) -> Option<Player> {
        let dirs = [(1,0),(1,1),(0,1)];
        for dir in dirs.iter(){
            let victor = self.check_victory(coord, dir);
            if let Some(p) = victor{
                return Some(p)
            }
        }
        None
    }

    fn check_victory(&self, coord: &(usize, usize), dir: &(usize, usize)) -> Option<Player> {
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
                return None
            }
            let new_val = self.board[new_x][new_y];
            if start_val != new_val {
                start_val = Piece::Empty;
            }
        }
        match start_val {
            Piece::Empty => None,
            Piece::X => Some(Player::X),
            Piece::O => Some(Player::O)
        }
    }
}

struct GameTree {
    children: Vec<Box<GameTree>>,
    state: BoardState,
}

impl GameTree {
    fn new(state: BoardState) -> GameTree {
        let children = Vec::with_capacity(4);
        GameTree {
            state,
            children
        }
    }
    /*
    fn search(&self) -> ((usize, usize), Option<Player>) {
        if self.children.len() == 0 {
            self.children = {
                self.state.legal_moves()
                    .map( |move| {
                        let state = 

                    });
            }

        }
    }
    */
}
