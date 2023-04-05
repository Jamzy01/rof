use crate::object_format::{property::property_type::PropertyType, string_escaper::escape_string};

use super::super::DataValue;

// String

#[derive(Debug)]
pub struct DataValueString {
    inner: String,
}

impl DataValueString {
    pub fn new(inner_value: String) -> Self {
        Self { inner: inner_value }
    }
}

impl DataValue for DataValueString {
    fn serialize(
        &self,
        _: bool,
        _: usize,
    ) -> (
        PropertyType, /* serialized type */
        String,       /* serialized value */
    ) {
        (
            PropertyType::simple(String::from("string")),
            format!("\"{}\"", escape_string(&self.inner, &['"'])),
        )
    }

    fn deserialize(
        serialized_type: &crate::object_format::property::property_type::PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.get_base_type() != "" && serialized_type.get_base_type() != "string"
            || serialized_type.sub_types_included()
        {
            return None;
        }

        if serialized_value.starts_with("\"")
            && serialized_value.ends_with("\"")
            && serialized_value.chars().count() > 1
        {
            return Some(Box::new(Self::new(
                serialized_value
                    .strip_prefix(|_| true)
                    .unwrap_or(serialized_value)
                    .strip_suffix(|_| true)
                    .unwrap_or(serialized_value)
                    .to_string()
                    .replace("\\", ""),
            )));
        }

        None
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(self.inner.clone()))
    }

    fn as_string(&self) -> String {
        self.inner.clone()
    }
}
