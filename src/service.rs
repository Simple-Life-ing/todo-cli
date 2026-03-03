use crate::model::Todo;
use crate::storage;
use anyhow::{Result, anyhow};
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

    let todos = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut total = 0;
    let mut completed_count = 0;

    for todo in todos {
        let todo = todo?;
        total += 1;

        let status = if todo.completed {
            completed_count += 1;
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

    if conn.changes() == 0 {
        println!("{}", format!("⚠️  ID {} 不存在", id).yellow());
    } else {
        println!("🎉 任务已完成");
    }
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
        Ok(crate::model::Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    println!("{}", "🔍 搜索结果:".blue());

    for todo in todos {
        let todo = todo?;
        let status = if todo.completed {
            "✔".green()
        } else {
            " ".normal()
        };

        println!(
            "[{}] {} - {}",
            status,
            todo.id.to_string().cyan(),
            todo.title
        );
    }

    Ok(())
}

pub fn export_json(path: String) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;

    let todos = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut vec = vec![];

    for todo in todos {
        vec.push(todo?);
    }

    std::fs::write(&path, serde_json::to_string_pretty(&vec)?)?;

    println!("{}", "📤 导出成功".green());
    Ok(())
}

pub fn import_json(path: String, preserve_id: bool) -> Result<()> {
    let content = std::fs::read_to_string(&path)?;
    let todos: Vec<Todo> = serde_json::from_str(&content)?;

    if todos.is_empty() {
        println!("{}", "文件中没有数据".yellow());
        return Ok(());
    }

    let mut conn = storage::get_connection()?;
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM todos", [])?;

    if preserve_id {
        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO todos (id, title, completed)
             VALUES (?1, ?2, ?3)",
        )?;

        for todo in todos {
            stmt.execute((todo.id, todo.title, todo.completed))?;
        }

        // 重置 AUTOINCREMENT 计数
        tx.execute("DELETE FROM sqlite_sequence WHERE name='todos'", [])?;
    } else {
        let mut stmt = tx.prepare(
            "INSERT INTO todos (title, completed)
             VALUES (?1, ?2)",
        )?;

        for todo in todos {
            stmt.execute((todo.title, todo.completed))?;
        }
    }

    tx.commit()?;

    println!("{}", "📥 导入完成".green());
    Ok(())
}

pub fn delete(id: usize) -> anyhow::Result<()> {
    let conn = storage::get_connection()?;

    conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;

    if conn.changes() == 0 {
        println!("{}", format!("⚠️  ID {} 不存在", id).yellow());
    } else {
        println!("🗑 删除成功");
    }
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
