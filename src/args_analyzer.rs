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
    _options: HashMap<String, String>
}

impl CommandResult {
    pub fn get_command(&self) -> Command {
        self.command
    }

    pub fn get_value(&self) -> String {
        self.value.to_string()
    }
}

fn extract_command(value: String) -> Option<Command> {
    let trimmed_lowercase_value = value.trim().to_lowercase();
    match trimmed_lowercase_value.as_str() {
       "" => None,
       "add" => Some(Command::Add),
       "edit" => Some(Command::Edit),
       "done" => Some(Command::Done),
       "delete" => Some(Command::Delete),
       _ => Some(Command::Unknown)
    }
}

fn get_option_patterns() -> Vec<String> {
    vec![String::from("-p="),
         String::from("--priority=")]
}

fn extract_value(values: Vec<String>) -> String {
    let mut retval = String::new();
    let option_patterns = get_option_patterns();
    for value in values {
        let mut pattern_found = false;
        for pattern in &option_patterns {
            if value.starts_with(pattern.as_str()) {
                pattern_found = true;
                break;
            }
        }
        if pattern_found {
            break;
        }
        else {
            retval = format!("{} {}", retval, value);
        }
    }
    retval.trim_start().to_string()
}

pub fn analyze_args(args: Vec<String>) -> Option<CommandResult> {
    if !args.is_empty() {
        let command_extracted = extract_command(args[0].to_string());

        let mut value_extracted: String = String::from("");
        if args.len() > 1 {
            value_extracted = extract_value(args[1..].to_vec());
        }
        match command_extracted {
            Some(command_extracted) => Some(CommandResult { 
                                                          command: command_extracted, 
                                                          value: value_extracted,
                                                          _options: HashMap::new()
                                                          }),
            None => None
        }
    }
    else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::args_analyzer::analyze_args;
    use crate::args_analyzer::Command;

    #[test]
    fn command_equality_unkown_add_return_false() {
        assert!(Command::Unknown != Command::Add);
    }

    #[test]
    fn command_equality_unkown_unknown_return_true() {
        assert!(Command::Unknown == Command::Unknown);
    }

    #[test]
    fn command_equality_add_add_return_true() {
        assert!(Command::Add == Command::Add);
    }

    #[test]
    fn analyze_args_with_empty_vec_return_none() {
        assert!(analyze_args(vec![]).is_none());
    }

    #[test]
    fn analyze_args_with_one_empty_string_return_none() {
        assert!(analyze_args(vec![String::from("")]).is_none());
    }

    #[test]
    fn analyze_args_with_one_whitespaces_string_return_none() {
        assert!(analyze_args(vec![String::from("   ")]).is_none());
    }

    #[test]
    fn analyze_args_with_blabla_string_command_return_unknown_command() {
        assert!(Command::Unknown == analyze_args(vec![String::from("blabla")]).unwrap().get_command());
    }

    #[test]
    fn analyze_args_with_add_lowercase_string_command_return_add_command() {
        assert!(Command::Add == analyze_args(vec![String::from("add")]).unwrap().get_command());
    }

    #[test]
    fn analyze_args_with_add_uppercase_string_command_return_add_command() {
        assert!(Command::Add == analyze_args(vec![String::from("ADD")]).unwrap().get_command());
    }

    #[test]
    fn analyze_args_with_add_uppercase_withspaces_string_command_return_add_command() {
        assert!(Command::Add == analyze_args(vec![String::from("   ADD    ")]).unwrap().get_command());
    }

    #[test]
    fn analyze_args_with_add_with_singleword_return_add_valid_command() {
        let command_result = analyze_args(vec![String::from("add"), String::from("test")]).unwrap();
        assert!(Command::Add == command_result.get_command());
        assert_eq!("test", command_result.get_value());
    }

    #[test]
    fn analyze_args_with_add_with_twowords_return_add_valid_command() {
        let command_result = analyze_args(vec![String::from("add"), 
                                               String::from("test"),
                                               String::from("two")]).unwrap();
        assert!(Command::Add == command_result.get_command());
        assert_eq!("test two", command_result.get_value());
    }

    #[test]
    fn analyze_args_with_add_with_twowords_and_one_option_return_add_valid_command() {
        let command_result = analyze_args(vec![String::from("add"), 
                                               String::from("test"),
                                               String::from("two"),
                                               String::from("-p=H")]).unwrap();
        assert!(Command::Add == command_result.get_command());
        assert_eq!("test two", command_result.get_value());
        //assert_eq!(1, command_result.());
    }
}