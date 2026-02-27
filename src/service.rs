use crate::model::Todo;
use crate::storage;
//use anyhow::{Result, anyhow};
use colored::*;

pub fn add(title: String) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, 0)",
        [&title],
    )?;

    println!("✅ 添加成功");
    Ok(())
}

pub fn list(show_all: bool) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    let mut stmt = if show_all {
        conn.prepare("SELECT id, title, completed FROM todos")?
    } else {
        conn.prepare("SELECT id, title, completed FROM todos WHERE completed = 0")?
    };

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(2)?,
        ))
    })?;

    let mut total = 0;
    let mut completed_count = 0;

    for row in rows {
        let (id, title, completed) = row?;
        total += 1;

        if completed {
            completed_count += 1;
        }

        let status = if completed {
            "✔".green()
        } else {
            " ".normal()
        };

        let title_display = if completed {
            title.strikethrough()
        } else {
            title.normal()
        };

        println!("[{}] {} - {}", status, id.to_string().cyan(), title_display);
    }

    if total == 0 {
        println!("{}", "📭 暂无任务".yellow());
    } else {
        println!("\n{} {}/{}", "完成进度:".blue(), completed_count, total);
    }

    Ok(())
}

pub fn done(id: usize) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    conn.execute("UPDATE todos SET completed = 1 WHERE id = ?1", [id])?;

    println!("🎉 任务已完成");
    Ok(())
}

pub fn delete(id: usize) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;

    println!("🗑 删除成功");
    Ok(())
}

pub fn clear() -> anyhow::Result<()> {
    let conn = storage::get_connection()?;
    conn.execute("DELETE FROM todos", [])?;
    println!("🧹 已清空");
    Ok(())
}
