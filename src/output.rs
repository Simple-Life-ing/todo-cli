use crate::model::Todo;
use colored::*;

/// Print a single todo with modern formatting
pub fn print_todo(todo: &Todo) {
    let (status, title_display) = if todo.completed {
        ("☑".green(), todo.title.strikethrough().dimmed())
    } else {
        ("☐".yellow(), todo.title.normal())
    };

    println!(
        "  {} {} - {}",
        status,
        todo.id.to_string().cyan().bold(),
        title_display
    );
}

/// Print todos iterator with header and progress, return (total, completed)
pub fn print_todos<I: Iterator<Item = Todo>>(iter: I) -> (usize, usize) {
    let todos: Vec<_> = iter.collect();
    let total = todos.len();
    let completed = todos.iter().filter(|t| t.completed).count();

    if total == 0 {
        println!("{}", "📭 暂无任务".yellow().italic());
        return (0, 0);
    }

    println!("{}", "📋 任务列表".blue().bold());
    println!("{}", "─".repeat(40).dimmed());

    for todo in todos {
        print_todo(&todo);
    }

    println!("{}", "─".repeat(40).dimmed());
    print_progress(completed, total);
    println!();

    (total, completed)
}

/// Print a simple progress bar
fn print_progress(completed: usize, total: usize) {
    let percentage = if total > 0 {
        (completed as f32 / total as f32 * 100.0) as usize
    } else {
        0
    };
    let bar_width = 20;
    let filled = (percentage as f32 / 100.0 * bar_width as f32) as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "{} {}",
        "█".repeat(filled).green(),
        "░".repeat(empty).dimmed()
    );

    println!(
        "📊 进度: {} {}/{} ({:.1}%)",
        bar,
        completed.to_string().green().bold(),
        total.to_string().cyan(),
        percentage as f32
    );
}
