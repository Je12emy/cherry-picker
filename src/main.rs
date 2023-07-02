use std::io;

fn main() {
    let options = vec!["Option A", "Option B"];
    let mut selected_options_indexes: Vec<usize> = vec![];
    let mut current_selection_index: usize = 0;
    loop {
        let mut input = String::new();
        println!("Use J or UP ARROW and K or DOWN ARROW to select an item:");
        for (i, value) in options.iter().enumerate() {
            if selected_options_indexes.contains(&i) {
                print!("[x] ");
            } else {
                print!("[ ] ");
            }
            print!("{}", value);
            if i == current_selection_index {
                println!(" <")
            } else {
                println!("");
            }
        }
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().chars().next();
        if let Some(c) = input {
            if c == 'j' {
                if (current_selection_index + 1) <= options.len() {
                    println!("DOWN");
                    current_selection_index = current_selection_index + 1;
                }
            } else if c == 'k' {
                if (current_selection_index - 1) >= 0 {
                    println!("UP");
                    current_selection_index = current_selection_index - 1;
                }
            } else {
                println!("Not a valid option");
                break;
            }
        }
    }
}
