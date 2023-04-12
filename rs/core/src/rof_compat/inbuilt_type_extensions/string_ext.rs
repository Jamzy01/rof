use crate::{object_format::data_value::string::DataValueString, rof_compat::RofCompat};

impl RofCompat for String {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueString::new(self.clone()))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_string()
    }
}

// Cannot implement RofCompat for str type because it does not implement Default

/*

impl RofCompat for str {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueString::from(self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_string().as_ref()
    }
}

*/
