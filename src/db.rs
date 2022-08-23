use diesel::prelude::*;
use diesel::SqliteConnection;


pub struct DB {

}

impl DB {

    pub fn new() -> Self {
        DB {
            
        }
    }

    pub fn connect(&mut self) -> SqliteConnection {
        let e = SqliteConnection::establish("db.sqlite");
        match e {
            Err(e) => todo!(),
            Ok(conn) => {
                println!("Success with connecting");
                return conn;
            }
        }
    }
}