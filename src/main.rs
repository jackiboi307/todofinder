use walkdir::WalkDir;
use termabc::prelude::*;
use defer_rs::defer;
use std::process::Command;

fn init() {
    init_term().unwrap();
    print!("{CUR_HIDE}{CUR_HOME}");
    flush();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let editor = "nvim";
    let dir = "src";

    let mut todos = Vec::new();
    let mut selected = 0;

    for entry in WalkDir::new(dir) {
        let entry = entry.expect("error");
        if entry.path().is_file() {
            let contents = std::fs::read_to_string(entry.path())?;
            for (i, line) in contents.lines().enumerate() {
                if let Some(index) = line.find("TODO") {
                    todos.push((
                        i + 1,
                        String::from(line.get(index+4..).unwrap().trim()),
                        String::from(format!("{}", entry.path().display())),
                    ));
                }
            }
        }
    }

    init();
    defer! {
        let _ = restore_term();
    };

    let path_col = Style::new().fg(Yellow);
    let line_col = Style::new().fg(BrightBlue);
    let todo_col = Style::new().fg(BrightWhite);

    loop {
        printf!("{ERASE_SCREEN}{CUR_HOME}{FG_WHITE_B}{BOLD}// TODOFINDER{CUR_SET}", 3, 1);

        for (i, (line, todo, path)) in todos.iter().enumerate() {
            let select = if i == selected {
                UNDERLINE
            } else {
                ""
            };

            printf!("{CUR_COL_HOME}{path_col}{select}{path}{RESET}:\
                     {line_col}{line}{RESET}{CUR_DOWN_ONE}{CUR_COL_HOME}    \
                     {todo_col}{todo}{CUR_DOWN}", 2);
        }

        flush();

        let bytes = &*read_bytes::<6>()?;

        match bytes {
            CTRL_C | ESCAPE => break,
            ARROW_DOWN => {
                selected = (selected + 1) % todos.len()
            }
            ARROW_UP => {
                selected = selected.checked_sub(1).unwrap_or_else(|| todos.len() - 1)
            }
            RETURN => {
                let _ = restore_term();
                Command::new(editor)
                    .arg("-c")
                    .arg(format!(":{}", todos[selected].0))
                    .arg(todos[selected].2.clone())
                    .spawn()?
                    .wait()?;
                init();
            }
            _ => {}
        }
    }

    Ok(())
}
