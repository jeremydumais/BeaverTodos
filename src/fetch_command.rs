use crate::common_structs::{CommandResult, ExecutableCommand};
use crate::data_service::read_all_todos;
use termion::style;
use std::error::Error;

#[derive(Debug)]
pub struct FetchCommand {
    id: u32
}
impl FetchCommand {
    pub fn new_from_command_result(command_result : &CommandResult) -> Result<FetchCommand, Box<dyn Error>> {
        let value = command_result.get_value().trim();
        if value.is_empty() {
            return Err("Value cannot be empty".into());
        }
        let id = value.parse::<u32>()?;
        Ok(FetchCommand { id: id })
    }
}

impl ExecutableCommand for FetchCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let todos = read_all_todos()?;
        let mut iter = todos.iter();
        match iter.find(|x| x.get_id() == self.id) {
            Some(todo) => {
                println!("Title: {}{}{}", style::Bold, todo.get_title(), style::Reset);
                println!("ID: {}", todo.get_id());
                println!("Priority: {}", todo.get_priority().to_string());
                println!("Created on: {}", todo.get_when_created_in_localtime());
            },
            None => return Err(format!("Unable to find the todo with id {}", self.id).into())
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::fetch_command::FetchCommand;
    use crate::common_structs::{Command, CommandResult};
    use std::collections::HashMap;

    #[test]
    fn fetch_command_new_from_command_result_with_empty_value_return_error() {
        let command = FetchCommand::new_from_command_result(&CommandResult::new(Command::Fetch, 
            "", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn fetch_command_new_from_command_result_with_whitestrings_value_return_error() {
        let command = FetchCommand::new_from_command_result(&CommandResult::new(Command::Fetch, 
            "  ", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn fetch_command_new_from_command_result_with_1_value_return_success() {
        let command = FetchCommand::new_from_command_result(&CommandResult::new(Command::Fetch, 
            "1", 
            HashMap::new()));
        assert_eq!(1, command.unwrap().id);
    }

    #[test]
    fn fetch_command_new_from_command_result_with_100_value_return_success() {
        let command = FetchCommand::new_from_command_result(&CommandResult::new(Command::Fetch, 
            "100", 
            HashMap::new()));
        assert_eq!(100, command.unwrap().id);
    }

    #[test]
    fn fetch_command_new_from_command_result_with_minus_1_value_return_error() {
        let command = FetchCommand::new_from_command_result(&CommandResult::new(Command::Fetch, 
            "-1", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }

    #[test]
    fn fetch_command_new_from_command_result_with_minus_abc_value_return_error() {
        let command = FetchCommand::new_from_command_result(&CommandResult::new(Command::Fetch, 
            "abc", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }
}