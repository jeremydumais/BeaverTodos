mod args_analyzer;
mod add_command;
mod common_structs;
mod data_service;
mod list_command;

use crate::common_structs::Command;
use crate::common_structs::ExecutableCommand;
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result_command = args_analyzer::analyze_args(args[1..].to_vec());
    match result_command {
        Some(result_command)=> 
            match result_command.get_command() {
                Command::Unknown => eprintln!("beaver: Unknown command"),
                Command::Add => {
                    match add_command::AddCommand::new_from_command_result(&result_command) {
                        Ok(command) => execute_command(command),
                        Err(e) => print_command_error(e)
                    }
                },
                Command::List => {
                    match list_command::ListCommand::new_from_command_result(&result_command) {
                        Ok(command) => execute_command(command),
                        Err(e) => print_command_error(e)
                    }
                }
                _ => eprintln!("Not implemented command")
            } 
        None => {
            //TODO: Print the command usage
            eprintln!("beaver: no command provided")
        }
    }
}

fn execute_command(command: impl ExecutableCommand) {
    match command.execute() {
        Ok(_) => (),
        Err(e) => print_command_error(e)
    }
}

fn print_command_error(error: Box<dyn Error>) {
    eprintln!("beaver: Command failed {}", error);
}