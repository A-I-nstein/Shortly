use rusqlite::Connection;

#[derive(Debug)]
pub struct ShortlyRecord {
    pub short_url_base: String,
    pub long_url: String
}

pub fn create_db() {
    let conn: Connection = Connection::open("shortly.db").unwrap();
    let result: Result<usize, rusqlite::Error> = conn.execute(
        "CREATE TABLE long_to_short (
            long_url TEXT NOT NULL,
            short_url TEXT NOT NULL
        )",
        []
    );
    match result {
        Ok(_) => println!("Created table."),
        Err(err) => {
            eprintln!("Database creation failed: {}", err);
        }
    }
}

pub fn insert_record(record: &ShortlyRecord) {
    let conn: Connection = Connection::open("shortly.db").unwrap();
    match conn.execute("INSERT INTO long_to_short (long_url, short_url) VALUES (?1, ?2)", [record.long_url.clone(), record.short_url_base.clone()]) {
        Ok(updated) => println!("Record inserted ({}).", updated),
        Err(err) => println!("update failed: {}", err),
    };
}

pub fn show_records() {
    let conn: Connection = Connection::open("shortly.db").unwrap();
    let mut stmt = conn.prepare("SELECT long_url, short_url FROM long_to_short").unwrap();
    let record_iter = stmt.query_map([], |row| {
        Ok(ShortlyRecord {
            long_url: row.get(0)?,
            short_url_base: row.get(1)?,
        })
    }).unwrap();
    for record in record_iter {
        println!("{:?}", record.unwrap());
    }  
}

pub fn clear_db() {
    let conn: Connection = Connection::open("shortly.db").unwrap();
    match conn.execute("DELETE FROM long_to_short", []) {
        Ok(updated) => println!("Records removed ({}).", updated),
        Err(err) => println!("update failed: {}", err),
    };
}