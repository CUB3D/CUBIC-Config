use super::schema::Projects as ProjectsTable;
use super::schema::Layers as LayersTable;

#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub projectUUID: String,
    pub owner: u32
}

#[derive(Insertable)]
#[table_name="ProjectsTable"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub projectUUID: &'a str,
    pub owner: &'a u32
}

#[derive(Insertable)]
#[table_name="LayersTable"]
pub struct NewLayer<'a> {
    pub name: &'a str,
    pub _condition: &'a str,
    pub projectID: i32
}

#[derive(Queryable)]
pub struct Layers {
    pub name: String,
    pub _condition: String,
    pub projectID: i32
}

#[derive(Queryable)]
pub struct Property {
    pub id: i32,
    pub name: String,
    pub type_: i32,
    pub value: String,
    pub layerID: i32
}
