use std::{
    collections::HashMap,
    fmt::Debug,
    hash::{Hash, Hasher},
};
pub mod data_value;

pub mod property;

use self::property::{property_type::PropertyType, Property};

pub trait DataValue: Debug {
    // Serliazing and Deserializing

    fn serialize(
        &self,
        pretty_print: bool,
        tab_index: usize,
    ) -> (
        PropertyType, /* serialized type */
        String,       /* serialized value */
    );

    fn deserialize(
        serialized_type: &PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized;

    // Types

    fn get_type(&self) -> PropertyType {
        self.serialize(false, 0).0
    }

    fn is_implicit(&self) -> bool {
        self.get_type().is_implicit()
    }

    // Cloning

    fn clone_data_value(&self) -> Box<dyn DataValue>;

    // Conversions

    fn as_vec(&self) -> Vec<Box<dyn DataValue>> {
        self.as_tuple_structure()
    }

    fn as_bool(&self) -> bool {
        self.as_vec().len() > 0 || self.as_usize() != 0
    }

    fn as_character(&self) -> char {
        'a'
    }

    fn as_enum_structure(
        &self,
    ) -> (
        String,                  /* enum name */
        Vec<Box<dyn DataValue>>, /* enum values */
    ) {
        (String::new(), Vec::new())
    }

    fn as_hashmap(&self) -> HashMap<Box<dyn DataValue>, Box<dyn DataValue>> {
        HashMap::new()
    }

    fn as_string(&self) -> String {
        String::new()
    }

    fn as_struct_structure(&self) -> HashMap<String, Box<dyn DataValue>> {
        HashMap::new()
    }

    fn as_tuple_structure(&self) -> Vec<Box<dyn DataValue>> {
        Vec::new()
    }

    // Floats

    fn as_f32(&self) -> f32 {
        0.
    }

    fn as_f64(&self) -> f64 {
        0.
    }

    // Integers

    fn as_u8(&self) -> u8 {
        0
    }

    fn as_i8(&self) -> i8 {
        0
    }

    fn as_u16(&self) -> u16 {
        0
    }

    fn as_i16(&self) -> i16 {
        0
    }

    fn as_u32(&self) -> u32 {
        0
    }

    fn as_i32(&self) -> i32 {
        0
    }

    fn as_u64(&self) -> u64 {
        0
    }

    fn as_i64(&self) -> i64 {
        0
    }

    fn as_u128(&self) -> u128 {
        0
    }

    fn as_i128(&self) -> i128 {
        0
    }

    fn as_usize(&self) -> usize {
        0
    }

    fn as_isize(&self) -> isize {
        0
    }
}

impl PartialEq for dyn DataValue {
    fn eq(&self, other: &Self) -> bool {
        self.serialize(false, 0) == other.serialize(false, 0)
    }
}

impl Eq for dyn DataValue {}

impl Hash for dyn DataValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.serialize(false, 0).1.hash(state)
    }
}

pub struct DataObject {
    properties: Vec<Property>,
}

impl DataObject {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
        }
    }
}

pub mod ignore_str_split;
pub mod rof;
pub mod string_escaper;
