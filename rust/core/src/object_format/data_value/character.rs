use crate::object_format::{property::property_type::PropertyType, string_escaper::escape_string};

use super::super::DataValue;

// Character

#[derive(Debug)]
pub struct DataValueCharacter {
    inner: char,
}

impl DataValueCharacter {
    pub fn new(inner_value: char) -> Self {
        Self { inner: inner_value }
    }
}

impl DataValue for DataValueCharacter {
    fn serialize(&self, _: bool, _: usize) -> (PropertyType, String) {
        (
            PropertyType::simple(String::from("char")),
            format!("\'{}\'", escape_string(&format!("{}", self.inner), &['\''])),
        )
    }

    fn deserialize(
        serialized_type: &crate::object_format::property::property_type::PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.get_base_type() != "" && serialized_type.get_base_type() != "char"
            || serialized_type.sub_types_included()
        {
            return None;
        }

        if serialized_value.starts_with("'")
            && serialized_value.ends_with("'")
            && serialized_value.chars().count() == 3
        {
            return Some(Box::new(Self::new(
                serialized_value.chars().skip(1).next().unwrap(),
            )));
        }

        None
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(self.inner))
    }

    fn as_character(&self) -> char {
        self.inner
    }

    fn as_string(&self) -> String {
        String::from(self.inner)
    }
}
