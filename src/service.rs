use crate::model::Todo;
use crate::storage;
//use anyhow::{Result, anyhow};
use colored::*;
use rusqlite::Connection;

pub fn add(title: String) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, 0)",
        [&title],
    )?;

    println!("✅ 添加成功");
    Ok(())
}

pub fn batch_add(titles: Vec<String>) -> anyhow::Result<()> {
    if titles.is_empty() {
        println!("{}", "⚠️  没有提供任务标题".yellow());
        return Ok(());
    }

    let mut conn = storage::get_connection()?;
    let tx = conn.transaction()?;

    {
        let mut stmt = tx.prepare("INSERT INTO todos (title, completed) VALUES (?1, 0)")?;

        for title in titles {
            stmt.execute([title])?;
        }
    }

    tx.commit()?;

    println!("{}", "✅ 批量添加完成".green());
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

pub fn search(keyword: String) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, title, completed 
         FROM todos 
         WHERE title LIKE ?1",
    )?;

    let pattern = format!("%{}%", keyword);

    let todos = stmt.query_map([pattern], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(2)?,
        ))
    })?;

    println!("{}", "🔍 搜索结果:".blue());

    for todo in todos {
        let (id, title, completed) = todo?;
        let status = if completed {
            "✔".green()
        } else {
            " ".normal()
        };

        println!("[{}] {} - {}", status, id.to_string().cyan(), title);
    }

    Ok(())
}

pub fn delete(id: usize) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;

    println!("🗑 删除成功");
    Ok(())
}

pub fn clear() -> anyhow::Result<()> {
    let mut conn = storage::get_connection()?;
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM todos", [])?;

    tx.commit()?; //提交事务

    println!("🧹 已清空");
    Ok(())
}

pub fn reset() -> anyhow::Result<()> {
    let mut conn = storage::get_connection()?;
    let tx = conn.transaction()?;

    tx.execute("DROP TABLE IF EXISTS todos", [])?;

    tx.execute(
        "CREATE TABLE todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
        [],
    )?;

    tx.commit()?;

    println!("{}", "🔄 数据库已重置".red());
    Ok(())
}
