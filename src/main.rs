use board::Board;
use render::draw;

mod board;
mod render;

fn main() {
    let board = Board::new(3);

    draw(&board);
}
