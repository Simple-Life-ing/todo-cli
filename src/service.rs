use crate::model::Todo;
use crate::output::print_todos;
use anyhow::Result;
use colored::*;
use rusqlite::Connection;

/// All service functions now take a database connection reference so
/// the caller (e.g. `main`) owns the connection. This decouples the
/// database acquisition from business logic, making the latter
/// easier to test and swap out.

pub fn add(conn: &Connection, title: String) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, 0)",
        [&title],
    )?;

    println!("✅ 添加成功");
    Ok(())
}

pub fn batch_add(conn: &mut Connection, titles: Vec<String>) -> anyhow::Result<()> {
    if titles.is_empty() {
        println!("{}", "⚠️  没有提供任务标题".yellow());
        return Ok(());
    }

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

pub fn list(conn: &Connection, show_all: bool) -> anyhow::Result<()> {
    let mut stmt = if show_all {
        conn.prepare("SELECT id, title, completed FROM todos")?
    } else {
        conn.prepare("SELECT id, title, completed FROM todos WHERE completed = 0")?
    };

    // collect all rows into a Vec<Todo>
    let todos: Vec<Todo> = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if todos.is_empty() {
        println!("{}", "📭 暂无任务".yellow());
    } else {
        let (total, completed_count) = print_todos(todos.into_iter());
        println!("\n{} {}/{}", "完成进度:".blue(), completed_count, total);
    }

    Ok(())
}

pub fn done(conn: &Connection, id: usize) -> anyhow::Result<()> {
    conn.execute("UPDATE todos SET completed = 1 WHERE id = ?1", [id])?;

    if conn.changes() == 0 {
        println!("{}", format!("⚠️  ID {} 不存在", id).yellow());
    } else {
        println!("🎉 任务已完成");
    }
    Ok(())
}

pub fn search(conn: &Connection, keyword: String) -> anyhow::Result<()> {
    let mut stmt = conn.prepare(
        "SELECT id, title, completed 
         FROM todos 
         WHERE title LIKE ?1",
    )?;

    let pattern = format!("%{}%", keyword);
    let todos: Vec<Todo> = stmt
        .query_map([pattern], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if todos.is_empty() {
        println!("{}", "🔍 未找到匹配的任务".yellow());
    } else {
        println!("{}", "🔍 搜索结果:".blue());
        print_todos(todos.into_iter());
    }

    Ok(())
}

pub fn export_json(conn: &Connection, path: String) -> anyhow::Result<()> {
    let mut stmt = conn.prepare("SELECT id, title, completed FROM todos")?;

    let todos: Vec<Todo> = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    std::fs::write(&path, serde_json::to_string_pretty(&todos)?)?;

    println!("{}", "📤 导出成功".green());
    Ok(())
}

pub fn import_json(conn: &mut Connection, path: String, preserve_id: bool) -> Result<()> {
    let content = std::fs::read_to_string(&path)?;
    let todos: Vec<Todo> = serde_json::from_str(&content)?;

    if todos.is_empty() {
        println!("{}", "文件中没有数据".yellow());
        return Ok(());
    }

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

pub fn delete(conn: &Connection, id: usize) -> anyhow::Result<()> {
    conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;

    if conn.changes() == 0 {
        println!("{}", format!("⚠️  ID {} 不存在", id).yellow());
    } else {
        println!("🗑 删除成功");
    }
    Ok(())
}

pub fn clear(conn: &mut Connection) -> anyhow::Result<()> {
    let tx = conn.transaction()?;

    tx.execute("DELETE FROM todos", [])?;

    tx.commit()?; //提交事务

    println!("🧹 已清空");
    Ok(())
}

pub fn reset(conn: &mut Connection) -> anyhow::Result<()> {
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
