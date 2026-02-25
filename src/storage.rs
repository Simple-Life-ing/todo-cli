use crate::model::Todo;
use anyhow::Result;
use std::fs;
use std::path::Path;

const FILE_PATH: &str = "todos.json";

pub fn load() -> Result<Vec<Todo>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let data = fs::read_to_string(FILE_PATH)?;
    let todos = serde_json::from_str(&data)?;
    Ok(todos)
}

pub fn save(todos: &Vec<Todo>) -> Result<()> {
    let data = serde_json::to_string_pretty(todos)?;
    fs::write(FILE_PATH, data)?;
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
