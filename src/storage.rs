use crate::model::Todo;
use anyhow::Result;
use dirs::data_dir;
use std::fs;
use std::path::{Path, PathBuf};

fn get_file_path() -> PathBuf {
    let mut path = data_dir().expect("无法获取数据目录");
    path.push("todo-cli");
    std::fs::create_dir_all(&path).ok();
    path.push("todos.json");
    path
}

pub fn load() -> Result<Vec<Todo>> {
    let file_path = get_file_path();
    //dbg!(&file_path);
    if !file_path.exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(file_path)?;
    let todos = serde_json::from_str(&data)?;
    Ok(todos)
}

pub fn save(todos: &Vec<Todo>) -> Result<()> {
    let file_path = get_file_path();
    let data = serde_json::to_string_pretty(todos)?;
    fs::write(file_path, data)?;
    Ok(())
}

/* Test Function */
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
        let loaded = load().unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].title, "test");
    }
}
