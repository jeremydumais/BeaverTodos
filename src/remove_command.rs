use crate::common_structs::{CommandResult, ExecutableCommand};
use crate::data_service::{read_all_todos, write_todos};
use question::{Question, Answer};
use termion::color;
use std::error::Error;

#[derive(Debug)]
pub struct RemoveCommand {
    id: u32
}

impl RemoveCommand {
    pub fn new_from_command_result(command_result : &CommandResult) -> Result<RemoveCommand, Box<dyn Error>> {
        let value = command_result.get_value().trim();
        if value.is_empty() {
            return Err("Value cannot be empty".into());
        }
        let id = value.parse::<u32>()?;
        Ok(RemoveCommand { id: id })
    }
}

impl ExecutableCommand for RemoveCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut todos = read_all_todos()?;
        //Find the todo to remove
        let mut todo_index = 0;
        for (i, todo) in todos.iter().enumerate() {
            if todo.get_id() == self.id && !todo.get_completed() {
                todo_index = i;
                break;
            }
        }
        if todo_index == 0 {
            return Err(format!("Unable to find the todo with id {}", self.id).into());
        }
        let answer = Question::new(format!("Are you sure you want to delete the todo with id {0}? (y/n)", self.id).as_str())
                                   .yes_no()
                                   .until_acceptable()
                                   .ask();
        if answer.unwrap_or(Answer::NO) == Answer::YES {
            todos.remove(todo_index);
            write_todos(&todos)?;
            println!("{}The todo with id {} has been removed!{}", color::Fg(color::Green), self.id, color::Fg(color::Reset));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_command::RemoveCommand;
    use crate::common_structs::{Command, CommandResult};
    use std::collections::HashMap;

    #[test]
    fn remove_command_new_from_command_result_with_empty_value_return_error() {
        let command = RemoveCommand::new_from_command_result(&CommandResult::new(Command::Remove, 
            "", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn remove_command_new_from_command_result_with_whitestrings_value_return_error() {
        let command = RemoveCommand::new_from_command_result(&CommandResult::new(Command::Remove, 
            "  ", 
            HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn remove_command_new_from_command_result_with_1_value_return_success() {
        let command = RemoveCommand::new_from_command_result(&CommandResult::new(Command::Remove, 
            "1", 
            HashMap::new()));
        assert_eq!(1, command.unwrap().id);
    }

    #[test]
    fn remove_command_new_from_command_result_with_100_value_return_success() {
        let command = RemoveCommand::new_from_command_result(&CommandResult::new(Command::Remove, 
            "100", 
            HashMap::new()));
        assert_eq!(100, command.unwrap().id);
    }

    #[test]
    fn remove_command_new_from_command_result_with_minus_1_value_return_error() {
        let command = RemoveCommand::new_from_command_result(&CommandResult::new(Command::Remove, 
            "-1", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }

    #[test]
    fn remove_command_new_from_command_result_with_minus_abc_value_return_error() {
        let command = RemoveCommand::new_from_command_result(&CommandResult::new(Command::Remove, 
            "abc", 
            HashMap::new()));
        assert_eq!("invalid digit found in string", command.unwrap_err().to_string());
    }
}