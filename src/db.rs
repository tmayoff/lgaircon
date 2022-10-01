use std::str::FromStr;

use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;

use crate::db::models::NewSetting;
use crate::db::models::NewTemperature;
use crate::lg_ac;

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct DB {
    connection: SqliteConnection,
}

impl DB {
    pub fn new() -> Self {
        let home_dir = std::env::var_os("HOME");
        let dir_prefix: String;
        match home_dir {
            Some(dir) => dir_prefix = dir.into_string().expect("Directory failed to be converted"),
            None => dir_prefix = String::from("/root"),
        }

        let db_path = dir_prefix + "/app.sqlite";
        println!("Using DB file {}", db_path);

        let connection =
            SqliteConnection::establish(db_path.as_str()).expect("Failed to create/connect to DB");

        DB { connection }
    }

    pub fn run_migrations(&mut self) {
        self.connection.run_pending_migrations(MIGRATIONS).unwrap();
    }

    pub fn add_setting<'a>(&mut self, name: String, value: String) {
        use schema::state;
        let new_state = NewSetting {
            name: name.as_str(),
            val: value.as_str(),
        };

        let ret = diesel::insert_into(state::table)
            .values(&new_state)
            .execute(&mut self.connection);
        ret.expect("Error adding new setting");
    }

    pub fn update_setting<'a>(&mut self, name: String, value: String) {
        use schema::state;
        let new_state = NewSetting {
            name: name.as_str(),
            val: value.as_str(),
        };

        let ret = diesel::update(state::table.find(&name))
            .set(&new_state)
            .execute(&mut self.connection);
        ret.expect("Error updating setting");
    }

    pub fn get_setting(&mut self, name: &str) -> Option<String> {
        use schema::state;

        let ret = state::table
            .filter(state::name.eq(name))
            .load::<self::models::Setting>(&mut self.connection);
        match ret {
            Err(_) => {
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

    pub fn get_state(&mut self) -> lg_ac::State {
        let mut s = lg_ac::State::default();

        if let Some(m) = self.get_setting("mode") {
            s.mode = lg_ac::Mode::from_str(m.as_str()).expect("Mode unreadable from db");
        }

        if let Some(m) = self.get_setting("min_temp") {
            s.min_temp = m.parse().expect("Expected a number");
        }

        if let Some(m) = self.get_setting("max_temp") {
            s.max_temp = m.parse().expect("Expected a number");
        }

        if let Some(m) = self.get_setting("target_temp") {
            s.target_temp = m.parse().expect("Expected a number");
        }

        if let Some(m) = self.get_setting("fan_speed") {
            s.fan_speed = m.parse().expect("Expected a number");
        }

        if let Some(m) = self.get_setting("fan_mode") {
            s.fan_mode = lg_ac::FanMode::from_str(m.as_str()).expect("Fan mode unreadable from db");
        }

        s
    }

    pub fn new_temp(&mut self, temp: f64) {
        use schema::temperature;
        let new_temp = NewTemperature { value: temp };

        let ret = diesel::insert_into(temperature::table)
            .values(new_temp)
            .execute(&mut self.connection);
        ret.expect("Error adding temperature");
    }

    pub fn update_state(&mut self, new_state: lg_ac::State) {
        match self.get_setting("mode") {
            None => self.add_setting(String::from("mode"), new_state.mode.to_string()),
            Some(_) => self.update_setting(String::from("mode"), new_state.mode.to_string()),
        }

        match self.get_setting("min_temp") {
            None => self.add_setting(String::from("min_temp"), new_state.min_temp.to_string()),
            Some(_) => {
                self.update_setting(String::from("min_temp"), new_state.min_temp.to_string())
            }
        }

        match self.get_setting("max_temp") {
            None => self.add_setting(String::from("max_temp"), new_state.max_temp.to_string()),
            Some(_) => {
                self.update_setting(String::from("max_temp"), new_state.max_temp.to_string())
            }
        }

        match self.get_setting("target_temp") {
            None => self.add_setting(
                String::from("target_temp"),
                new_state.target_temp.to_string(),
            ),
            Some(_) => self.update_setting(
                String::from("target_temp"),
                new_state.target_temp.to_string(),
            ),
        }

        match self.get_setting("fan_speed") {
            None => self.add_setting(String::from("fan_speed"), new_state.fan_speed.to_string()),
            Some(_) => {
                self.update_setting(String::from("fan_speed"), new_state.fan_speed.to_string())
            }
        }

        match self.get_setting("fan_mode") {
            None => self.add_setting(String::from("fan_mode"), new_state.target_temp.to_string()),
            Some(_) => {
                self.update_setting(String::from("fan_mode"), new_state.target_temp.to_string())
            }
        }
    }
}
