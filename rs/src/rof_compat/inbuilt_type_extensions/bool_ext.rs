use crate::{object_format::data_value::bool::DataValueBool, rof_compat::RofCompat};

impl RofCompat for bool {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueBool::new(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_bool()
    }
}
