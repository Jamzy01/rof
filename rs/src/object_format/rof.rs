use crate::object_format::{DataValue, PropertyType};
use std::fs::OpenOptions;

use std::io::{BufReader, Read, Write};

use super::data_value::bool::DataValueBool;
use super::data_value::enum_value::DataValueEnum;
use super::ignore_str_split::{ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType};
use super::property::data_value_from_string;

/// ## Rof
/// A struct that contains an inner ```DataValue``` trait implementing object which contains utility functions for easy saving and loading that type to files, also is the backbone for the higher level ```RofCompat``` api.
///
/// A Rof ```DataValue``` does not strictly have to be a struct or enum, you can save and load any ```DataValue``` implementing type including
/// * ```DataValueBool```
/// * ```DataValueChararacter```
/// * ```DataValueFloat```
/// * ```DataValueInteger```
/// * ```DataValueString```
/// * ```DataValueTuple``` (although this data value type does exist, the RofCompat API does not support types in it's current state)
/// * ```DataValueArray```
/// * ```DataValueHashmap```
/// * ```DataValueEnum```
/// * ```DataValueStruct```
///
/// You cannot create your own ```DataValue``` types, as when deserializing a Rof from a string, a function named ```data_value_from_string``` checks the string against every ```DataValue``` type
/// until it finds a type that when deserialized doesn't return ```None```. As this function only checks against inbuilt types that it knows about, you cannot create your own
/// as they would not get deserialized.

#[derive(Debug)]
pub struct Rof {
    inner: Box<dyn DataValue>,
}

impl Rof {
    /// Create a ```Rof``` from an inner DataValue implementing type
    pub fn new(inner: Box<dyn DataValue>) -> Self {
        Self { inner }
    }

    /// ## Serialize
    /// Serialize the ```Rof``` to a string
    ///
    /// ## Argmuents
    ///
    /// * `pretty_print` - A bool which when set to true adds unneccesary white space, tabs and newlines to the serialized output that will not change the data, but will make the output string more human-readable if saved to a file.
    pub fn serialize(&self, pretty_print: bool) -> String {
        match self.inner.is_implicit() {
            true => self.inner.serialize(pretty_print, 0).1,
            false => match pretty_print {
                true => format!(
                    "{}: {}",
                    self.inner.serialize(pretty_print, 0).1,
                    self.inner.get_type().serialize(pretty_print)
                ),
                false => format!(
                    "{}:{}",
                    self.inner.serialize(pretty_print, 0).1,
                    self.inner.get_type().serialize(pretty_print)
                ),
            },
        }
    }

    /// ## Deserialize
    /// Deserialize a string to a ```Rof```
    ///
    /// ## Arguments
    ///
    /// * `serialized_rof` - A string slice that represents a ```DataValue``` implementing data type in string form.
    ///
    /// If this function fails to to deserialize the type, it will default to returning ```DataValueEnum::none()```
    pub fn deserialize(serialized_rof: &str) -> Self {
        match &ignoring_compliant_split_str(
            serialized_rof,
            ':',
            true,
            vec![
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('"')).set_ecapsulates_raw_text(true),
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('\''))
                    .set_ecapsulates_raw_text(true),
                SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('{', '}')),
            ],
        )[..]
        {
            [implicit_value] => Self {
                inner: data_value_from_string(&PropertyType::implicit(), implicit_value.trim()),
            },
            [explicit_value, explicit_type] => Self {
                inner: data_value_from_string(
                    &PropertyType::deserialize(&explicit_type.trim()),
                    explicit_value.trim(),
                ),
            },
            _ => Self {
                inner: Box::new(DataValueEnum::none()),
            },
        }
    }

    /// ## Load From File
    /// Loads a ```Rof``` object from file by loading the file as a string and using that on the ```deserialize``` function to convert that to a Rof
    ///
    /// ## Arguments
    ///
    /// * `file_path` - A string slice representing the load file path
    ///
    /// If the function fails in any way, ```DataValueEnum::none()``` will be returned
    pub fn load_from_file(file_path: &str) -> Self {
        match OpenOptions::new().read(true).open(file_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);

                let mut data = String::new();

                reader
                    .read_to_string(&mut data)
                    .expect("Unable to read rof from file");

                Self::deserialize(&data)
            }
            Err(_) => Self::new(Box::new(DataValueEnum::none())),
        }
    }

    /// ## Save To File
    /// Saves the inner ``DataValue``` implementing value of the ```Rof``` to a file using the object on the ```serialize``` function to serialize the object to a string
    ///
    /// Will return an error if the file saving fails
    ///
    /// ## Arguments
    ///
    /// * `file_path` - A string slice representing the save file path
    /// * `pretty_print` - A bool which when set to true adds unneccesary white space, tabs and newlines to the serialized output that will not change the data, but will make the output file more human-readable.
    pub fn save_to_file(&self, file_path: &str, pretty_print: bool) -> Result<(), ()> {
        match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)
        {
            Ok(mut file) => match file.write_all(self.serialize(pretty_print).as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    /// ## Get Object
    /// Returns an **immutable** clone of the inner ```DataValue``` implementing object which this ```Rof``` contains.
    ///
    /// It is important to note that the returned value is immutable
    /// meaning you cannot get the inner value, modify something and run the ```serialize``` or ```save_to_file``` function on the same ```Rof```, as the inner value of the ```Rof``` will not have changed. You will need to create a new ```Rof``` with the newly modified
    /// ```DataValue``` implementing object for the changes to be reflected for the file.

    pub fn get_object(&self) -> Box<dyn DataValue> {
        self.inner.clone_data_value()
    }
}
