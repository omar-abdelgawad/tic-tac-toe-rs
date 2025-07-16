mod board;
mod cli;
use board::Board;
use board::ElementShape;
use cli::get_args;
use std::io::{self, Write};

fn clear_screen() {
    print!("\x1B[H\x1B[J"); // Move cursor to top-left and clear screen
    io::stdout().flush().unwrap();
}
fn main() {
    let args = get_args();
    clear_screen();
    let mut board = Board::new();
    let mut winner = ElementShape::default();
    let mut input = String::new();
    println!("{}", board.state_str());
    while !board.is_full() {
        input.clear();
        if args.computer {
            println!("Playing against computer.");
        }
        println!("It is {}'s turn", board.get_cur_player().value());
        std::io::stdin().read_line(&mut input).unwrap();
        let index = input.trim().parse::<usize>();
        if let Err(e) = index {
            println!("Please input an integer from 0 to 8. Error: {}", e);
            continue;
        }
        let index = index.unwrap();
        if let Err(e) = board.set_element(index) {
            println!("Invalid move: {}", e);
            continue;
        };
        if args.computer {
            println!("Playing computer's move!");
            board.play_computer_move();
        }
        clear_screen();
        println!("{}", board.state_str());
        if let Some(cur_winner) = board.get_winner() {
            winner = cur_winner;
            break;
        }
    }
    match winner {
        ElementShape::Empty => println!("It's a draw!"),
        _ => println!("Winner: {}", winner.value()),
    }
}
