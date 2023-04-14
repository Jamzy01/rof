// Array

use crate::object_format::{
    ignore_str_split::{ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType},
    property::{data_value_from_string, property_type::PropertyType, Property},
    DataValue,
};

#[derive(Debug)]
pub struct DataValueArray {
    property_type: PropertyType,
    properties: Vec<Property>,
}

impl DataValueArray {
    pub fn new(property_type: PropertyType, properties: Vec<Property>) -> Self {
        Self {
            property_type,
            properties,
        }
    }
}

impl DataValue for DataValueArray {
    fn serialize(
        &self,
        pretty_print: bool,
        tab_index: usize,
    ) -> (
        crate::object_format::property::property_type::PropertyType, /* serialized type */
        String,                                                      /* serialized value */
    ) {
        (
            PropertyType::complex(String::from("array"), vec![self.property_type.clone()]),
            format!(
                "[{}]",
                self.properties
                    .iter()
                    .map(|property| property.property_value.serialize(pretty_print, tab_index).1)
                    .collect::<Vec<String>>()
                    .join(match pretty_print {
                        true => ", ",
                        false => ",",
                    })
            ),
        )
    }

    fn deserialize(
        serialized_type: &PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.get_base_type() != "array" {
            return None;
        }

        if !(serialized_value.starts_with("[") && serialized_value.ends_with("]")) {
            return None;
        }

        let property_type = serialized_type.get_sub_types().get(0).unwrap().clone();

        let mut properties: Vec<Property> = Vec::new();

        for serialized_property in ignoring_compliant_split_str(
            &serialized_value[1..serialized_value.len() - 1].trim(),
            ',',
            true,
            vec![
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('"')).set_ecapsulates_raw_text(true),
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('\''))
                    .set_ecapsulates_raw_text(true),
                SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('[', ']')),
                SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('<', '>')),
            ],
        ) {
            if serialized_property.trim().len() < 1 {
                continue;
            }

            properties.push(Property::unnamed(data_value_from_string(
                &property_type,
                serialized_property.trim(),
            )));
        }

        Some(Box::new(Self::new(property_type, properties)))
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(
            self.property_type.clone(),
            self.properties.clone(),
        ))
    }

    // Conversions

    fn as_vec(&self) -> Vec<Box<dyn DataValue>> {
        self.properties
            .iter()
            .map(|property| property.property_value.clone_data_value())
            .collect()
    }
}
