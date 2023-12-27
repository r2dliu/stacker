use ndarray::Array2;

pub struct StackerGame {
    started: bool,
    board: Board,
    players: Vec<Player>,
}

pub struct Board {
    rows: i32,
    cols: i32,
    board: ndarray::Array2<i32>,
}

impl Board {
    pub fn new(rows: i32, cols: i32) -> Board {
        let board = Array2::zeros((rows as usize, cols as usize));

        Board {
            rows: rows,
            cols: cols,
            board: board,
        }
    }

    pub fn get_board(self) -> ndarray::Array2<i32> {
        self.board
    }
}

struct Player {
    isWinner: Option<bool>,
}
