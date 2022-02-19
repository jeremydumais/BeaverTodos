use crate::common_structs::CommandResult;
use crate::common_structs::Priority;
use crate::common_structs::Todo;
use crate::data_service::add_todo;
use std::error::Error;

pub struct AddCommand {
    title: String,
    priority: Priority
}

impl AddCommand {
    pub fn new(title: &str, priority: Priority) -> AddCommand {
        AddCommand { title: title.to_string(), priority: priority }
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_priority(&self) -> Priority {
        self.priority
    }
}

pub fn execute(command : &CommandResult) -> Result<AddCommand, Box<dyn Error>> {
    //Validate required fields
    let add_command = parse_command(command)?;
    let todo = Todo::new(add_command.get_title(), add_command.get_priority());
    add_todo(todo)?;
    Ok(add_command)
}

fn parse_command(command : &CommandResult) -> Result<AddCommand, Box<dyn Error>> {
    //Check command required values
    if command.get_value().trim().is_empty() {
        return Err("Value cannot be empty".into());
    }
    let priority = match command.get_options().get("priority") {
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

    let add_command = AddCommand::new(command.get_value(), priority);
    Ok(add_command)
}

#[cfg(test)]
mod tests {
    use crate::add_command::parse_command;
    use crate::add_command::Priority;
    use crate::common_structs::Command;
    use crate::common_structs::CommandResult;
    use std::collections::HashMap;

    #[test]
    fn parse_command_with_no_value_return_error() {
        let command_result = CommandResult::new(Command::Add, 
                                                "", 
                                                HashMap::new());
        let _actual = match parse_command(&command_result) {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!("Value cannot be empty", e.to_string())
        };
    }

    #[test]
    fn parse_command_with_priority_value_empty_return_error() {
        let command_result = CommandResult::new(Command::Add, 
                                                "test", 
                                                HashMap::from([(String::from("priority"), String::from(""))]));
        let _actual = match parse_command(&command_result) {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!("Invalid priority value. Must be H, M or L", e.to_string())
        };
    }

    #[test]
    fn parse_command_with_priority_value_z_return_error() {
        let command_result = CommandResult::new(Command::Add, 
                                                "test", 
                                                HashMap::from([(String::from("priority"), String::from("z"))]));
        let _actual = match parse_command(&command_result) {
            Ok(_) => panic!("Test failed"),
            Err(e) => assert_eq!("Invalid priority value. Must be H, M or L", e.to_string())
        };
    }

    #[test]
    fn parse_command_with_no_priority_return_valid_addcommand() {
        let command_result = CommandResult::new(Command::Add, 
                                                "test", 
                                                HashMap::new());
        let _actual = match parse_command(&command_result) {
            Ok(add_command) => {
                assert_eq!("test", add_command.get_title());
                assert_eq!(Priority::Low, add_command.get_priority());
            }
            Err(_) => panic!("Test failed")
        };
    }

    fn parse_command_with_x_priority_return_valid_addcommand(priority: Priority, letter: &str) {
        let command_result = CommandResult::new(Command::Add, 
            "test", 
            HashMap::from([(String::from("priority"), String::from(letter))]));
        let _actual = match parse_command(&command_result) {
            Ok(add_command) => {
                assert_eq!("test", add_command.get_title());
                assert_eq!(priority, add_command.get_priority());
            }
            Err(_) => panic!("Test failed")
        };
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