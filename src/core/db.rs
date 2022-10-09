use serde::{Deserialize, Serialize};

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


}




#[cfg(test)]
mod tests {
    use crate::core::db::{ToDo, save_todo};

    #[test]
    fn save_a_todo() {
        let to_do = ToDo{id: "123", description: "Test todo", done: false};
        save_todo(to_do);
    }
}
