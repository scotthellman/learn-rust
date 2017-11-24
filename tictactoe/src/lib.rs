//TODO: unify piece, player, and victory
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Piece {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone, Copy)]
pub enum Player {
    X,
    O
}

#[derive(Copy)]
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

impl Clone for BoardState {
    fn clone(&self) -> BoardState {
        BoardState {
            board : self.board.clone(),
            turn : self.turn.clone(),
            victor : self.victor.clone()
        }
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

    pub fn make_move(&self, coord: &(usize, usize)) -> Option<BoardState>{
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

pub struct GameTree {
    children: Vec<Box<GameTree>>,
    play: Option<(usize, usize)>,
    state: BoardState,
}

fn comp_hack(v: &(Vec<Option<(usize, usize)>>, Option<Player>)) -> i32 {
    match v.1 {
        None => 0,
        Some(p) => match p {
            Player::X => 1,
            Player::O => -1
        }
    }
}

impl GameTree {
    pub fn new(state: BoardState, play: Option<(usize, usize)>) -> GameTree {
        let children = Vec::with_capacity(4);
        GameTree {
            state,
            play,
            children
        }
    }

    pub fn determine_move(&mut self) -> Option<(usize, usize)> {
        let branch = self.search().0;
        match branch.get(1) {
            None => None,
            Some(b) => *b
        }
    }

    fn search(&mut self) -> (Vec<Option<(usize, usize)>>, Option<Player>) {
        if let Some(p) = self.state.victor {
            //TODO: bet this doesn't work
            let moves = vec![self.play];
            return (moves, Some(p))
        }
        if self.children.len() == 0 {
            let result: Vec<_> = {
                self.state.legal_moves()
                    .iter()
                    .map( |play| {
                        let state = self.state.make_move(play).unwrap();
                        Box::new(GameTree::new(state, Some(*play)))
                    })
                   .collect()
            };
            self.children.extend(result);
        }

        let mut ordered = Vec::new();
        for i in 0..self.children.len() {
            ordered.push(self.children[i].search())
        }

        ordered.sort_unstable_by_key(comp_hack);
        let mut result = match self.state.turn {
            Player::X => ordered.last(),
            Player::O => ordered.first()
        };

        //no legal moves left
        match result {
            None => {
                let moves = vec![self.play];
                return (moves, None)
            },
            Some(p) => {
                //this is bad
                let mut p_copy = p.clone();
                if let Some(this_play) = self.play {
                    p_copy.0.push(Some(this_play));
                }
                return p_copy
            }
        }
    }
}
