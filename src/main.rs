mod table;
use std::io::{self, Write};
use table::Table;

fn clear_screen() {
    print!("\x1B[H\x1B[J"); // Move cursor to top-left and clear screen
    io::stdout().flush().unwrap();
}

fn main() {
    clear_screen();
    let mut table: Table = Table::new();
    println!("{}", table.state_str());
    while !table.is_full() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let index: usize = input.trim().parse().unwrap();
        if let Err(()) = table.set_element(index) {
            println!("Invalid move: Already filled");
            continue;
        };
        clear_screen();
        println!("{}", table.state_str());
        if let Some(winner) = table.get_winner() {
            println!("Winner: {}", winner);
            break;
        }
    }
    println!("It's a draw!");
}
