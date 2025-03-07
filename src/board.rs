pub struct Board {
    elements: [ElementShape; 9],
    cur_player: ElementShape,
}
impl Board {
    pub fn new() -> Board {
        Board {
            elements: [ElementShape::Empty; 9],
            cur_player: ElementShape::X,
        }
    }
    pub fn state_str(&self) -> String {
        self.elements
            .iter()
            .enumerate()
            .map(|(i, element)| match element {
                ElementShape::Empty => format!("[{}]", i.to_string()),
                _ => format!(" {} ", element.value().to_string()),
            }+ if i%3==2{"\n"}else{""} )
            .collect()
    }
    pub fn is_full(&self) -> bool {
        self.elements
            .iter()
            .all(|element| element != &ElementShape::Empty)
    }
    pub fn set_element(&mut self, index: usize) -> Result<(), ()> {
        if self.elements[index] != ElementShape::Empty {
            return Err(());
        }
        self.elements[index] = self.cur_player;
        self.switch_player();
        return Ok(());
    }
    fn switch_player(&mut self) {
        match self.cur_player {
            ElementShape::X => self.cur_player = ElementShape::O,
            ElementShape::O => self.cur_player = ElementShape::X,
            _ => {}
        }
    }
    pub fn get_winner(&self) -> Option<String> {
        let win_patterns = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // horizontal
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // vertical
            [0, 4, 8],
            [2, 4, 6], // diagonal
        ];
        for pattern in win_patterns.iter() {
            let first = self.elements[pattern[0]];
            if first == ElementShape::Empty {
                continue;
            }
            if self.elements[pattern[1]] == first && self.elements[pattern[2]] == first {
                return Some(first.value().to_string());
            }
        }
        None
    }
}
#[derive(Clone, Copy, PartialEq)]
enum ElementShape {
    Empty,
    X,
    O,
}
impl ElementShape {
    fn value(&self) -> &'static str {
        match self {
            ElementShape::X => "X",
            ElementShape::O => "O",
            ElementShape::Empty => "",
        }
    }
}
