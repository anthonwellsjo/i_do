use rusqlite::{Connection, Result};

static TEST_DB_PATH: &str = "./tests.sql";
static DB_PATH: &str = "./db.sql";

pub struct ToDo {
    pub description: String,
    pub done: bool,
}

impl ToDo {
    pub fn new(description: &str) -> ToDo {
        ToDo {
            description: description.to_owned(),
            done: false,
        }
    }
}

pub fn get_db_connection() -> Result<Connection> {
    let conn = Connection::open(get_db_path())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS to_dos (
             id INTEGER PRIMARY KEY,
             description TEXT NOT NULL,
             done BOOL NOT NULL,
             created TEXT DEFAULT CURRENT_TIMESTAMP 
         )",
        [],
    )?;

    Ok(conn)
}

/// Gets all todos from the database
/// # Examples
/// ```
/// use core::db::get_todos;
/// let res = get_todos();
/// ```
pub fn get_todos() -> Result<Vec<ToDo>> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT description, done
         FROM to_dos",
    )?;

    let to_dos = stmt.query_map([], |row| {
        Ok(ToDo {
            description: row.get(0)?,
            done: row.get(1)?,
        })
    })?;

    let mut todiloes: Vec<ToDo> = Vec::new();

    for todo in to_dos {
        let greeting_file = match todo {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        todiloes.push(greeting_file);
    }

    Ok(todiloes)
}

/// Saves a todo to the database
/// # Arguments
/// * `to_do` - In instance of the ToDo struct that will be saved.
/// # Examples
/// ```
/// use core::db::{ToDo, save_todo};
/// let to_do = ToDo::new("Fix the bike wheel");
/// let res = save_todo(to_do);
/// assert_eq!(res, Ok(()));
/// ```
pub fn save_todo_to_db(to_do: ToDo) -> Result<ToDo> {
    let conn = get_db_connection()?;

    conn.execute(
        "INSERT INTO to_dos (description, done) values (?1, 0)",
        &[&to_do.description.to_string()],
    )?;

    conn.close()
        .unwrap_or_else(|_| panic!("Panickin while closing conection."));

    Ok(to_do)
}

fn get_db_path() -> &'static str {
    if cfg!(test) {
        &TEST_DB_PATH
    } else {
        &DB_PATH
    }
}

#[cfg(test)]
mod tests {

    use super::{get_todos, save_todo, ToDo, TEST_DB_PATH};
    use std::fs;

    #[test]
    fn save_a_todo() {
        let description = "Test description";
        let to_do = ToDo::new(description);
        let res = save_todo(to_do).unwrap();
        assert_eq!(&res.description, description);
    }

    #[test]
    fn save_and_load_todos() {
        let description = "Cut the grass";
        let description_two = "Call Carl";
        let to_do = ToDo::new(description);
        let to_do2 = ToDo::new(description_two);
        save_todo(to_do).unwrap();
        save_todo(to_do2).unwrap();

        let todos = get_todos().unwrap();
        assert!(&todos.iter().any(|x| x.description == description_two));
    }

    #[test]
    #[ignore]
    fn cleanup_test_database() {
        fn remove_test_db() {
            fs::remove_file(&TEST_DB_PATH)
                .unwrap_or_else(|err| panic!("Panicking while deleting test database: {}", err));
        }
        remove_test_db();
    }
}
