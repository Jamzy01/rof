pub mod property_type;

use std::fmt;

use self::property_type::PropertyType;
use super::data_value::{
    array::DataValueArray, bool::DataValueBool, character::DataValueCharacter,
    enum_value::DataValueEnum, float::DataValueFloat, hashmap::DataValueHashmap,
    integer::DataValueInteger, string::DataValueString, struct_value::DataValueStruct,
    tuple::DataValueTuple,
};
use super::DataValue;

pub fn data_value_from_string(
    data_type: &PropertyType,
    serialized_string: &str,
) -> Box<dyn DataValue> {
    if let Some(boolean) = DataValueBool::deserialize(data_type, serialized_string) {
        return boolean;
    }

    if let Some(character) = DataValueCharacter::deserialize(data_type, serialized_string) {
        return character;
    }

    if let Some(integer) = DataValueInteger::deserialize(data_type, serialized_string) {
        return integer;
    }

    if let Some(float) = DataValueFloat::deserialize(data_type, serialized_string) {
        return float;
    }

    if let Some(string) = DataValueString::deserialize(data_type, serialized_string) {
        return string;
    }

    if let Some(struct_value) = DataValueStruct::deserialize(data_type, serialized_string) {
        return struct_value;
    }

    if let Some(tuple_value) = DataValueTuple::deserialize(data_type, serialized_string) {
        return tuple_value;
    }

    if let Some(array_value) = DataValueArray::deserialize(data_type, serialized_string) {
        return array_value;
    }

    if let Some(enum_value) = DataValueEnum::deserialize(data_type, serialized_string) {
        return enum_value;
    }

    if let Some(hashmap_value) = DataValueHashmap::deserialize(data_type, serialized_string) {
        return hashmap_value;
    }

    Box::new(DataValueEnum::none())
}

#[derive(Debug)]
pub struct Property {
    pub property_index: String,
    pub property_value: Box<dyn DataValue>,
}

impl Clone for Property {
    fn clone(&self) -> Self {
        Self::new(
            self.property_index.clone(),
            self.property_value.clone_data_value(),
        )
    }
}

impl Property {
    pub fn new(property_index: String, property_value: Box<dyn DataValue>) -> Self {
        Self {
            property_index,
            property_value,
        }
    }

    pub fn serialize(&self, pretty_print: bool, tab_index: usize) -> String {
        match pretty_print {
            true => match self.get_value_type().is_implicit() {
                true => format!(
                    "{} = {}",
                    self.property_index,
                    self.property_value.serialize(pretty_print, tab_index).1
                ),
                false => format!(
                    "{}: {} = {}",
                    self.property_index,
                    self.get_value_type().serialize(pretty_print),
                    self.property_value.serialize(pretty_print, tab_index).1
                ),
            },
            false => match self.get_value_type().is_implicit() {
                true => format!(
                    "{}={}",
                    self.property_index,
                    self.property_value.serialize(pretty_print, tab_index).1
                ),
                false => format!(
                    "{}:{}={}",
                    self.property_index,
                    self.get_value_type().serialize(pretty_print),
                    self.property_value.serialize(pretty_print, tab_index).1
                ),
            },
        }
    }

    pub fn deserialize(serialized_string: &str) -> Self {
        let mut property_index: &str = "";

        let mut property_value: Box<dyn DataValue> = Box::new(DataValueEnum::none());

        // Get Property Index (Property Name)

        if let Some((raw_data, raw_value)) = serialized_string.split_once("=") {
            if let Some((raw_index, raw_type)) = raw_data.split_once(":") {
                // Explicit type

                property_index = raw_index.trim();

                property_value = data_value_from_string(
                    &PropertyType::deserialize(raw_type.trim()),
                    raw_value.trim(),
                );
            } else {
                // Implicit type (only supported in certain situations)

                property_index = raw_data.trim();

                property_value =
                    data_value_from_string(&PropertyType::implicit(), raw_value.trim());
            }
        }

        Self {
            property_index: property_index.to_string(),
            property_value: property_value,
        }
    }

    pub fn get_value_type(&self) -> PropertyType {
        self.property_value.get_type()
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialize(true, 0))
    }
}
