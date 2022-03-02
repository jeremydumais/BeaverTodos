use crate::common_structs::Command;
use crate::common_structs::CommandResult;
use std::collections::HashMap;

fn extract_command(value: String) -> Option<Command> {
    let trimmed_lowercase_value = value.trim().to_lowercase();
    match trimmed_lowercase_value.as_str() {
       "" => None,
       "add" => Some(Command::Add),
       "edit" => Some(Command::Edit),
       "delete" => Some(Command::Delete),
       "done" => Some(Command::Done),
       "fetch" => Some(Command::Fetch),
       "list" => Some(Command::List),
       "next" => Some(Command::Next),
       "purge" => Some(Command::Purge),
       "remove" => Some(Command::Remove),
       _ => Some(Command::Unknown)
    }
}

fn get_option_patterns() -> Vec<String> {
    vec![String::from("-p="),
         String::from("--priority="),
         String::from("-t="),
         String::from("--title="),
         String::from("-s="),
         String::from("--sort="),
         String::from("-a"),
         String::from("--all")]
}

fn get_option_name_from_pattern(value: &str) -> Option<String> {
    let option_patterns = get_option_patterns();
    let mut result: Option<String> = None;
    for pattern in option_patterns {

        if (!pattern.ends_with('=') && value == pattern) || 
           (pattern.ends_with('=') && value.starts_with(&pattern)) {
            result = match pattern.as_str() {
                "-p=" => Some(String::from("priority")),
                "--priority=" => Some(String::from("priority")),
                "-t=" => Some(String::from("title")),
                "--title=" => Some(String::from("title")),
                "-s=" => Some(String::from("sort")),
                "--sort=" => Some(String::from("sort")),
                "-a" => Some(String::from("all")),
                "--all" => Some(String::from("all")),
                _ => None
            }
        }
    }
    result
}

