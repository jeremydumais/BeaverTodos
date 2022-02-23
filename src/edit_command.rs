#[derive(Debug)]
pub struct EditCommand {
    id: u32,
    title: Option<String>,
    priority: Option<Priority>
}