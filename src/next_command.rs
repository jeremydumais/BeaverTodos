use crate::common_structs::ExecutableCommand;
use crate::data_service::read_all_todos;
use termion::style;
use std::error::Error;

#[derive(Debug)]
pub struct NextCommand {
}

impl ExecutableCommand for NextCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut todos = read_all_todos()?;
        todos.retain(|x| !x.get_completed());
        if !todos.is_empty() {
            todos.sort_unstable_by_key(|item| (item.get_priority(), item.get_when_created_in_localtime()));
            let todo = &todos[0]; 
            println!("Title: {}{}{}", style::Bold, todo.get_title(), style::Reset);
            println!("ID: {}", todo.get_id());
            println!("Priority: {}", todo.get_priority().to_string());
            println!("Created on: {}", todo.get_when_created_in_localtime());
        }
        else {
            println!("Your todo list is empty! :)");
        }
        Ok(())
    }
}