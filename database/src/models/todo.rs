use std::fmt;

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


impl fmt::Display for TodoList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[#{}] {} — {} [{}]",
            self.id,
            self.name,
            self.description,
            if self.done == 1 { "✔ done" } else { "⧗ pending" }
        )
    }
}

