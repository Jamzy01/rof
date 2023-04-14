// A trait that allows a rust struct to serialized and deserialized

use crate::object_format::{rof::Rof, DataValue};

/// ## Rof Compatible
/// A trait that enables a high level abstraction from the low level system that allows serializing and deserializing any ```RofCompat``` implementing type to string.
/// Using the ```RofCompat``` derive macro, you can automatically implement the ```RofCompat``` trait on any struct or enum in most situations (refer to docs for more information).
///
/// In order to manually implement ```RofCompat``` for any type, the `serialize` and `deserialize` methods only have to be implemented, as the other functions are only utility functions based on those two simple methods.
pub trait RofCompat: Default {
    /// ## Serialize
    ///
    /// Serialize ```RofCompat``` implementing object to a ```DataValue``` implementing object.
    ///
    /// ## Example
    ///
    /// ```
    /// fn serialize(&self) -> Box<dyn DataValue> {
    ///     Box::new(DataValueInteger::U32(
    ///         65536 * (self.r as u32) + 256 * (self.g as u32) + (self.b as u32),
    ///     ))
    /// }
    /// ```
    fn serialize(&self) -> Box<dyn DataValue>;

    fn serialize_to_string(&self, pretty_print: bool) -> String {
        Rof::new(self.serialize()).serialize(pretty_print)
    }

    /// ## Deserialize
    ///
    /// Deserialize a ```DataValue``` implementing (not string) to itself.
    ///
    /// ## Example
    ///
    /// ```
    /// fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
    ///     let color_int: u32 = rof_object.as_u32();
    ///
    ///     Self {
    ///         r: (color_int % 16_777_216 / 65_536) as u8,
    ///         g: (color_int % 65_536 / 256) as u8,
    ///         b: (color_int % 256) as u8,
    ///     }
    /// }
    /// ```
    fn deserialize(rof_object: Box<dyn DataValue>) -> Self;

    fn deserialize_from_string(serialized_rof: &str) -> Self {
        Self::deserialize(Rof::deserialize(serialized_rof).get_object())
    }

    /// ## As Rof
    ///
    /// Returns a new Rof created from itself's serialized value
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
