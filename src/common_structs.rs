use std::collections::HashMap;

#[derive(Eq, Copy)]
pub enum Command {
    Unknown,
    Add,
    Edit,
    Done,
    Delete
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

#[derive(Eq, Copy, Debug)]
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