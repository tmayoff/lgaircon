use diesel::prelude::*;
use diesel::SqliteConnection;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::embed_migrations;

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
        let connection = SqliteConnection::establish("app.sqlite").expect("Failed to create/connect to DB");

        DB {
            connection
        }
    }

    pub fn run_migrations(&mut self) {
        self.connection.run_pending_migrations(MIGRATIONS).unwrap();
    }

    pub fn add_setting<'a>(&mut self, name: &'a str, value: &'a str) {
        use schema::state;
        let new_state = NewSetting {
            name,
            val: value,
        };

        let ret = diesel::insert_into(state::table)
        .values(&new_state)
        .execute(&mut self.connection);
        ret.expect("Error adding new setting");
    }

    pub fn update_setting<'a>(&mut self, name: &'a str, value: &'a str) {
        use schema::state;
        let new_state = NewSetting {
            name,
            val: value,
        };

        let ret = diesel::update(state::table.find(name)).set(&new_state).execute(&mut self.connection);
        ret.expect("Error updating setting");
    }

    pub fn get_setting (&mut self, name: &str) -> Option<String> {
        use schema::state;

        let ret = state::table.filter(state::name.eq(name)).load::<self::models::Setting>(&mut self.connection);
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

    pub fn new_temp(&mut self, temp: f64) {
        use schema::temperature;
        let new_temp = NewTemperature {
            value: temp
        };

        let ret = diesel::insert_into(temperature::table)
        .values(new_temp)
        .execute(&mut self.connection);
        ret.expect("Error adding temperature");
    }

    pub fn update_state(&mut self, new_state: lg_ac::State) {
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

        match self.get_setting("target_temp") {
            None => self.add_setting("target_temp", new_state.target_temp.to_string().as_str()),
            Some(_) => self.update_setting("target_temp", new_state.target_temp.to_string().as_str()),
        }

        match self.get_setting("fan_speed") {
            None => self.add_setting("fan_speed", new_state.fan_speed.to_string().as_str()),
            Some(_) => self.update_setting("fan_speed", new_state.fan_speed.to_string().as_str()),
        }

        match self.get_setting("fan_mode") {
            None => self.add_setting("fan_mode", new_state.target_temp.to_string().as_str()),
            Some(_) => self.update_setting("fan_mode", new_state.target_temp.to_string().as_str()),
        }
    }
}