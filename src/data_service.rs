use crate::common_structs::Todo;
use std::error::Error;
use home::home_dir;

pub fn get_todos_file() -> String {
    format!("{}/.beaver/todos.json", home_dir().unwrap().display())
}

pub fn add_todo(mut todo: Todo) -> Result<(), Box<dyn Error>> {
    let mut todos = read_all_todos()?;
    //TODO: Find the first available id
    todo.set_id(1);
    todos.push(todo);
    write_todos(&todos)?;
    Ok(())
}

pub fn read_all_todos() -> Result<Vec<Todo>, Box<dyn Error>> {
    let todo_file_path = get_todos_file();
    if std::fs::metadata(&todo_file_path).is_ok() {
        let content = std::fs::read_to_string(&todo_file_path)?;
        let todos: Vec<Todo> = serde_json::from_str(&content)?;
        Ok(todos)
    }
    else {
        Ok(vec![])
    }
}

pub fn write_todos(todos: &Vec<Todo>) -> Result<(), Box<dyn Error>> {
    let todos_str = serde_json::to_string_pretty(todos)?;
    std::fs::write(get_todos_file(), todos_str)?;
    Ok(())
}

/*#[test]
fn s() {
    
}*/