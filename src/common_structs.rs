use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Eq, Copy)]
pub enum Command {
    Unknown,
    Add,
    Edit,
    Done,
    Delete,
    List
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        *self as u8 == *other as u8
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct CommandResult
{
    command: Command,
    value: String,
    options: HashMap<String, String>
}

impl CommandResult {
    pub fn new(command : Command, 
               value : &str,
               options : HashMap<String, String>) -> CommandResult {
        CommandResult {
            command: command,
            value: String::from(value),
            options: options
        }
    }

    pub fn get_command(&self) -> Command {
        self.command
    }

    pub fn get_value(&self) -> &str {
        self.value.as_str()
    }

    pub fn get_options(&self) -> &HashMap<String, String> {
        &self.options
    }
}

#[derive(Eq, Copy, Debug, Deserialize, Serialize)]
pub enum Priority {
    High,
    Medium,
    Low
}

impl PartialEq for Priority {
    fn eq(&self, other: &Priority) -> bool {
        *self as u8 == *other as u8
    }
}

impl Clone for Priority {
    fn clone(&self) -> Self {
        *self
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        match self {
            Priority::High => String::from("High"),
            Priority::Medium => String::from("Medium"),
            Priority::Low => String::from("Low")
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    id: u32,
    title: String,
    priority: Priority
}

impl Todo {
    pub fn new(title: &str, priority: Priority) -> Todo {
        Todo { id: 0, title: title.to_string(), priority: priority }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_title(&self) -> &str {
        self.title.as_str()
    }

    pub fn get_priority(&self) -> Priority {
        self.priority
    }

    pub fn set_id(& mut self, id: u32) {
        self.id = id;
    }
}

#[cfg(test)]
mod tests {
    use crate::common_structs::Command;
    use crate::common_structs::CommandResult;
    use crate::common_structs::Priority;
    use std::collections::HashMap;

    #[test]
    fn command_equality_with_same_return_true() {
        assert_eq!(Command::Add, Command::Add);
    }

    #[test]
    fn command_equality_with_different_return_true() {
        assert_ne!(Command::Add, Command::Delete);
    }

    #[test]
    fn command_clone_return_valid_copy() {
        let mut actual = Command::Add;
        let clone = actual.clone();
        actual = Command::Delete;
        assert_eq!(actual, Command::Delete);
        assert_eq!(clone, Command::Add);
    }

    fn get_sample_add_command() -> CommandResult {
        CommandResult::new(Command::Add, 
            "Test", 
            HashMap::from([(String::from("priority"), String::from("H"))]))
    }

    #[test]
    fn commandresult_new_return_new_instance() {
        let cr = get_sample_add_command();
        assert_eq!(Command::Add, cr.get_command());
        assert_eq!("Test", cr.get_value());
        let options = cr.get_options();
        assert_eq!(1, options.len());
        assert!(options.contains_key("priority"));
        assert_eq!("H", options.get("priority").unwrap());
    }

    #[test]
    fn commandresult_get_command_return_command() {
        let cr = get_sample_add_command();
        assert_eq!(Command::Add, cr.get_command());
    }

    #[test]
    fn commandresult_get_value_return_value() {
        let cr = get_sample_add_command();
        assert_eq!("Test", cr.get_value());
    }

    #[test]
    fn commandresult_get_options_return_value() {
        let cr = get_sample_add_command();
        let options = cr.get_options();
        assert_eq!(1, options.len());
        assert!(options.contains_key("priority"));
        assert_eq!("H", options.get("priority").unwrap());
    }

    
}