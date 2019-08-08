use super::schema::Projects as ProjectsTable;

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub projectUUID: String
}

#[derive(Insertable)]
#[table_name="ProjectsTable"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub projectUUID: &'a str
}

pub struct Layers {
    pub id: i32,
    pub name: String,
    pub _condition: String,
    pub projectID: i32
}

pub struct Property {
    pub id: i32,
    pub name: String,
    pub type_: i32,
    pub value: String,
    pub layerID: i32
}