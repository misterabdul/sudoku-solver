pub fn read_input() -> [[u8; 9]; 9] {
    const EMPTY_STRING: String = String::new();

    let mut results: [[u8; 9]; 9] = [[0; 9]; 9];
    let mut raw_inputs: [String; 9] = [EMPTY_STRING; 9];
    let mut row_count: u8;

    for i in 0..9 {
        if let Err(e) = std::io::stdin().read_line(&mut raw_inputs[i]) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
    for i in 0..9 {
        row_count = 0;
        for c in raw_inputs[i].chars() {
            if c >= '0' && c <= '9' {
                results[i][row_count as usize] = ((c as u8) - ('0' as u8)) as u8;
                row_count = row_count + 1;
                if row_count >= 9 {
                    break;
                };
            }
        }
    }

    return results;
}
