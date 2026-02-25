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
