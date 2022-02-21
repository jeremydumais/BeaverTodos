use crate::data_service;
use crate::common_structs::CommandResult;
use crate::common_structs::ExecutableCommand;
use crate::common_structs::Priority;
use crate::common_structs::Todo;
use termion::style;
use std::error::Error;
use std::io;
use std::io::Write;


pub enum SortOrder {
    PriorityDESC,
    PriorityASC,
    CreationTimeDESC,
    CreationTimeASC
}

pub struct ListCommand {
    sort_order: SortOrder,
    all: bool
}

impl ListCommand {
    const ID_WIDTH: usize = 3;
    const PRIORITY_WIDTH: usize = 10;
    const CREATION_WIDTH: usize = 33;
    const COMPLETED_DATE_WIDTH: usize = 33;

    pub fn new_from_command_result(command_result : &CommandResult) -> Result<ListCommand, Box<dyn Error>> {
        let sort_order_choosen = match command_result.get_options().get("sort") {
            Some(p) => match p.as_str() {
                "PriorityDESC" => SortOrder::PriorityDESC,
                "prioritydesc" => SortOrder::PriorityDESC,
                "Priority" => SortOrder::PriorityASC,
                "priority" => SortOrder::PriorityASC,
                "CreationTimeDESC" => SortOrder::CreationTimeDESC,
                "creationtimedesc" => SortOrder::CreationTimeDESC,
                "CreationTime" => SortOrder::CreationTimeASC,
                "creationtime" => SortOrder::CreationTimeASC,
                _ => return Err("Invalid sort value. Must be PriorityDESC, Priority, CreationTimeDESC or CreationTime".into())
            }
            None => SortOrder::PriorityDESC
        };
        let show_all = command_result.get_options().contains_key("all");
        
        Ok(ListCommand { sort_order: sort_order_choosen, all: show_all })
    }

    fn print_todos(&self, todos: &Vec<Todo>, include_creation: bool) {
        let terminal_size = match termion::terminal_size() {
            Ok(size) => size,
            Err(_) => (100, 100)
        };
        
        let mut title_width = usize::from(terminal_size.0);
        if title_width >= ListCommand::ID_WIDTH + ListCommand::PRIORITY_WIDTH + 3 {
            title_width -= ListCommand::ID_WIDTH + ListCommand::PRIORITY_WIDTH + 3;
        }
        if include_creation && title_width >= ListCommand::CREATION_WIDTH {
            title_width -= ListCommand::CREATION_WIDTH;
        }
        if self.all && title_width >= ListCommand::COMPLETED_DATE_WIDTH {
            title_width -= ListCommand::COMPLETED_DATE_WIDTH;
        }

        self.print_header(title_width, include_creation);

        for todo in todos {
            self.print_todo_line(todo, title_width, include_creation);
        }
    }

    fn print_header(&self, title_width: usize, include_creation: bool) {
        print!("{}{id:<widthi$} {title:<widtht$} {priority:widthp$}", 
                style::Underline, id="ID", widthi=ListCommand::ID_WIDTH, 
                title="Title", widtht=title_width, 
                priority="Priority", widthp=ListCommand::PRIORITY_WIDTH);
        if include_creation {
            print!("{creation:widthc$}", creation="Creation date", widthc=ListCommand::CREATION_WIDTH)
        }
        if self.all {
            print!("{when_completed:widthw$}", when_completed="Completed date", widthw=ListCommand::COMPLETED_DATE_WIDTH)

        }
        print!("\n{}", style::NoUnderline);
        io::stdout().flush().unwrap();
    }

    fn print_todo_line(&self, todo: &Todo, title_width: usize, include_creation: bool) {
        let mut title = todo.get_title().to_string();
            if title.len() > title_width {
                title = title[..title_width].to_string();
            }
            let id = match todo.get_completed() { 
                true => String::from("[X]"), 
                false => todo.get_id().to_string()
            };
            if todo.get_priority() == Priority::High {
                print!("{}", style::Bold);
            }
            print!("{id:<widthi$} {title:<widtht$} {priority:widthp$}", 
                    id=id, widthi=ListCommand::ID_WIDTH, 
                    title=title, widtht=title_width, 
                    priority=todo.get_priority().to_string(), widthp=ListCommand::PRIORITY_WIDTH);
            if include_creation {
                print!("{creation:widthc$}", 
                       creation=todo.get_when_created_in_localtime().to_rfc2822(), 
                       widthc=ListCommand::CREATION_WIDTH);
            }
            if self.all {
                let mut completed_date = todo.get_when_completed_in_localtime().to_rfc2822();
                if !todo.get_completed() {
                    completed_date = String::new();
                }
                print!("{when_completed:widthw$}", 
                       when_completed=completed_date, 
                       widthw=ListCommand::COMPLETED_DATE_WIDTH);
            }
            print!("\n{}", style::NoBold);
            io::stdout().flush().unwrap();
    }
}

impl ExecutableCommand for ListCommand {
    fn execute(&self,) -> Result<(), Box<dyn Error>> {
        let mut todos = data_service::read_all_todos()?;
        //Remove all the completed todos
        if !self.all {
            todos.retain(|x| !x.get_completed());
        }
        match self.sort_order {
            SortOrder::CreationTimeASC => todos.sort_by(|a, b| a.get_when_created_in_localtime().cmp(&b.get_when_created_in_localtime())),
            SortOrder::CreationTimeDESC => todos.sort_by(|a, b| b.get_when_created_in_localtime().cmp(&a.get_when_created_in_localtime())),
            SortOrder::PriorityASC => todos.sort_by(|a, b| b.get_priority().cmp(&a.get_priority())),
            SortOrder::PriorityDESC => todos.sort_by(|a, b| a.get_priority().cmp(&b.get_priority())),            
        }
        //Sort todos by priority by default (Highest to lowest)
        if todos.len() > 0 {
            match self.sort_order {
                SortOrder::PriorityASC => self.print_todos(&todos, false),
                SortOrder::PriorityDESC => self.print_todos(&todos, false),
                SortOrder::CreationTimeASC => self.print_todos(&todos, true),
                SortOrder::CreationTimeDESC => self.print_todos(&todos, true)
            }
        }
        else {
            println!("Your todo list is empty! :)");
        }
        Ok(())
    }
}