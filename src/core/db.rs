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

pub fn get_todos() -> Result<()> {
    let conn = get_db_connection()?;

    let mut stmt = conn.prepare(
        "SELECT description, done
         FROM to_dos;",
    )?;

    let to_dos = stmt.query_map([], |row| {
        Ok(ToDo {
            description: row.get(0)?,
            done: row.get(1)?,
        })
    })?;

    for todo in to_dos {
        todo.unwrap_or_else(|err| {
            panic!("Error while unpacking rows in ");
        });
    }

    Ok(())
}

pub fn save_todo(to_do: ToDo) -> Result<()> {
    let conn = get_db_connection()?;

    conn.execute(
        "INSERT INTO to_dos (description, done) values (?1, 0)",
        &[&to_do.description.to_string()],
    )?;

    Ok(())
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
    use crate::core::db::{save_todo, ToDo};

    #[test]
    fn save_a_todo_is_persistent() {
        let to_do = ToDo::new("Test description");
        let res = save_todo(to_do);
        assert_eq!(res, Ok(()));
    }
}
