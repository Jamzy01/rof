// A trait that allows a rust struct to serialized and deserialized

use crate::object_format::{rof::Rof, DataValue};

pub trait RofCompat: Default {
    fn serialize(&self) -> Box<dyn DataValue>;

    fn deserialize(rof_object: Box<dyn DataValue>) -> Self;

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
