mod board;
use board::Board;
use std::io::{self, Write};

fn clear_screen() {
    print!("\x1B[H\x1B[J"); // Move cursor to top-left and clear screen
    io::stdout().flush().unwrap();
}

fn main() {
    clear_screen();
    let mut board: Board = Board::new();
    println!("{}", board.state_str());
    while !board.is_full() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().unwrap();
        if let Err(()) = board.set_element(index) {
            println!("Invalid move: Already filled");
            continue;
        };
        clear_screen();
        println!("{}", board.state_str());
        if let Some(winner) = board.get_winner() {
            println!("Winner: {}", winner);
            break;
        }
    }
    println!("It's a draw!");
}
