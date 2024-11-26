use crate::board::Board;

pub fn draw(board: &Board) {
    let fields = board.get_fields();

    for (i, row) in fields.iter().enumerate() {
        for cell in row {
            print!("| {} ", cell);
        }
        println!("|");

        if i < fields.len() - 1 {
            println!();
        }
    }
}
