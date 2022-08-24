use super::schema::_state;

#[derive(Clone, Queryable)]
pub struct Setting {
    pub name: String,
    pub val: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name="_state"]
pub struct NewSetting <'a> {
    pub name: &'a str,
    pub val: &'a str,
}