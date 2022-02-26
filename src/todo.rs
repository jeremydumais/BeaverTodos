use crate::common_structs::Priority;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    id: u32,
    title: String,
    priority: Priority,
    #[serde(with = "utc_date_formatter")]
    when_created_utc: DateTime<Utc>,
    completed: bool,
    #[serde(with = "utc_date_formatter")]
    when_completed_utc: DateTime<Utc>
}

impl Todo {
    pub fn new(id: u32, title: &str, priority: Priority, when_created_utc: DateTime<Utc>) -> Result<Todo, Box<dyn Error>> {
        if title.trim().is_empty() {
            return Err("Title is required".into())
        }

        Ok(Todo { id: id, 
                  title: title.to_string(), 
                  priority: priority,
                  when_created_utc: when_created_utc,
                  completed: false,
                  when_completed_utc: Todo::get_default_completed_date()
                })
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

    pub fn get_when_created_in_localtime(&self) -> DateTime<Local> {
        DateTime::from(self.when_created_utc)
    }

    pub fn get_completed(&self) -> bool {
        self.completed
    }

    pub fn get_when_completed_in_localtime(&self) -> DateTime<Local> {
        DateTime::from(self.when_completed_utc)
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), Box<dyn Error>> {
        if title.trim().is_empty() {
            return Err("Title cannot be empty".into())
        }
        self.title = title.to_string();
        Ok(())
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority
    }

    pub fn set_completed(&mut self, value: bool, when_completed: Option<DateTime<Utc>>) {
        self.completed = value;
        match when_completed {
            Some(date) => self.when_completed_utc = date,
            None => self.when_completed_utc = Utc::now()
        }
    }

    pub fn get_default_completed_date() -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc)
    }
}

mod utc_date_formatter {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::common_structs::Priority;
    use crate::todo::Todo;
    use chrono::{DateTime, Local, Utc, TimeZone};

    fn get_sample_todo() -> Todo {
        Todo::new(1, "Test", Priority::Low, Utc::now()).unwrap()
    }

    #[test]
    fn todo_new_with_empty_title_return_error() {
        assert_eq!("Title is required", Todo::new(1, "", Priority::Low, Utc::now()).unwrap_err().to_string());
    }

    #[test]
    fn todo_new_with_whitespaces_title_return_error() {
        assert_eq!("Title is required", Todo::new(1, "   ", Priority::Low, Utc::now()).unwrap_err().to_string());
    }

    #[test]
    fn todo_get_id_with_one_return_one() {
        let actual = get_sample_todo();
        assert_eq!(1, actual.get_id());
    }

    #[test]
    fn todo_get_title_with_test_return_test() {
        let actual = get_sample_todo();
        assert_eq!("Test", actual.get_title());
    }

    #[test]
    fn todo_get_priority_with_low_return_low() {
        let actual = get_sample_todo();
        assert_eq!(Priority::Low, actual.get_priority());
    }

    #[test]
    fn todo_get_when_created_return_creation_tim() {
        let time_now = Utc::now();
        let local_time_now: DateTime<Local> = DateTime::from(time_now);
        let actual = Todo::new(1, "Test", Priority::Low, time_now).unwrap();

        assert_eq!(local_time_now, actual.get_when_created_in_localtime());
    }

    #[test]
    fn todo_get_completed_return_false() {
        let actual = get_sample_todo();
        assert_eq!(false, actual.get_completed());
    }

    #[test]
    fn todo_get_when_completed_return_completed_time() {
        let time_now = Utc::now();
        let actual = Todo::new(1, "Test", Priority::Low, time_now).unwrap();

        assert_eq!(Todo::get_default_completed_date(), actual.get_when_completed_in_localtime());
    }

    #[test]
    fn todo_set_id_with_2_return_success() {
        let mut actual = get_sample_todo();
        assert_eq!(1, actual.get_id());
        actual.set_id(2);
        assert_eq!(2, actual.get_id());
    }

    #[test]
    fn todo_set_title_with_empty_return_error() {
        let mut actual = get_sample_todo();
        assert_eq!("Title cannot be empty", actual.set_title("").unwrap_err().to_string());
    }

    #[test]
    fn todo_set_title_with_whitespaces_return_error() {
        let mut actual = get_sample_todo();
        assert_eq!("Title cannot be empty", actual.set_title("  ").unwrap_err().to_string());
    }


    #[test]
    fn todo_set_title_with_valid_value_return_success() {
        let mut actual = get_sample_todo();
        assert_eq!("Test", actual.get_title());
        const NEW_VALUE: &str = "Another value";
        actual.set_title(NEW_VALUE).unwrap();
        assert_eq!(NEW_VALUE, actual.get_title());
    }

    #[test]
    fn todo_set_priority_with_medium_return_success() {
        let mut actual = get_sample_todo();
        assert_eq!(Priority::Low, actual.get_priority());
        actual.set_priority(Priority::Medium);
        assert_eq!(Priority::Medium, actual.get_priority());
    }

    #[test]
    fn todo_set_completed_with_true_return_success() {
        let completed_date = Utc.ymd(1970, 2, 2).and_hms(1, 1, 1);
        let mut actual = get_sample_todo();

        assert_eq!(false, actual.get_completed());
        assert_eq!(Todo::get_default_completed_date(), actual.get_when_completed_in_localtime());
        actual.set_completed(true, Some(completed_date));
        assert_eq!(true, actual.get_completed());
        assert_eq!(completed_date, actual.get_when_completed_in_localtime());
    }
}