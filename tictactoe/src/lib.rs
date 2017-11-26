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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum Outcome {
    O = 1,
    Draw,
    X,
}

#[derive(Copy)]
pub struct BoardState {
    board: [[Piece; 3]; 3],
    pub turn: Player,
    pub victor: Outcome,
    pub ended: bool
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
            victor : self.victor.clone(),
            ended: self.ended.clone()
        }
    }
}

impl BoardState {
    pub fn new_board() -> BoardState{
        let board = [[Piece::Empty; 3]; 3];
        let turn = Player::X;
        let victor = Outcome::Draw;
        let ended = false;
        BoardState { board, turn, victor, ended }
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
        if let Piece::Empty = self.board[coord.0][coord.1] {
            let mut new_board = self.board.clone();
            new_board[coord.0][coord.1] = self.turn.player_to_piece();
            let new_turn = match self.turn {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            let ended = !new_board.iter()
                .flat_map(|b| b.iter())
                .any(|&v| v == Piece::Empty);
            let mut new_state = BoardState { 
                board: new_board,
                turn: new_turn,
                victor: Outcome::Draw,
                ended: ended };
            //TODO: this is a bit awkward, calling a method on a struct that
            //isn't really initialized
            new_state.victor = new_state.check_any_victory(&coord);
            if new_state.victor != Outcome::Draw{
                new_state.ended = true;
            }
            Some(new_state)
        } else {
            None
        }
    }

    pub fn check_any_victory(&self, coord: &(usize, usize)) -> Outcome {
        let dirs = [(1,0),(1,1),(0,1),(-1,1)];
        for dir in dirs.iter(){
            let victor = self.check_victory(coord, dir);
            if let Outcome::Draw = victor {
            } else {
                return victor
            }
        }
        Outcome::Draw
    }

    fn check_victory(&self, coord: &(usize, usize), dir: &(i32, i32)) -> Outcome {
        //find start
        let mut start = (coord.0 as i32, coord.1 as i32);
        if dir.0 != 0 {
            let mut projected = start.0;
            loop {
                let new_projected = projected - dir.0;
                if new_projected < 0 || new_projected > 2{
                    break;
                }
                projected = new_projected;
            }
            start.0 = projected;
        }
        if dir.1 != 0 {
            let mut projected = start.1;
            loop {
                let new_projected =  projected - dir.1;
                if new_projected < 0 || new_projected > 2{
                    break;
                }
                projected = new_projected;
            }
            start.1 = projected;
        }
        let cells : Vec<(usize, usize)> = (0..3).map(|i| (start.0 + i * dir.0, start.1 + i * dir.1))
            .filter(|&(x,y)| x >= 0 && x < 3 && y >= 0 && y < 3)
            .map(|(x,y)| (x as usize, y as usize))
            .collect();
        if cells.len() != 3 {
            return Outcome::Draw
        }
        let values : Vec<_> = cells.iter().map(|&(x,y)| self.board[x][y]).collect();
        let prospective_winner = values[0];
        if values.iter().all(|&x| x == prospective_winner) {
            return match prospective_winner {
                Piece::X => Outcome::X,
                Piece::O => Outcome::O,
                Piece::Empty => Outcome::Draw
            }
        }
        Outcome::Draw
    }
}

pub struct GameTree {
    children: Vec<Box<GameTree>>,
    play: Option<(usize, usize)>,
    state: BoardState,
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
        let (branch, result) = self.search();
        match branch.last() {
            None => None,
            Some(b) => *b
        }
    }

    fn search(&mut self) -> (Vec<Option<(usize, usize)>>, Outcome) {
        if self.state.ended {
            let moves = vec![self.play];
            return (moves, self.state.victor)
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

        ordered.sort_unstable_by_key(|r| r.1);
        let result = match self.state.turn {
            Player::X => ordered.last(),
            Player::O => ordered.first()
        };

        match result {
            //no legal moves left
            //i believe this is never hit?
            None => {
                let moves = vec![self.play];
                return (moves, self.state.victor)
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
