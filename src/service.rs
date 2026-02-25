use crate::model::Todo;
use crate::storage;
use anyhow::Result;

pub fn add(title: String) -> Result<()> {
    let mut todos = storage::load()?;

    let id = todos.len() + 1;

    todos.push(Todo {
        id,
        title,
        completed: false,
    });

    storage::save(&todos)?;
    println!("添加成功");
    Ok(())
}

pub fn list() -> Result<()> {
    let todos = storage::load()?;

    for todo in todos {
        let status = if todo.completed { "✔" } else { " " };
        println!("[{}] {} - {}", status, todo.id, todo.title);
    }

    Ok(())
}
