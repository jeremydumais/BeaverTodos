use crate::common_structs::ExecutableCommand;
use crate::data_service::{read_all_todos, write_todos};
use question::{Question, Answer};
use termion::color;
use std::error::Error;

#[derive(Debug)]
pub struct PurgeCommand {
}

impl ExecutableCommand for PurgeCommand {
    fn execute(&self) -> Result<(), Box<dyn Error>> {
        let mut todos = read_all_todos()?;
        let before_count = todos.len();
        todos.retain(|x| !x.get_completed());
        let after_count = todos.len();
        let todos_count_to_remove = before_count - after_count;
        let answer = Question::new(format!("Are you sure you want to delete {} completed todos? (y/n)", todos_count_to_remove).as_str())
                                   .yes_no()
                                   .until_acceptable()
                                   .ask();
        if answer.unwrap_or(Answer::NO) == Answer::YES {
            write_todos(&todos)?;
            println!("{}The purge has removed {} completed todo(s)!", color::Fg(color::Green), todos_count_to_remove);
        }
        Ok(())
    }
}