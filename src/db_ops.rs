use rusqlite::Connection;
use std::process::exit;

#[derive(Debug)]
pub struct ShortlyRecord {
    pub short_url_base: String,
    pub long_url: String,
}

fn get_conn() -> Connection {
    let connection: Result<Connection, rusqlite::Error> = Connection::open("shortly.db");
    match connection {
        Ok(conn) => return conn,
        Err(err) => {
            eprintln!("\nDatabase connection failed: {}", err);
            exit(1);
        }
    }
}

pub fn create_db() {
    let conn: Connection = get_conn();
    let result: Result<usize, rusqlite::Error> = conn.execute(
        "CREATE TABLE long_to_short (
            long_url TEXT NOT NULL,
            short_url TEXT NOT NULL
        )",
        [],
    );
    match result {
        Ok(_) => println!("\nDatabase and Table created."),
        Err(err) => {
            eprintln!("\nDatabase creation failed: {}", err);
            exit(1);
        }
    }
}

pub fn insert_record(record: &ShortlyRecord) {
    let conn: Connection = get_conn();
    match conn.execute(
        "INSERT INTO long_to_short (long_url, short_url) VALUES (?1, ?2)",
        [record.long_url.clone(), record.short_url_base.clone()],
    ) {
        Ok(_) => println!("\nRecord inserted."),
        Err(err) => {
            println!("\nUpdate failed: {}", err);
            exit(1);
        }
    };
}

pub fn get_record(short_url: &String) -> String {
    let conn: Connection = get_conn();
    let mut stmt = conn
        .prepare("SELECT long_url, short_url FROM long_to_short WHERE short_url = ?1")
        .unwrap();
    let mut record_iter = stmt
        .query_map([short_url], |row| {
            Ok(ShortlyRecord {
                long_url: row.get(0)?,
                short_url_base: row.get(1)?,
            })
        })
        .unwrap();
    match record_iter.next() {
        Some(record) => {
            return record.unwrap().long_url;
        }
        None => {
            return "".to_string();
        }
    }
}

pub fn show_records() {
    let conn: Connection = get_conn();
    let mut stmt = conn
        .prepare("SELECT long_url, short_url FROM long_to_short")
        .unwrap();
    let record_iter = stmt
        .query_map([], |row| {
            Ok(ShortlyRecord {
                long_url: row.get(0)?,
                short_url_base: row.get(1)?,
            })
        })
        .unwrap();
    println!();
    for record in record_iter {
        println!("{:?}", record.unwrap());
    }
}

pub fn clear_db() {
    let conn: Connection = get_conn();
    match conn.execute("DELETE FROM long_to_short", []) {
        Ok(updated) => println!("\nRecords removed ({}).", updated),
        Err(err) => {
            println!("\nUpdate failed: {}", err);
            exit(1);
        }
    };
}
