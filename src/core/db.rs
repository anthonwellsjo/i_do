use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ToDo {
    title: String,
    descriptions: String,
    phones: Vec<String>,
}
