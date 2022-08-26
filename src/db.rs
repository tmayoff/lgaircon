use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::db::models::NewSetting;
use crate::embedded_migrations;
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

    pub fn run_migrations(&mut self) {
        match &self.conn {
            None => todo!(),
            Some(c) => {
                embedded_migrations::run(c).unwrap()
            }
        }
    }

    pub fn add_setting<'a>(&mut self, name: &'a str, value: &'a str) {
        use schema::state;
        let new_state = NewSetting {
            name,
            val: value,
        };

        match &self.conn {
            Some(conn) => {
                let ret = diesel::insert_into(state::table)
                .values(&new_state)
                .execute(conn);
                ret.expect("Error adding new setting");
            }
            None => todo!(),
        }
    }

    pub fn update_setting<'a>(&mut self, name: &'a str, value: &'a str) {
        use schema::state;
        let new_state = NewSetting {
            name,
            val: value,
        };

        match &self.conn {
            None => todo!(),
            Some(conn) => {
                let ret = diesel::update(state::table.find(name)).set(&new_state).execute(conn);
                ret.expect("Error updating setting");
            }
        }
    }

    pub fn get_setting (&mut self, name: &str) -> Option<String> {
        use schema::state;
        match &self.conn {
            None => todo!(),

            Some(conn) => {
                let ret = state::table.filter(state::name.eq(name)).load::<self::models::Setting>(conn);
                match ret {
                    Err(e) => {
                        return None;
                    }
                    Ok(vals) => {
                        if vals.len() == 0 {
                            return None;
                        }

                        let setting = vals[0].clone();
                        return Some(String::from(setting.val));
                    }
                }

            }
        }
    }

    pub fn update_state(&mut self, new_state: lg_ac::State) {
        use schema::state;

        match &self.conn {
            None => todo!(),

            Some(conn) => {
                match self.get_setting("on") {
                    None => self.add_setting("on", if new_state.on {"on"} else {"false"}),
                    Some(_) => self.update_setting("on", if new_state.on {"on"} else {"false"}),
                }

                match self.get_setting("mode") {
                    None => self.add_setting("mode", new_state.mode.to_string().as_str()),
                    Some(_) => self.update_setting("mode", new_state.mode.to_string().as_str()),
                }

                match self.get_setting("min_temp") {
                    None => self.add_setting("min_temp", new_state.min_temp.to_string().as_str()),
                    Some(_) => self.update_setting("min_temp", new_state.min_temp.to_string().as_str()),
                }

                match self.get_setting("max_temp") {
                    None => self.add_setting("max_temp", new_state.max_temp.to_string().as_str()),
                    Some(_) => self.update_setting("max_temp", new_state.max_temp.to_string().as_str()),
                }

                match self.get_setting("cur_temp") {
                    None => self.add_setting("cur_temp", new_state.cur_temp.to_string().as_str()),
                    Some(_) => self.update_setting("cur_temp", new_state.cur_temp.to_string().as_str()),
                }

                match self.get_setting("fan_speed") {
                    None => self.add_setting("fan_speed", new_state.fan_speed.to_string().as_str()),
                    Some(_) => self.update_setting("fan_speed", new_state.fan_speed.to_string().as_str()),
                }

                match self.get_setting("fan_mode") {
                    None => self.add_setting("fan_mode", new_state.cur_temp.to_string().as_str()),
                    Some(_) => self.update_setting("fan_mode", new_state.cur_temp.to_string().as_str()),
                }
            }
        }
    }
}