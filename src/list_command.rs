use crate::data_service;
use crate::common_structs::CommandResult;
use crate::common_structs::ExecutableCommand;
use std::error::Error;

pub struct ListCommand {
}

impl ListCommand {
    pub fn new_from_command_result(_command : &CommandResult) -> Result<ListCommand, Box<dyn Error>> {
        Ok(ListCommand {})
    }
}

impl ExecutableCommand for ListCommand {
    fn execute(&self,) -> Result<(), Box<dyn Error>> {
        let mut todos = data_service::read_all_todos()?;
        //Sort todos by priority by default (Highest to lowest)
        todos.sort_by(|a, b| a.get_priority().cmp(&b.get_priority()));
        if todos.len() > 0 {
            // Print all todos
            const ID_WIDTH: usize = 3;
            const TITLE_WIDTH: usize = 50;
            const PRIORITY_WIDTH: usize = 9;
            println!("{id:<widthi$} {title:<widtht$} {priority:widthp$}", 
                    id="ID", widthi=ID_WIDTH, 
                    title="Title", widtht=TITLE_WIDTH, 
                    priority="Priority", widthp=PRIORITY_WIDTH);
            println!("{underline:-<width$}", underline="", width=ID_WIDTH+TITLE_WIDTH+PRIORITY_WIDTH+1);
            for todo in todos {
                println!("{id:<widthi$} {title:<widtht$} {priority:widthp$}", 
                        id=todo.get_id(), widthi=ID_WIDTH, 
                        title=todo.get_title(), widtht=TITLE_WIDTH, 
                        priority=todo.get_priority().to_string(), widthp=PRIORITY_WIDTH);
            }
        }
        else {
            println!("Your todo list is empty! :)");
        }
        Ok(())
    }
}