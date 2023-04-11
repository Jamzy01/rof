use crate::{object_format::data_value::string::DataValueString, rof_compat::RofCompat};

impl RofCompat for String {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueString::new(self.clone()))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_string()
    }
}
