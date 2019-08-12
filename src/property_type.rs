use diesel::IntoSql;
use crate::property_type::PropertyType::{STRING, INT, NULL};

pub enum PropertyType {
    NULL = 0,
    STRING = 1,
    INT = 2
}

pub fn into_property_type(val: i32) -> PropertyType {
    match val {
        1 => STRING,
        2 => INT,
        _ => NULL
    }
}