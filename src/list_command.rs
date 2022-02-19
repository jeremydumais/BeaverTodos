use crate::data_service;
use crate::common_structs::CommandResult;
use std::error::Error;

pub fn execute(_command : &CommandResult) -> Result<(), Box<dyn Error>> {
    let todos = data_service::read_all_todos()?;
    //Sort todos by priority by default (Highest to lowest)

    if todos.len() > 0 {
        // Print all todos
        const ID_WIDTH: usize = 3;
        const TITLE_WIDTH: usize = 50;
        const PRIORITY_WIDTH: usize = 9;
        println!("{id:<widthi$} {title:<widtht$} {priority:widthp$}", 
                 id="ID", widthi=ID_WIDTH, 
                 title="Title", widtht=TITLE_WIDTH, 
                 priority="Priority", widthp=PRIORITY_WIDTH);
        println!("{underline:-<width$}", underline="", width=ID_WIDTH+TITLE_WIDTH+PRIORITY_WIDTH);
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