use crate::model::Todo;
use colored::*;

/// Print a single todo with colored formatting
pub fn print_todo(todo: &Todo) {
    let status = if todo.completed {
        "✔".green()
    } else {
        " ".normal()
    };

    let title_display = if todo.completed {
        todo.title.strikethrough()
    } else {
        todo.title.normal()
    };

    println!(
        "[{}] {} - {}",
        status,
        todo.id.to_string().cyan(),
        title_display
    );
}

/// Print todos iterator and return (total, completed)
pub fn print_todos<I: Iterator<Item = Todo>>(iter: I) -> (usize, usize) {
    let mut total = 0usize;
    let mut completed = 0usize;

    for todo in iter {
        if todo.completed {
            completed += 1;
        }
        total += 1;
        print_todo(&todo);
    }

    (total, completed)
}
