use std::{error::Error, fmt::Display};

#[derive(Clone, Debug)]
pub struct Board {
    elements: [ElementShape; 9],
    cur_player: ElementShape,
    turn_no: u8,
}
type Outcome = ElementShape;
type Move = usize;
impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
impl Board {
    pub fn new() -> Board {
        Board {
            elements: [ElementShape::Empty; 9],
            cur_player: ElementShape::X,
            turn_no: 1,
        }
    }
    pub fn from_move_sequence(
        move_seq: impl Iterator<Item = Move>,
    ) -> Result<Board, Box<dyn Error>> {
        let mut board = Board::new();
        for move_ele in move_seq {
            board.set_element(move_ele)?
        }
        Ok(board)
    }
    pub fn get_cur_player(&self) -> ElementShape {
        self.cur_player
    }
    pub fn state_str(&self) -> String {
        self.elements
            .iter()
            .enumerate()
            .map(|(i, element)| match element {
                ElementShape::Empty => format!("[{}]", i),
                _ => format!(" {} ", element.value()),
            }+ if i%3==2{"\n"}else{""} )
            .collect()
    }
    pub fn is_full(&self) -> bool {
        self.elements
            .iter()
            .all(|element| element != &ElementShape::Empty)
    }
    pub fn make_move(&mut self, index: Move) -> Result<Option<Outcome>, String> {
        self.set_element(index)?;
        Ok(self.get_winner())
    }
    pub fn set_element(&mut self, index: Move) -> Result<(), String> {
        if index > 8 {
            Err(String::from("Input has to be less than 8."))
        } else if self.elements[index] != ElementShape::Empty {
            Err(format!("Position {} already taken try again.", index))
        } else {
            self.elements[index] = self.cur_player;
            self.switch_player();
            Ok(())
        }
    }
    fn switch_player(&mut self) {
        match self.cur_player {
            ElementShape::X => self.cur_player = ElementShape::O,
            ElementShape::O => self.cur_player = ElementShape::X,
            _ => {}
        }
        self.turn_no += 1;
    }
    const WIN_PATTERNS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8], // horizontal
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8], // vertical
        [0, 4, 8],
        [2, 4, 6], // diagonal
    ];
    pub fn get_winner(&self) -> Option<ElementShape> {
        for pattern in Board::WIN_PATTERNS.iter() {
            let first = self.elements[pattern[0]];
            if first == ElementShape::Empty {
                continue;
            }
            if self.elements[pattern[1]] == first && self.elements[pattern[2]] == first {
                return Some(first);
            }
        }
        if !self.winning_is_possible() {
            return Some(ElementShape::Empty);
        }
        None
    }
    fn winning_is_possible(&self) -> bool {
        let mut board_clone = self.clone();
        board_clone.winning_is_possible_helper()
    }
    /// backtracing helper function
    fn winning_is_possible_helper(&mut self) -> bool {
        // I can prob terminate early with if stmt on self.turn_no but
        // who cares
        for i in self.get_all_valid_moves() {
            self.set_element(i).unwrap();
            for pattern in Board::WIN_PATTERNS.iter() {
                let first = self.elements[pattern[0]];
                if first == ElementShape::Empty {
                    continue;
                }
                if self.elements[pattern[1]] == first && self.elements[pattern[2]] == first {
                    return true;
                }
            }
            if self.winning_is_possible_helper() {
                return true;
            }
            // backtrack
            self.undo_move(i);
        }
        false
    }
    fn undo_move(&mut self, last_move: Move) {
        self.elements[last_move] = ElementShape::Empty;
        self.switch_player();
        self.turn_no -= 2;
    }
    fn get_all_valid_moves(&self) -> Vec<Move> {
        self.elements
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if *e == ElementShape::Empty {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    // currently random move
    pub fn play_computer_move(&mut self) {
        let next_move = self.calculate_best_move();
        let _ = self.set_element(next_move);
    }

    // return best move to take
    // if multiple moves win then pick smallest in number
    // if no moves win pick move draws
    // if no moves draw then pick first losing move
    fn calculate_best_move(&self) -> Move {
        let mut move_and_outcome: Vec<_> = Vec::new();
        for valid_move in self.get_all_valid_moves() {
            let move_outcome = self.evaluate_move(valid_move);
            if move_outcome == self.cur_player {
                return valid_move;
            }
            move_and_outcome.push((valid_move, move_outcome));
        }
        for (valid_move, move_outcome) in &move_and_outcome {
            if *move_outcome == Outcome::Empty {
                return *valid_move;
            }
        }
        move_and_outcome[0].0
    }

    // evaluate if a move is winning, drawing, or losing
    fn evaluate_move(&self, move_to_make: Move) -> Outcome {
        let mut copy_board = self.clone();
        copy_board.evaluate_move_helper(move_to_make)
    }
    // Minimax function
    // A more general form of this function is to give values
    // for winning or losing and making one player try to maximize
    // while player B try to minimize and so we can assign a value for each state
    // For example: we can assign X winning as 1 drawing as 0 and losing as -1
    // and thus the initial state in tic-tac-toe is equal to 0 (always a draw)
    fn evaluate_move_helper(&mut self, move_to_make: Move) -> Outcome {
        match self.make_move(move_to_make) {
            Err(_) => unreachable!(),
            Ok(Some(outcome)) => outcome,
            Ok(None) => {
                let mut opp_can_win = false;
                let mut opp_can_draw = false;
                let mut opp_can_lose = false;
                let mut outcomes = Vec::new();
                for valid_move in self.get_all_valid_moves() {
                    outcomes.push(self.evaluate_move_helper(valid_move));
                    self.undo_move(valid_move);
                }
                // notice now that current player is the person we play against
                for outcome in outcomes {
                    match outcome {
                        Outcome::Empty => opp_can_draw = true,
                        val if val == self.cur_player => opp_can_win = true,
                        _ => opp_can_lose = true,
                    }
                }
                match (opp_can_win, opp_can_draw, opp_can_lose) {
                    (true, _, _) => self.cur_player,
                    (false, true, _) => Outcome::Empty,
                    (false, false, true) => self.cur_player.opposite(),
                    _ => unreachable!(),
                }
            }
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum ElementShape {
    #[default]
    Empty,
    X,
    O,
}
impl ElementShape {
    pub fn value(&self) -> &'static str {
        match self {
            ElementShape::X => "X",
            ElementShape::O => "O",
            ElementShape::Empty => "",
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            ElementShape::Empty => ElementShape::Empty,
            ElementShape::X => ElementShape::O,
            ElementShape::O => ElementShape::X,
        }
    }
}
impl Display for ElementShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_normal_game() {
        let mut board = Board::new();
        assert_eq!(board.get_all_valid_moves(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
        /*
         *  X  O  X
         *  O  X  O
         *  X [7][8]
         *  */
        for i in 0..6 {
            let e = board.set_element(i);
            assert!(e.is_ok());
            assert!(board.get_winner().is_none());
            assert!(board.winning_is_possible());
            assert_eq!(board.get_all_valid_moves(), Vec::from_iter(i + 1..=8))
        }
        assert!(board.set_element(6).is_ok());
        assert_eq!(board.get_winner().unwrap(), ElementShape::X);
    }
    #[test]
    fn new_game_is_winnable() {
        let board = Board::new();
        assert!(board.winning_is_possible());
    }

    #[test]
    fn create_board_from_move_seq() {
        let seq = vec![0_usize, 1, 2, 3, 4, 5].into_iter();
        let mut board = Board::from_move_sequence(seq).unwrap();
        assert!(board.set_element(6).is_ok());
        assert_eq!(board.get_winner().unwrap(), ElementShape::X);
    }

    #[test]
    fn win_in_one_move() {
        let seq = vec![0_usize, 1, 2, 3, 4, 5].into_iter();
        let mut board = Board::from_move_sequence(seq).unwrap();
        assert_eq!(board.calculate_best_move(), 6);
        /*
         * winning move is 6
         *  X  O  X
         *  O  X  O
         * [6][7][8]
         *  */
        board.set_element(7).unwrap();
        board.set_element(8).unwrap();
        assert_eq!(board.calculate_best_move(), 6)
    }
    #[test]
    fn evaluate_win_in_one_move() {
        let seq = vec![0_usize, 1, 2, 3, 4, 5].into_iter();
        let mut board = Board::from_move_sequence(seq).unwrap();
        assert_eq!(board.evaluate_move(6), Outcome::X);
        assert_eq!(board.evaluate_move(7), Outcome::X);
        assert_eq!(board.evaluate_move(8), Outcome::X);
        board.set_element(7).unwrap();
        assert_eq!(board.evaluate_move(6), Outcome::X);
    }
    #[test]
    fn predict_loss_from_two_moves() {
        let seq = vec![0_usize, 1, 4].into_iter();
        let board = Board::from_move_sequence(seq).unwrap();
        assert_eq!(board.evaluate_move(3), Outcome::X);
        assert_eq!(board.evaluate_move(8), Outcome::X);
        /*
         * best move is 8 to avoid losing
         * but game is lost anyways
         *  X  O [2]
         * [3] X [5]
         * [6][7][8]
         *  */
    }
    #[test]
    fn win_from_opp_misplay() {
        let seq = vec![4_usize, 0, 5, 3, 7].into_iter();
        let board = Board::from_move_sequence(seq).unwrap();
        /*
         * best move is 8 to avoid losing
         * but game is lost anyways
         *  O [1][2]
         *  O  X  X
         * [6] X [8]
         *  */
        assert_eq!(board.evaluate_move(6), Outcome::O);
        assert_eq!(board.evaluate_move(1), Outcome::O);
        assert_eq!(board.cur_player, ElementShape::O);
        // the following fails because it also wins from [1]
        //assert_eq!(board.calculate_best_move(), 6);
    }

    #[test]
    fn perfect_game_always_draw() {
        let board = Board::new();
        for valid_move in board.get_all_valid_moves() {
            assert_eq!(board.evaluate_move(valid_move), Outcome::Empty);
        }
    }
}
