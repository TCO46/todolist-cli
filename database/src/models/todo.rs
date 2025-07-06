use std::fmt;

#[derive(Debug)]
pub struct TodoList {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub priority: u8,
    pub done: i32,
}

impl TodoList {
    pub fn new(name: &str, description: &str, priority: u8) -> TodoList {
        TodoList {
            id: 0,
            name: name.to_string(),
            description: description.to_string(),
            priority: priority,
            done: 0,
        }
    }
}

impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} — {} [{}]",
            self.name,
            self.description,
            if self.done == 1 {
                "✔ Done"
            } else {
                "⧗ In Progress"
            }
        )
    }
}
