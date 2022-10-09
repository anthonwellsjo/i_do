use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToDo {
    id: String,
    description: String,
    done: bool,
}

impl ToDo {
    pub fn new(description: &str) -> ToDo {
        ToDo {
            id: "123".to_owned(),
            description: description.to_owned(),
            done: false,
        }
    }
}

pub fn save_todo(todo: ToDo) {
    let json = serialize_todo(todo);

    std::fs::write(get_db_path(), serde_json::to_string_pretty(&json).unwrap()).unwrap();
}

fn serialize_todo(todo: ToDo) -> String {
    serde_json::to_string(&todo).unwrap_or_else(|_| {
        panic!("Error, could not covert to JSON");
    })
}

fn get_db_path() -> String {
    if cfg!(test) {
        "test_db.json".to_string()
    } else {
        "db.json".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::db::{save_todo, ToDo};

    #[test]
    fn save_a_todo() {
        let to_do = ToDo::new("Test description");
        save_todo(to_do);
    }
}
