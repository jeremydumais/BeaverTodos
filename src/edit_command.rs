use crate::common_structs::{CommandResult, ExecutableCommand, Priority};
use crate::data_service::{read_all_todos, write_todos};
use termion::color;
use std::error::Error;

#[derive(Debug)]
pub struct EditCommand {
    id: u32,
    title: Option<String>,
    priority: Option<Priority>
}

impl EditCommand {
    pub fn new_from_command_result(command_result: &CommandResult) -> Result<EditCommand, Box<dyn Error>> {
        let value = command_result.get_value().trim();
        if value.is_empty() {
            return Err("Value cannot be empty".into());
        }
        let id = value.parse::<u32>()?;
        let priority = match command_result.get_options().get("priority") {
            Some(val) => Priority::from_string(val),
            _ => None
        };
        let title: Option<String> = match command_result.get_options().get("title") {
            Some(s) => Some(s.to_string()),
            _ => None
        };
        if title.is_some() && title.as_ref().unwrap().trim().is_empty() {
            return Err("The title cannot be empty".into());
        }

        if title.is_none() && priority.is_none() {
            return Err("At least one option must be supplied".into());
        }
        Ok(EditCommand {id: id, title: title, priority: priority})
    }
}

impl ExecutableCommand for EditCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut todos = read_all_todos()?;
        //Find the todo to update
        let mut iter = todos.iter_mut();
        match iter.find(|x| x.get_id() == self.id && !x.get_completed()) {
            Some(todo) => {
                if self.title.is_some() {
                    todo.set_title(self.title.as_ref().unwrap().as_str())?;
                }
                if self.priority.is_some() {
                    todo.set_priority(self.priority.unwrap())
                }
            },
            None => return Err(format!("Unable to find the todo with id {}", self.id).into())
        }
        write_todos(&todos)?;
        println!("{}The todo {} has been updated!", color::Fg(color::Green), self.id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::common_structs::{Command, CommandResult, Priority};
    use crate::edit_command::EditCommand;
    use std::collections::HashMap;

    #[test]
    fn edit_command_new_from_command_result_with_value_empty_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn edit_command_new_from_command_result_with_value_whitespaces_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "   ", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn edit_command_new_from_command_result_with_1_value_return_success() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::from([(String::from("title"), String::from("test"))])));
        assert_eq!(1, command.unwrap().id);
    }

    #[test]
    fn edit_command_new_from_command_result_with_100_value_return_success() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "100", 
            HashMap::from([(String::from("title"), String::from("test"))])));
        assert_eq!(100, command.unwrap().id);
    }

    #[test]
    fn edit_command_new_from_command_result_with_minus_1_value_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "-1", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }

    #[test]
    fn edit_command_new_from_command_result_with_minus_abc_value_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "abc", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }

    #[test]
    fn edit_command_new_from_command_result_with_no_options_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::new()));
        assert_eq!("At least one option must be supplied", command.unwrap_err().to_string());
    }

    #[test]
    fn edit_command_new_from_command_result_with_empty_title_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::from([(String::from("title"), String::from(""))])));
        assert_eq!("The title cannot be empty", command.unwrap_err().to_string());
    }

    #[test]
    fn edit_command_new_from_command_result_with_whitespaces_title_return_error() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::from([(String::from("title"), String::from("   "))])));
        assert_eq!("The title cannot be empty", command.unwrap_err().to_string());
    }

    #[test]
    fn edit_command_new_from_command_result_with_test_title_return_success() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::from([(String::from("title"), String::from("test"))]))).unwrap();
        assert_eq!(1, command.id);
        assert_eq!("test", command.title.unwrap());
        assert!(command.priority.is_none());
    }

    #[test]
    fn edit_command_new_from_command_result_with_priority_m_return_success() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::from([(String::from("priority"), String::from("m"))]))).unwrap();
        assert_eq!(1, command.id);
        assert_eq!(Priority::Medium, command.priority.unwrap());
        assert!(command.title.is_none());
    }

    #[test]
    fn edit_command_new_from_command_result_with_both_title_and_priority_return_success() {
        let command = EditCommand::new_from_command_result(&CommandResult::new(Command::Edit, 
            "1", 
            HashMap::from([(String::from("title"), String::from("test")),
                           (String::from("priority"), String::from("m"))]))).unwrap();
        assert_eq!(1, command.id);
        assert_eq!(Priority::Medium, command.priority.unwrap());
        assert_eq!("test", command.title.unwrap());
    }
}