use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{ErrorKind, Write},
};

#[derive(Serialize, Deserialize)]
pub struct ToDo<'a> {
    id: &'a str,
    description: &'a str,
    done: bool,
}

pub fn save_todo(todo: ToDo) {
    let json = serde_json::to_string(&todo).unwrap_or_else(|_| {
        panic!("Error, could not covert to JSON");
    });

    let file = File::open("db.json").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("db.json").unwrap_or_else(|error| {
                panic!("Problem creating a new database: {:?}", error);
            })
        } else {
            panic!("Problem opening the database: {:?}", error);
        }
    });

    std::fs::write(
        "db.json",
        serde_json::to_string_pretty(&json).unwrap(),
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::core::db::{save_todo, ToDo};

    #[test]
    fn save_a_todo() {
        let to_do = ToDo {
            id: "123",
            description: "Test todo",
            done: false,
        };
        save_todo(to_do);
    }
}
