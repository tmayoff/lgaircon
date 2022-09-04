use super::schema::{state, temperature};

#[derive(Clone, Queryable)]
pub struct Setting {
    pub name: String,
    pub val: String,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = state)]
pub struct NewSetting <'a> {
    pub name: &'a str,
    pub val: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = temperature)]
pub struct NewTemperature {
    pub value: f64,
}