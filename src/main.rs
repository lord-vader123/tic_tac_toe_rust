use std::io;

use board::Board;
use render::draw;

mod board;
mod render;

fn main() {
    let mut board = Board::new(3);

    let mut input_line = String::new();

    let mut current_symbol: char = 'X';
    loop {
        draw(&board);

        println!("Please type row and column numbers(e.g. '1 2'): ");

        input_line.clear();

        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read input");

        let parts: Vec<&str> = input_line.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input. Please enter two numbers");
            continue;
        }
        let row: usize = match parts[0].parse::<i32>() {
            Ok(num) if num >= 0 => num as usize,
            Ok(_) => {
                println!("Row number cannot be negative.");
                continue;
            }
            Err(_) => {
                println!("Invalid row number.");
                continue;
            }
        };

        let column: usize = match parts[1].parse::<i32>() {
            Ok(num) if num >= 0 => num as usize,
            Ok(_) => {
                println!("Column number cannot be negative.");
                continue;
            }
            Err(_) => {
                println!("Invalid column number.");
                continue;
            }
        };

        if row > 2 || column > 2 {
            println!("Invalid numbers. Please choose between 0 and 2");
            continue;
        }

        match board.set_symbol(row, column, current_symbol) {
            Ok(_) => {
                if board.is_win() {
                    draw(&board);
                    println!("Player {} wins!", current_symbol);
                    break;
                }

                current_symbol = if current_symbol == 'X' { 'O' } else { 'X' };
            }
            Err(_) => {
                println!("Invalid position. Please choose a different spot.");
            }
        }
    }
}
