mod checker;
mod collector;
mod reader;
mod solver;

fn main() {
    let base_puzzle = reader::read_input();

    match checker::check(&base_puzzle) {
        checker::Status::Solved => {
            print_puzzle(&base_puzzle);
            println!("\nAlready solved nyaa~");
            return;
        }
        checker::Status::Wrong => {
            println!("Something's wrong I can feel it nyaa~");
            return;
        }
        _ => {}
    }

    match solver::brute_force(base_puzzle, collector::collect_possibilities(base_puzzle)) {
        Ok(solved_puzzle) => {
            print_puzzle(&solved_puzzle);
            println!("\nI've done it nyaa~\nPraise me nyaa~");
        }
        Err(_) => {
            println!("Couldn\'t find any solutions nyaa~\nI've tried my best, feel free contact my creator nyaa~");
        }
    }
}

fn print_puzzle(puzzle: &[[u8; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            print!("{} ", (*puzzle)[i][j])
        }
        println!()
    }
}
