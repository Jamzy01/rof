// A trait that allows a rust struct to serialized and deserialized

use std::hash::Hash;

use crate::object_format::{rof::Rof, DataValue};

pub trait RofCompat: Default {
    fn serialize(&self) -> Box<dyn DataValue>;

    fn serialize_to_string(&self, pretty_print: bool) -> String {
        Rof::new(self.serialize()).serialize(pretty_print)
    }

    fn deserialize(rof_object: Box<dyn DataValue>) -> Self;

    fn deserialize_from_string(serialized_rof: &str) -> Self {
        Self::deserialize(Rof::deserialize(serialized_rof).get_object())
    }

    fn as_rof(&self) -> Rof {
        Rof::new(self.serialize())
    }

    fn load_from_file(file_path: &str) -> Self {
        Self::deserialize(Rof::load_from_file(file_path).get_object())
    }

    fn save_to_file(&self, file_path: &str, pretty_print: bool) -> Result<(), ()> {
        Rof::new(self.serialize()).save_to_file(file_path, pretty_print)
    }
}

pub mod inbuilt_type_extensions;
