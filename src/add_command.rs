use crate::common_structs::{ CommandResult, ExecutableCommand, Priority, Todo};
use crate::data_service::add_todo;
use chrono::Utc;
use termion::color;
use std::error::Error;

#[derive(Debug)]
pub struct AddCommand {
    title: String,
    priority: Priority
}

impl AddCommand {
    pub fn new(title: &str, priority: Priority) -> Result<AddCommand, Box<dyn Error>> {
        if title.trim().is_empty() {
            return Err("Value cannot be empty".into());
        }
        Ok(AddCommand { title: title.to_string(), priority: priority })
    }

    pub fn new_from_command_result(command_result: &CommandResult) -> Result<AddCommand, Box<dyn Error>> {
        if command_result.get_value().trim().is_empty() {
            return Err("Value cannot be empty".into());
        }
        let priority = match command_result.get_options().get("priority") {
            Some(p) => match p.as_str() {
                "H" => Priority::High,
                "h" => Priority::High,
                "M" => Priority::Medium,
                "m" => Priority::Medium,
                "L" => Priority::Low,
                "l" => Priority::Low,
                _ => return Err("Invalid priority value. Must be H, M or L".into())
            }
            None => Priority::Low
        };
    
        let add_command = AddCommand::new(command_result.get_value(), priority)?;
        Ok(add_command)
    }
}

impl ExecutableCommand for AddCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let todo = Todo::new(0, self.title.as_str(), self.priority, Utc::now())?;
        let id_assigned = add_todo(todo)?;
        println!("{}The todo {} has been added with id {}!", color::Fg(color::Green), self.title, id_assigned);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::add_command::{ AddCommand, Priority };
    use crate::common_structs::{ Command, CommandResult };
    use std::collections::HashMap;

    #[test]
    fn add_command_new_with_no_value_return_error() {
        let command = AddCommand::new("", Priority::Low);
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn add_command_new_with_whitespaces_value_return_error() {
        let command = AddCommand::new("   ", Priority::Low);
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn add_command_new_with_valid_args_return_valid() {
        let command = AddCommand::new("Test", Priority::Medium).unwrap();
        assert_eq!("Test", command.title);
        assert_eq!(Priority::Medium, command.priority);
    }

    #[test]
    fn add_command_new_from_command_result_with_no_value_return_error() {
        let command = AddCommand::new_from_command_result(&CommandResult::new(Command::Add, 
                                                                              "", 
                                                                              HashMap::new()));
        assert_eq!("Value cannot be empty", command.unwrap_err().to_string())
    }

    #[test]
    fn add_command_new_from_command_with_priority_value_empty_return_error() {
        let command_result = CommandResult::new(Command::Add, 
            "test", 
            HashMap::from([(String::from("priority"), String::from(""))]));
        let command = AddCommand::new_from_command_result(&command_result);
        assert_eq!("Invalid priority value. Must be H, M or L", command.unwrap_err().to_string());
    }

    #[test]
    fn add_command_new_from_command_with_priority_value_z_return_error() {
        let command_result = CommandResult::new(Command::Add, 
                                                "test", 
                                                HashMap::from([(String::from("priority"), String::from("z"))]));
        let command = AddCommand::new_from_command_result(&command_result);
        assert_eq!("Invalid priority value. Must be H, M or L", command.unwrap_err().to_string());
    }

    #[test]
    fn add_command_new_from_command_with_no_priority_return_valid_addcommand() {
        let command_result = CommandResult::new(Command::Add, 
                                                "test", 
                                                HashMap::new());
        let add_command = AddCommand::new_from_command_result(&command_result).unwrap();
        assert_eq!("test", add_command.title);
        assert_eq!(Priority::Low, add_command.priority);
    }

    fn parse_command_with_x_priority_return_valid_addcommand(priority: Priority, letter: &str) {
        let command_result = CommandResult::new(Command::Add, 
            "test", 
            HashMap::from([(String::from("priority"), String::from(letter))]));
            let add_command = AddCommand::new_from_command_result(&command_result).unwrap();
            assert_eq!("test", add_command.title);
            assert_eq!(priority, add_command.priority);
    }

    #[test]
    fn parse_command_with_mcaps_priority_return_valid_addcommand() {
        parse_command_with_x_priority_return_valid_addcommand(Priority::Medium, "M");
    }

    #[test]
    fn parse_command_with_hcaps_priority_return_valid_addcommand() {
        parse_command_with_x_priority_return_valid_addcommand(Priority::High, "H");
    }

    #[test]
    fn parse_command_with_m_priority_return_valid_addcommand() {
        parse_command_with_x_priority_return_valid_addcommand(Priority::Medium, "m");
    }

    #[test]
    fn parse_command_with_h_priority_return_valid_addcommand() {
        parse_command_with_x_priority_return_valid_addcommand(Priority::High, "h");
    }
}