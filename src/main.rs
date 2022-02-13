mod args_analyzer;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let result_command = args_analyzer::analyze_args(args[1..].to_vec());
    match result_command {
        Some(result_command)=> 
            match result_command.get_command() {
                args_analyzer::Command::Unknown => println!("Unknown command"),
                _ => println!("valid command")
            } 
        None => println!("No command provided")
    }
}
