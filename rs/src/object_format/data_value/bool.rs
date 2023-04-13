use crate::object_format::{property::property_type::PropertyType, DataValue};

// Bool

#[derive(Debug)]
pub struct DataValueBool {
    inner: bool,
}

impl DataValueBool {
    pub fn new(inner_value: bool) -> Self {
        Self { inner: inner_value }
    }
}

impl DataValue for DataValueBool {
    fn serialize(&self, _: bool, _: usize) -> (PropertyType, String) {
        (
            PropertyType::simple(String::from("bool")),
            match self.inner {
                true => String::from("true"),
                false => String::from("false"),
            },
        )
    }

    fn deserialize(
        serialized_type: &crate::object_format::property::property_type::PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.get_base_type() != "" && serialized_type.get_base_type() != "bool"
            || serialized_type.sub_types_included()
        {
            return None;
        }

        match serialized_value {
            "true" => Some(Box::new(Self::new(true))),
            "false" => Some(Box::new(Self::new(false))),
            _ => None,
        }
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(self.inner))
    }

    fn as_bool(&self) -> bool {
        self.inner
    }
}
