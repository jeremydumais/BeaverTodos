use crate::common_structs::{CommandResult, ExecutableCommand};
use crate::data_service::{read_all_todos, write_todos};
use termion::color;
use std::error::Error;

#[derive(Debug)]
pub struct DoneCommand {
    id: u32
}

impl DoneCommand {
    pub fn new_from_command_result(command_result : &CommandResult) -> Result<DoneCommand, Box<dyn Error>> {
        let value = command_result.get_value().trim();
        if value.is_empty() {
            return Err("Value cannot be empty".into());
        }
        let id = value.parse::<u32>()?;
        Ok(DoneCommand { id: id })
    }
}

impl ExecutableCommand for DoneCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut todos = read_all_todos()?;
        let todo_title;
        //Find the todo to complete
        let mut iter = todos.iter_mut();
        match iter.find(|x| x.get_id() == self.id) {
            Some(todo) => {
                if todo.get_completed() {
                    return Err(format!("The todo with id {} is already completed", self.id).into());
                }
                todo.set_completed(true, None);
                todo_title = String::from(todo.get_title());
            },
            None => return Err(format!("Unable to find the todo with id {}", self.id).into())
        }
        write_todos(&todos)?;
        println!("{}The todo {} has been completed!{}", color::Fg(color::Green), todo_title, color::Fg(color::Reset));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::done_command::DoneCommand;
    use crate::common_structs::{Command, CommandResult};
    use std::collections::HashMap;

    #[test]
    fn done_command_new_from_command_result_with_empty_value_return_error() {
        let command = DoneCommand::new_from_command_result(&CommandResult::new(Command::Done, 
            "", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn done_command_new_from_command_result_with_whitestrings_value_return_error() {
        let command = DoneCommand::new_from_command_result(&CommandResult::new(Command::Done, 
            "  ", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn done_command_new_from_command_result_with_1_value_return_success() {
        let command = DoneCommand::new_from_command_result(&CommandResult::new(Command::Done, 
            "1", 
            HashMap::new()));
        assert_eq!(1, command.unwrap().id);
    }

    #[test]
    fn done_command_new_from_command_result_with_100_value_return_success() {
        let command = DoneCommand::new_from_command_result(&CommandResult::new(Command::Done, 
            "100", 
            HashMap::new()));
        assert_eq!(100, command.unwrap().id);
    }

    #[test]
    fn done_command_new_from_command_result_with_minus_1_value_return_error() {
        let command = DoneCommand::new_from_command_result(&CommandResult::new(Command::Done, 
            "-1", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }

    #[test]
    fn done_command_new_from_command_result_with_minus_abc_value_return_error() {
        let command = DoneCommand::new_from_command_result(&CommandResult::new(Command::Done, 
            "abc", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }
}