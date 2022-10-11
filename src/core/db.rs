use rusqlite::{Connection, MappedRows, Result};

pub struct ToDo {
    description: String,
    done: bool,
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
             description TEXT NOT NULL UNIQUE,
             done BOOL NOT NULL,
             created TEXT DEFAULT CURRENT_TIMESTAMP 
         )",
        [],
    )?;

    Ok(conn)
}

pub fn get_todos() -> Result<Vec<ToDo>> {
    println!("Gettting todoes");
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

    for todo in &todiloes {
        println!("{}", todo.description);
    }

    Ok(todiloes)
}

pub fn save_todo(to_do: ToDo) -> Result<ToDo> {
    let conn = get_db_connection()?;

    conn.execute(
        "INSERT INTO to_dos (description, done) values (?1, 0)",
        &[&to_do.description.to_string()],
    )?;

    Ok(to_do)
}

fn get_db_path() -> String {
    if cfg!(test) {
        ":memory:".to_string()
    } else {
        "./db.sql".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{get_db_connection, get_todos, save_todo, ToDo};

    #[test]
    fn save_a_todo() {
        let description = "Test description";
        let to_do = ToDo::new(description);
        let res = save_todo(to_do).unwrap();
        assert_eq!(&res.description, description);
    }

    #[test]
    fn save_and_load_todos() {
        let conn = get_db_connection();
        let description = "Cut the grass";
        let description_two = "Call Carl";
        let to_do = ToDo::new(description);
        let to_do2 = ToDo::new(description_two);
        save_todo(to_do, &conn).unwrap();
        save_todo(to_do2, &conn).unwrap();

        let todos = get_todos().unwrap();
        // assert!(&todos.iter().any(|x| x.description == description_two));
    }
}
