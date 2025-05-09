#[derive(Debug)]
pub struct TodoList {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub done: i32
}

impl TodoList {
    pub fn new(name: &str, description: &str) -> TodoList {
        TodoList {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            done: 0
        }
    }
}
