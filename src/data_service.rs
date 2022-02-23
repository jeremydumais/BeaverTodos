use crate::common_structs::Todo;
use std::error::Error;
use std::fs;
use std::path::Path;
use home::home_dir;

pub fn get_todos_file() -> String {
    format!("{}/.beaver/todos.json", home_dir().unwrap().display())
}

pub fn add_todo(mut todo: Todo) -> Result<u32, Box<dyn Error>> {
    let mut todos = read_all_todos()?;
    let id_assigned = find_next_available_todo_id(&todos);
    todo.set_id(find_next_available_todo_id(&todos));
    todos.push(todo);
    write_todos(&todos)?;
    Ok(id_assigned)
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
    //Check if the .beaver folder exist in the home directory
    let beaver_folder = format!("{}/.beaver", home_dir().unwrap().display());
    if !Path::new(&beaver_folder).exists() {
        fs::create_dir(&beaver_folder)?;
    }
    std::fs::write(get_todos_file(), todos_str)?;
    Ok(())
}

pub fn find_next_available_todo_id(todos: &Vec<Todo>) -> u32 {
    let mut available_id = 1;
    let mut is_id_available = false;
    while !is_id_available {
        is_id_available = true;
        // Check if the id suggested is already taken
        for todo in todos {
            if todo.get_id() == available_id && !todo.get_completed() {
                is_id_available = false;
                available_id += 1;
                break; 
            }
        }
    }
    available_id
}

#[cfg(test)]
mod tests {
    use crate::common_structs::{ Priority, Todo };
    use crate::data_service::find_next_available_todo_id;
    use chrono::Utc;

    #[test]
    fn find_next_available_todo_id_with_empty_return_one() {
        let todos: Vec<Todo> = vec![];
        assert_eq!(1, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_1_return_two() {
        let todos: Vec<Todo> = vec![
            Todo::new(1, "a", Priority::Low, Utc::now()).unwrap()
        ];
        assert_eq!(2, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_1_2_return_three() {
        let todos: Vec<Todo> = vec![
            Todo::new(1, "a", Priority::Low, Utc::now()).unwrap(),
            Todo::new(2, "b", Priority::Low, Utc::now()).unwrap()
        ];
        assert_eq!(3, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_1_2_3_return_three() {
        let todos: Vec<Todo> = vec![
            Todo::new(1, "a", Priority::Low, Utc::now()).unwrap(),
            Todo::new(2, "b", Priority::Low, Utc::now()).unwrap(),
            Todo::new(3, "c", Priority::Low, Utc::now()).unwrap()
        ];
        assert_eq!(4, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_2_3_return_one() {
        let todos: Vec<Todo> = vec![
            Todo::new(2, "a", Priority::Low, Utc::now()).unwrap(),
            Todo::new(3, "b", Priority::Low, Utc::now()).unwrap()
        ];
        assert_eq!(1, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_1_3_return_two() {
        let todos: Vec<Todo> = vec![
            Todo::new(1, "a", Priority::Low, Utc::now()).unwrap(),
            Todo::new(3, "b", Priority::Low, Utc::now()).unwrap()
        ];
        assert_eq!(2, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_1_completed_return_one() {
        let mut todos: Vec<Todo> = vec![
            Todo::new(1, "a", Priority::Low, Utc::now()).unwrap(),
        ];
        todos[0].set_completed(true, None);
        assert_eq!(1, find_next_available_todo_id(&todos));
    }

    #[test]
    fn find_next_available_todo_id_with_1_3_and_2_completed_return_two() {
        let mut todos: Vec<Todo> = vec![
            Todo::new(1, "a", Priority::Low, Utc::now()).unwrap(),
            Todo::new(2, "b", Priority::Low, Utc::now()).unwrap(),
            Todo::new(3, "c", Priority::Low, Utc::now()).unwrap()
        ];
        todos[1].set_completed(true, None);
        assert_eq!(2, find_next_available_todo_id(&todos));
    }
}