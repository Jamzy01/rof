use crate::{object_format::data_value::float::DataValueFloat, rof_compat::RofCompat};

impl RofCompat for f32 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueFloat::F32(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_f32()
    }
}

impl RofCompat for f64 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueFloat::F64(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_f64()
    }
}