fn extract_value(values: &Vec<String>) -> String {
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

fn extract_options(values: &Vec<String>) -> HashMap<String, String> {
    let mut retval = HashMap::new();
    let mut option_found = false;
    let mut current_option_name = String::new();
    let mut current_option_value = String::new();
    for value in values {
        match get_option_name_from_pattern(&value) {
            Some(option) => {
                if option_found {
                    //Add the last option found
                    retval.insert(current_option_name, current_option_value);
                }
                option_found = true;
                current_option_name = option;
                current_option_value = extract_option_value(&value);
                }
            _ => {
                if option_found {
                    if current_option_value.len() > 0 {
                        current_option_value.push(' ');
                    }
                    current_option_value.push_str(&value);
                }
            }
        }           
    }
    if option_found {
        //Add the last option
        retval.insert(current_option_name, current_option_value);
    }
    retval
}

fn extract_option_value(option: &str) -> String {
    let mut retval = String::new();
    let option_parts: Vec<_> = option.split('=').collect();
    if option_parts.len() > 1 {
        for elem in option_parts[1..].to_vec() {
            if retval.len() > 0 {
                retval.push('=');
            }
            retval.push_str(elem);
        }
    }
    retval.trim_end().to_string()
}

pub fn analyze_args(args: Vec<String>) -> Option<CommandResult> {
    if !args.is_empty() {
        let command_extracted = extract_command(args[0].to_string());

        let mut value_extracted: String = String::new();
        let mut options_extracted: HashMap<String, String> = HashMap::new();
        if args.len() > 1 {
            let values_and_options = args[1..].to_vec();
            value_extracted = extract_value(&values_and_options);
            options_extracted = extract_options(&values_and_options);
        }
        match command_extracted {
            Some(command_extracted) => Some(CommandResult::new(command_extracted, 
                                                               value_extracted.as_str(),
                                                               options_extracted)),
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
    use crate::args_analyzer::extract_option_value;
    use crate::args_analyzer::get_option_name_from_pattern;

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
        let actual_options = command_result.get_options();
        assert_eq!(1, actual_options.len());
        assert!(actual_options.contains_key("priority"));
        assert_eq!("H", actual_options["priority"]);
    }

    #[test]
    fn analyze_args_with_add_with_twowords_and_one_option_long_return_add_valid_command() {
        let command_result = analyze_args(vec![String::from("add"), 
                                               String::from("test"),
                                               String::from("two"),
                                               String::from("--priority=H")]).unwrap();
        assert!(Command::Add == command_result.get_command());
        assert_eq!("test two", command_result.get_value());
        let actual_options = command_result.get_options();
        assert_eq!(1, actual_options.len());
        assert!(actual_options.contains_key("priority"));
        assert_eq!("H", actual_options["priority"]);
    }

    #[test]
    fn analyze_args_with_add_with_twowords_and_one_option_value_with_spaces_return_add_valid_command() {
        let command_result = analyze_args(vec![String::from("add"), 
                                               String::from("test"),
                                               String::from("two"),
                                               String::from("--priority=H"),
                                               String::from("and"),
                                               String::from("P")]).unwrap();
        assert!(Command::Add == command_result.get_command());
        assert_eq!("test two", command_result.get_value());
        let actual_options = command_result.get_options();
        assert_eq!(1, actual_options.len());
        assert!(actual_options.contains_key("priority"));
        assert_eq!("H and P", actual_options["priority"]);
    }

    #[test]
    fn analyze_args_with_add_with_twowords_and_two_options_return_add_valid_command() {
        let command_result = analyze_args(vec![String::from("add"), 
                                               String::from("test"),
                                               String::from("two"),
                                               String::from("--priority=H"),
                                               String::from("and"),
                                               String::from("P"),
                                               String::from("--title=Test"),
                                               String::from("again")]).unwrap();
        assert!(Command::Add == command_result.get_command());
        assert_eq!("test two", command_result.get_value());
        let actual_options = command_result.get_options();
        assert_eq!(2, actual_options.len());
        assert!(actual_options.contains_key("priority"));
        assert_eq!("H and P", actual_options["priority"]);
        assert!(actual_options.contains_key("title"));
        assert_eq!("Test again", actual_options["title"]);
    }

    #[test]
    fn get_option_name_from_pattern_with_p_return_priority() {
        let actual = get_option_name_from_pattern("-p=H");
        match actual {
            Some(value) => assert_eq!("priority", value),
            None => panic!("Test failed")
        }
    }

    #[test]
    fn get_option_name_from_pattern_with_priority_return_priority() {
        let actual = get_option_name_from_pattern("--priority=H");
        match actual {
            Some(value) => assert_eq!("priority", value),
            None => panic!("Test failed")
        }
    }

    #[test]
    fn get_option_name_from_pattern_with_blabla_return_none() {
        assert!(get_option_name_from_pattern("--blabla=H").is_none());
    }

    #[test]
    fn get_option_name_from_pattern_with_long_all_return_all() {
        assert_eq!("all", get_option_name_from_pattern("--all").unwrap());
    }

    #[test]
    fn get_option_name_from_pattern_with_long_allocator_return_none() {
        assert!(get_option_name_from_pattern("--allocator").is_none());
    }

    #[test]
    fn get_option_name_from_pattern_with_short_all_return_all() {
        assert_eq!("all", get_option_name_from_pattern("-a").unwrap());
    }

    #[test]
    fn get_option_name_from_pattern_with_short_allocator_return_none() {
        assert!(get_option_name_from_pattern("-allocator").is_none());
    }

    #[test]
    fn extract_option_value_with_p_equal_h_return_h() {
        assert_eq!("H".to_string(), extract_option_value("-p=H"));
    }

    #[test]
    fn extract_option_value_with_p_equal_hequalh_return_hequalh() {
        assert_eq!("H=H".to_string(), extract_option_value("-p=H=H"));
    }

    #[test]
    fn extract_option_value_with_p_equal_hequalhequal_return_hequalhequal() {
        assert_eq!("H=H=".to_string(), extract_option_value("-p=H=H="));
    }

    #[test]
    fn extract_option_value_with_p_equal_return_empty() {
        assert_eq!("".to_string(), extract_option_value("-p="));
    }

    #[test]
    fn extract_option_value_with_p_return_empty() {
        assert_eq!("".to_string(), extract_option_value("-p"));
    }
}