use std::io;
use std::io::Write;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let options = vec!["Option A", "Option B"];

    let mut selected: Vec<usize> = vec![];
    let mut current_index: usize = 0;

    let stdin = io::stdin();
    let mut input = stdin.lock().keys();
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    loop {
        write!(
            stdout,
            "{} {}",
            termion::cursor::Goto(0, 2),
            termion::clear::AfterCursor
        )
        .unwrap();
        write!(stdout, "Available options:\r\n").unwrap();
        for (index, value) in options.iter().enumerate() {
            let selected = selected.contains(&index);
            let focused = index == current_index;
            let selected_indicator = if selected { "[x]" } else { "[ ]" };
            let focused_indicator = if focused { "<" } else { "" };
            write!(
                stdout,
                "{} {} {}\r\n",
                selected_indicator, value, focused_indicator,
            )
            .unwrap();
        }
        write!(
            stdout,
            "Use K or ↑ (UP ARROW) and J or ↓ (DOWN ARROW) to select or deselect an item.{}\r\n",
            termion::cursor::Hide,
        )
        .unwrap();
        stdout.flush().unwrap();
        if let Some(Ok(key_event)) = input.next() {
            match key_event {
                Key::Char('j') | Key::Down => {
                    if current_index < options.len() - 1 {
                        current_index += 1;
                    }
                }
                Key::Char('k') | Key::Up => {
                    if current_index > 0 {
                        current_index -= 1;
                    }
                }
                Key::Char(' ') => {
                    let is_selected = selected.contains(&current_index);
                    if is_selected {
                        if let Some(to_remove) = selected.iter().position(|x| x == &current_index) {
                            selected.remove(to_remove);
                        }
                    } else {
                        selected.push(current_index);
                    }
                }
                Key::Char('\n') => {
                    stdout.flush().unwrap();
                    write!(stdout, "{:?}\r\n", selected).unwrap();
                    break;
                }
                _ => {}
            }
        }
    }
}
