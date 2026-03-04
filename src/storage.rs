use anyhow::Result;
use rusqlite::Connection;

fn get_db_path() -> std::path::PathBuf {
    let mut path = dirs::data_dir().expect("无法获取数据目录");
    path.push("todo-cli");
    std::fs::create_dir_all(&path).ok();
    path.push("todo.db");
    dbg!(&path);
    path
}

pub fn get_connection() -> Result<Connection> {
    let conn = Connection::open(get_db_path())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
        [],
    )?;

    conn.execute("CREATE INDEX IF NOT EXISTS idx_title ON todos(title)", [])?;

    Ok(conn)
}
/* Test Function */
/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Todo;

    #[test]
    fn test_save_and_load() {
        let todos = vec![Todo {
            id: 1,
            title: "test".into(),
            completed: false,
        }];

        save(&todos).unwrap();
        let loaded = load().unwrap();ß

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title, "test");
    }
}*/
