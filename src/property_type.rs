use crate::property_type::PropertyType::{INT, NULL, STRING};

pub enum PropertyType {
    NULL = 0,
    STRING = 1,
    INT = 2,
}

pub fn into_property_type(val: i32) -> PropertyType {
    match val {
        1 => STRING,
        2 => INT,
        _ => NULL,
    }
}
