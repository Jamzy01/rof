use crate::{object_format::data_value::character::DataValueCharacter, rof_compat::RofCompat};

impl RofCompat for char {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueCharacter::new(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_character()
    }
}
