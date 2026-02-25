use crate::model::Todo;
use crate::storage;
use anyhow::{Result, anyhow};
use colored::*;

pub fn add(title: String) -> Result<()> {
    let mut todos = storage::load()?;
    let id = todos.len() + 1;

    todos.push(Todo {
        id,
        title,
        completed: false,
    });

    storage::save(&todos)?;
    println!("✅ 添加成功");
    Ok(())
}

pub fn list(show_all: bool) -> Result<()> {
    let todos = storage::load()?;

    if todos.is_empty() {
        println!("{}", "📭 暂无任务".yellow());
        return Ok(());
    }

    let mut completed = 0;

    for todo in &todos {
        if !show_all && todo.completed {
            continue;
        }

        let status = if todo.completed {
            completed += 1;
            "✔".green()
        } else {
            " ".normal()
        };

        println!(
            "[{}] {} - {}",
            status,
            todo.id.to_string().cyan(),
            if todo.completed {
                todo.title.strikethrough()
            } else {
                todo.title.normal()
            }
        );
    }

    println!("\n{} {}/{}", "完成进度:".blue(), completed, todos.len());

    Ok(())
}

pub fn done(id: usize) -> Result<()> {
    let mut todos = storage::load()?;

    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or(anyhow!("未找到该任务"))?;

    todo.completed = true;

    storage::save(&todos)?;
    println!("🎉 任务已完成");
    Ok(())
}

pub fn delete(id: usize) -> Result<()> {
    let mut todos = storage::load()?;

    if id == 0 || id > todos.len() {
        return Err(anyhow!("任务不存在"));
    }

    todos.remove(id - 1);

    // 重新排序 id
    for (index, todo) in todos.iter_mut().enumerate() {
        todo.id = index + 1;
    }

    storage::save(&todos)?;
    println!("🗑 删除成功");
    Ok(())
}

pub fn clear() -> Result<()> {
    storage::save(&vec![])?;
    println!("🧹 已清空所有任务");
    Ok(())
}
