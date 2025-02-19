/*
Runo, nano-like rust text editer.
By: Hadrian Lazic
Last Updated: 2/8/25
License Under: MIT

'########::'##::::'##:'##::: ##::'#######::
 ##.... ##: ##:::: ##: ###:: ##:'##.... ##:
 ##:::: ##: ##:::: ##: ####: ##: ##:::: ##:
 ########:: ##:::: ##: ## ## ##: ##:::: ##:
 ##.. ##::: ##:::: ##: ##. ####: ##:::: ##:
 ##::. ##:: ##:::: ##: ##:. ###: ##:::: ##:
 ##:::. ##:. #######:: ##::. ##:. #######::
..:::::..:::.......:::..::::..:::.......:::

*/

use std::env;
use std::io::Write;
use TerimalRtdm::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1..].join(" ");

    if args.len() - 1 != 1 {
        println!("Enter File path as argument!");
        std::process::exit(1);
    }

    if !file_path.contains(".") {
        println!("Add file extension!");
        std::process::exit(1);
    }

    clear();
    let mut app = App::new();

    let mut current_text: Vec<String> = vec![String::new()];
    let mut is_typing: bool = false;
    let mut current_line = 0;

    raw_line("┌───────────────────────────────────────────────┐");
    raw_line("│ Runo-Controls │ │ :q -> (Quit) | e -> Type    │");
    raw_line("│ Enter -> (Save) │ │ Use ↑ and ↓ arrows to move│");
    raw_line("└───────────────────────────────────────────────┘");

    raw_mode(true);

    // app loop
    loop {
        clear(); // clear last loop, or Rust debug logs
        collect_presses(&mut app);

        if key_press(&app, ":") {
            if halt_press_check(&mut app, "q") {
                clear();
                break;
            }
        }

        if key_press(&app, "e") && !is_typing {
            is_typing = true;
            app.keys_pressed = String::new();
        }

        if is_typing && key_press(&app, "Enter") {
            is_typing = false;

            let message = format!("Press S to Save: {}", file_path);
            line(position!(0, 1), &message, "red");

            if halt_press_check(&mut app, "S") {
                std::fs::File::create(file_path).unwrap();
                for line in current_text.clone() {
                    let mut file = std::fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(file_path)
                        .unwrap();

                    writeln!(file, "{}", line).unwrap();
                }
                clear();
                is_typing = true;
            }
        }

        if is_typing && key_press(&app, "Space") {
            current_text[current_line] = format!("{} ", current_text[current_line]);
        }

        if is_typing && key_press(&app, "Backspace") {
            current_text[current_line].pop();
        }

        if is_typing {
            if app.keys_pressed.len() == 1 {
                current_text[current_line] =
                    format!("{}{}", current_text[current_line], app.keys_pressed);
            }
            let mut line_iter = 1;
            for text_line in current_text.clone() {
                line(Position { x: line_iter, y: 0 }, &text_line, "yellow");
                line_iter += 1;
            }
        }

        if is_typing && key_press(&app, "Up") {
            if current_line != 0 {
                current_line -= 1;
            }
        }

        if is_typing && key_press(&app, "Down") {
            current_line += 1;
            if current_line >= current_text.len() {
                current_text.push(String::from(""));
            }
        }
    }

    raw_mode(false);
}
