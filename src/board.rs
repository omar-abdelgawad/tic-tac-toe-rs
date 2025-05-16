use std::{error::Error, fmt::Display};

#[derive(Clone, Debug)]
pub struct Board {
    elements: [ElementShape; 9],
    cur_player: ElementShape,
    turn_no: u8,
}
impl Board {
    pub fn new() -> Board {
        Board {
            elements: [ElementShape::Empty; 9],
            cur_player: ElementShape::X,
            turn_no: 1,
        }
    }
    //pub fn from_move_sequence(
    //    move_seq: impl Iterator<Item = usize>,
    //) -> Result<Board, Box<dyn Error>> {
    //    let mut board = Board::new();
    //    for move_ele in move_seq {
    //        board.set_element(move_ele)?
    //    }
    //    Ok(board)
    //}
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
    pub fn set_element(&mut self, index: usize) -> Result<(), String> {
        if index > 8usize {
            Err(String::from("Input has to be less than 8."))
        } else if self.elements[index] != ElementShape::Empty {
            Err(format!("Space {} already taken try again.", index))
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
        for i in 0..9 {
            if self.elements[i] == ElementShape::Empty {
                self.elements[i] = self.cur_player;
                self.switch_player();
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
                self.elements[i] = ElementShape::Empty;
                self.switch_player();
                self.turn_no -= 2;
            }
        }
        false
    }
    fn get_all_empty_spaces(&self) -> impl Iterator<Item = usize> + '_ {
        self.elements.iter().enumerate().filter_map(|(i, e)| {
            if *e == ElementShape::Empty {
                Some(i)
            } else {
                None
            }
        })
    }
    pub fn play_computer_move(&mut self) {
        use rand::rng;
        use rand::seq::IteratorRandom; // for `.choose()` method
        let mut rng = rng();
        if let Some(index) = self.get_all_empty_spaces().choose(&mut rng) {
            let _ = self.set_element(index);
        } else {
            eprintln!("Computer couldn't make a move!")
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
        for i in 0..6 {
            let e = board.set_element(i);
            assert!(e.is_ok());
            assert!(board.get_winner().is_none());
            assert!(board.winning_is_possible());
        }
        assert!(board.set_element(6).is_ok());
        assert_eq!(board.get_winner().unwrap(), ElementShape::X);
    }
}
