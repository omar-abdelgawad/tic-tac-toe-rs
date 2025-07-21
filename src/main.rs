mod cli;
use cli::get_args;
use std::io::{self, Write};
use tic_tac_toe_rs::board::Board;
use tic_tac_toe_rs::board::ElementShape;

fn clear_screen() {
    print!("\x1B[H\x1B[J"); // Move cursor to top-left and clear screen
    io::stdout().flush().unwrap();
}
fn main() {
    let args = get_args();
    clear_screen();
    let mut board = Board::new();
    let mut winner = ElementShape::default();
    let mut std_in_input = String::new();
    println!("{}", board.state_str());
    while !board.is_full() {
        std_in_input.clear();
        if args.computer {
            println!("Playing against computer.");
        }
        println!("It is {}'s turn", board.get_cur_player().value());
        std::io::stdin().read_line(&mut std_in_input).unwrap();
        let index = std_in_input.trim().parse::<usize>();
        if let Err(e) = index {
            println!("Please input an integer from 0 to 8. Error: {}", e);
            continue;
        }
        let index = index.unwrap();
        match board.make_move(index) {
            Err(e) => {
                println!("Invalid move: {}", e);
                continue;
            }
            Ok(Some(outcome)) => {
                winner = outcome;
                break;
            }
            Ok(None) => {}
        }
        if args.computer {
            println!("Playing computer's move!");
            board.play_computer_move();
            if let Some(cur_winner) = board.get_winner() {
                winner = cur_winner;
                break;
            }
        }
        clear_screen();
        println!("{}", board.state_str());
    }
    clear_screen();
    println!("{}", board.state_str());
    match winner {
        ElementShape::Empty => println!("It's a draw!"),
        _ => println!("Winner: {}", winner.value()),
    }
}
