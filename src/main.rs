extern crate rusqlite;
use rusqlite::{params, Connection, Result};
use std::io;

enum Command {
    AddUser,
    ListUsers,
    UpdateUser,
    DeleteUser,
    Exit,
    Invalid,
}

fn add_user(conn: &Connection, name: &str, age: i32, address: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, age, address) VALUES (?1, ?2, ?3)",
        params![name, &age, address],
    )?;
    println!("User added successfully!");
    Ok(())
}

fn list_users(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, age, address FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;

    for user in user_iter {
        let (id, name, age, address) = user?;
        println!("ID: {}, Name: {}, Age: {}, Address: {}", id, name, age, address);
    }

    Ok(())
}

fn update_user(conn: &Connection, id: i32, new_name: &str, new_age: i32, new_address: &str) -> Result<()> {
    conn.execute(
        "UPDATE users SET name = ?2, age = ?3, address = ?4 WHERE id = ?1",
        params![id, new_name, new_age, new_address],
    )?;
    println!("User updated successfully!");
    Ok(())
}

fn delete_user(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM users WHERE id = ?", params![id])?;
    println!("User deleted successfully!");
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open("rust_sqlite_cli.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL,
            address TEXT NOT NULL
        )",
        [],
    )?;

    loop {
        println!("Rust SQLite CLI Menu:");
        println!("1. Add user");
        println!("2. List users");
        println!("3. Update user");
        println!("4. Delete user");
        println!("5. Exit");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let cmd = match input.trim().parse::<u32>() {
            Ok(1) => Command::AddUser,
            Ok(2) => Command::ListUsers,
            Ok(3) => Command::UpdateUser,
            Ok(4) => Command::DeleteUser,
            Ok(5) => Command::Exit,
            _ => Command::Invalid,
        };

        match cmd {
            Command::AddUser => {
                println!("Enter name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");

                println!("Enter age: ");
                let mut age_input = String::new();
                io::stdin().read_line(&mut age_input).expect("Failed to read age");
                let age: i32 = age_input.trim().parse().expect("Please enter a valid number for age");

                println!("Enter address: ");
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Failed to read address");

                add_user(&conn, &name.trim(), age, &address.trim())?;
            },
            Command::ListUsers => {
                list_users(&conn)?;
            },
            Command::UpdateUser => {
                println!("Enter user ID to update: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read ID");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid number for ID");

                println!("Enter new name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read name");

                println!("Enter new age: ");
                let mut age_input = String::new();
                io::stdin().read_line(&mut age_input).expect("Failed to read age");
                let age: i32 = age_input.trim().parse().expect("Please enter a valid number for age");

                println!("Enter new address: ");
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Failed to read address");

                update_user(&conn, id, &name.trim(), age, &address.trim())?;
            },
            Command::DeleteUser => {
                println!("Enter user ID to delete: ");
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).expect("Failed to read ID");
                let id: i32 = id_input.trim().parse().expect("Please enter a valid number for ID");

                delete_user(&conn, id)?;
            },
            Command::Exit => {
                break;
            },
            Command::Invalid => {
                println!("Invalid option, please try again.");
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 初始化用于测试的数据库
    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                address TEXT NOT NULL
            )",
            [],
        ).unwrap();
        conn
    }

    #[test]
    fn test_add_user() {
        let conn = setup_db();
        
        add_user(&conn, "Alice", 30, "123 Street").unwrap();
        
        let mut stmt = conn.prepare("SELECT name, age, address FROM users WHERE name = 'Alice'").unwrap();
        let user_iter = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
            ))
        }).unwrap();

        let mut found = false;
        for user in user_iter {
            let (name, age, address) = user.unwrap();
            assert_eq!(name, "Alice");
            assert_eq!(age, 30);
            assert_eq!(address, "123 Street");
            found = true;
        }
        assert!(found, "User Alice should have been found");
    }

    #[test]
    fn test_update_user() {
        let conn = setup_db();
        add_user(&conn, "Bob", 25, "456 Lane").unwrap();
        
        let user_id = conn.last_insert_rowid();
        update_user(&conn, user_id as i32, "Bobby", 26, "789 Road").unwrap();

        let mut stmt = conn.prepare("SELECT name, age, address FROM users WHERE id = ?1").unwrap();
        let user_iter = stmt.query_map(params![user_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i32>(1)?,
                row.get::<_, String>(2)?,
            ))
        }).unwrap();

        let mut found = false;
        for user in user_iter {
            let (name, age, address) = user.unwrap();
            assert_eq!(name, "Bobby");
            assert_eq!(age, 26);
            assert_eq!(address, "789 Road");
            found = true;
        }
        assert!(found, "Updated user should have been found");
    }

    #[test]
    fn test_delete_user() {
        let conn = setup_db();
        add_user(&conn, "Charlie", 35, "101 Ave").unwrap();
        
        let user_id = conn.last_insert_rowid();
        delete_user(&conn, user_id as i32).unwrap();

        let mut stmt = conn.prepare("SELECT name FROM users WHERE id = ?1").unwrap();
        let user: Result<String> = stmt.query_row(params![user_id], |row| row.get(0));

        assert!(user.is_err(), "User Charlie should have been deleted");
    }

    #[test]
    fn test_list_users() {
        let conn = setup_db();
        add_user(&conn, "David", 40, "202 Blvd").unwrap();
        add_user(&conn, "Eve", 45, "303 Cir").unwrap();

        let mut count = 0;
        let mut stmt = conn.prepare("SELECT name FROM users").unwrap();
        let user_iter = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        }).unwrap();
        
        for user in user_iter {
            let name = user.unwrap();
            assert!(["David", "Eve"].contains(&name.as_str()));
            count += 1;
        }
        assert_eq!(count, 2, "Should have listed 2 users");
    }
}
