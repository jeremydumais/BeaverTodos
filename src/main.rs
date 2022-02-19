mod args_analyzer;
mod add_command;
mod common_structs;
mod data_service;
mod list_command;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result_command = args_analyzer::analyze_args(args[1..].to_vec());
    match result_command {
        Some(result_command)=> 
            match result_command.get_command() {
                common_structs::Command::Unknown => println!("Unknown command"),
                common_structs::Command::Add => {
                    let res = add_command::execute(&result_command);
                    match res {
                        Ok(add_command) => println!("The todo {} has been added!", add_command.get_title()),
                        Err(e) => eprintln!("Command failed {}", e)
                    }
                },
                common_structs::Command::List => {
                    match list_command::execute(&result_command) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Command failed {}", e)
                    }
                }
                _ => println!("valid command")
            } 
        None => eprintln!("No command provided")
    }
}
