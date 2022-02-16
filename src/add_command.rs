use crate::args_analyzer::CommandResult;
use crate::common_structs::Priority;
use std::error::Error;

struct AddCommand {
    title: String,
    priority: Priority
}

pub fn execute(command : &CommandResult) -> Result<(), Box<dyn Error>> {
    //Validate required fields
    println!("test {}", command.get_value());
    Ok(())
}

/*fn parse_command() -> Result<AddCommand, Box<dyn Error>> {
    Ok()
}*/

#[cfg(test)]
mod tests {
    //use crate::add_command::parse_command;

    #[test]
    fn validate_command_with_no_value_return_error() {
        //validate_command()
    }
}