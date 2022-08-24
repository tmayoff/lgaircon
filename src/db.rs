use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::db::models::NewSetting;
use crate::lg_ac;

pub mod models;
pub mod schema;

pub struct DB {
    conn: Option<SqliteConnection>,
}

impl DB {

    pub fn new() -> Self {
        DB {
            conn: Some(Self::connect()),
        }
    }

    pub fn connect() -> SqliteConnection {
        SqliteConnection::establish("app.sqlite").expect("Failed to create/connect to DB")
    }

    pub fn add_setting<'a>(&mut self, name: &'a str, value: &'a str) {
        use schema::_state;
        let new_state = NewSetting {
            name,
            val: value,
        };

        match &self.conn {
            Some(conn) => {
                let ret = diesel::insert_into(_state::table)
                .values(&new_state)
                .execute(conn);
                ret.expect("Error adding new setting");
            }
            None => todo!(),
        }
    }

    pub fn update_setting<'a>(&mut self, name: &'a str, value: &'a str) {
        use schema::_state;
        let new_state = NewSetting {
            name,
            val: value,
        };

        match &self.conn {
            None => todo!(),
            Some(conn) => {
                let ret = diesel::update(_state::table.find(name)).set(&new_state).execute(conn);
                ret.expect("Error updating setting");
            }
        }
    }

    pub fn get_setting (&mut self, name: &str) -> Option<String> {
        use schema::_state;
        match &self.conn {
            None => todo!(),

            Some(conn) => {
                let ret = _state::table.filter(_state::name.eq(name)).load::<self::models::Setting>(conn);
                match ret {
                    Err(e) => {
                        return None;
                    }
                    Ok(vals) => {
                        let setting = vals[0].clone();
                        return Some(String::from(setting.val));
                    }
                }

            }
        }
    }

    pub fn update_state(&mut self, new_state: lg_ac::State) {
        use schema::_state;

        match &self.conn {
            None => todo!(),

            Some(conn) => {
                match self.get_setting("cur_temp") {
                    None => self.add_setting("cur_temp", new_state.cur_temp.to_string().as_str()),
                    Some(_) => self.update_setting("cur_temp", new_state.cur_temp.to_string().as_str()),
                }

            }
        }
    }
}