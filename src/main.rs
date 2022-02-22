mod args_analyzer;
mod add_command;
mod common_structs;
mod data_service;
mod done_command;
mod list_command;

use crate::common_structs::{Command, ExecutableCommand};
use termion::{color, style};
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || (args[1] == "-h" || args[1] == "--help") {
        print_usage();
        return;
    }
    if args.len() >= 1 && args[1] == "-v" || args[1] == "--version" {
        print_version();
        return;
    }
    let result_command = args_analyzer::analyze_args(args[1..].to_vec());
    match result_command {
        Some(result_command)=> 
            match result_command.get_command() {
                Command::Unknown => eprintln!("{}beaver: Unknown command", color::Fg(color::Red)),
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
                },
                Command::Done => {
                    match done_command::DoneCommand::new_from_command_result(&result_command) {
                        Ok(command) => execute_command(command),
                        Err(e) => print_command_error(e)
                    }
                }
                _ => eprintln!("{}Not implemented command", color::Fg(color::Red))
            } 
        None => {
            //TODO: Print the command usage
            eprintln!("{}beaver: no command provided", color::Fg(color::Red))
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
    eprintln!("{}beaver: {}", color::Fg(color::Red), error);
}

fn print_version() {
    println!("Beaver todos");
    println!("Version 0.1");
    println!("Source : https://github.com/jeremydumais/BeaverTodos");
}

fn print_usage()
{
    println!("Beaver todos\n");
    println!("USAGE:");
    println!("beaver command [OPTIONS]\n");
    println!("COMMANDS:");
    println!("    -v, --version                  Print version info and exit");
    println!("    -h, --help                     Prints help information");
    println!("    list                           Display the todo list");
    println!("    add                            Add a new todo");
    println!("    done                           Complete a todo");
    println!("");
    println!("USAGE BY COMMAND:");
    println!("{}add{} title [OPTIONS]", style::Underline, style::NoUnderline);
    println!("    title                          The title (text) of the todo");
    println!("    -p=x, --priority=x             The priority of the todo, possible values are H, M and L");
    println!("                                   for High, Medium and Low");
    println!("{}list{} [OPTIONS]", style::Underline, style::NoUnderline);
    println!("    -a, --all                      Display all todos even those who are completed");
    println!("    -s=x, --sort=x                 Sort the todo list by one of the following:");
    println!("                                   {}prioritydesc:{} Sort by priority from High to Low (Default)", style::Underline, style::NoUnderline);
    println!("                                   {}priority:{} Sort by priority from Low to High", style::Underline, style::NoUnderline);
    println!("                                   {}creationtimedesc:{} Sort by creation time by more to less recent", style::Underline, style::NoUnderline);
    println!("                                   {}creationtime:{} Sort by creation time by less to more recent", style::Underline, style::NoUnderline);
    println!("{}done{} id", style::Underline, style::NoUnderline);
    println!("    id                             The id of the todo to complete");
    println!("\n");
}