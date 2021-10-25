use crate::property_type::PropertyType::{Int, Null, Str};

pub enum PropertyType {
    Null = 0,
    Str = 1,
    Int = 2,
}

pub fn into_property_type(val: i32) -> PropertyType {
    match val {
        1 => Str,
        2 => Int,
        _ => Null,
    }
}
