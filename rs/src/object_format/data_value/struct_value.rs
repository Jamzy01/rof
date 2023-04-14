use std::collections::HashMap;

use crate::object_format::{
    ignore_str_split::{ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType},
    property::{property_type::PropertyType, Property},
    DataValue,
};

// Struct

#[derive(Debug, Clone)]
pub struct DataValueStruct {
    properties: Vec<Property>,
}

impl DataValueStruct {
    pub fn new(properties: Vec<Property>) -> Self {
        Self { properties }
    }
}

impl DataValue for DataValueStruct {
    fn serialize(
        &self,
        pretty_print: bool,
        tab_index: usize,
    ) -> (
        PropertyType, /* serialized type */
        String,       /* serialized value */
    ) {
        (
            PropertyType::struct_type(),
            match pretty_print {
                true => format!(
                    "{{\n{}\n{}}}",
                    self.properties
                        .iter()
                        .map(|property| format!(
                            "{}{};",
                            "\t".repeat(tab_index + 1),
                            property.serialize(pretty_print, tab_index + 1)
                        ))
                        .collect::<Vec<String>>()
                        .join("\n"),
                    "\t".repeat(tab_index)
                ),
                false => format!(
                    "{{{}}}",
                    self.properties
                        .iter()
                        .map(|property| format!(
                            "{};",
                            property.serialize(pretty_print, tab_index + 1)
                        ))
                        .collect::<Vec<String>>()
                        .join("")
                ),
            },
        )
    }

    fn deserialize(
        serialized_type: &PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.get_base_type() != "" && !serialized_type.is_struct() {
            return None;
        }

        if !(serialized_value.starts_with("{")
            && serialized_value.ends_with("}")
            && serialized_value.chars().count() > 1)
        {
            return None;
        }

        let mut properties: Vec<Property> = Vec::new();

        for serialized_property in ignoring_compliant_split_str(
            &serialized_value[1..serialized_value.len() - 1].trim(),
            ';',
            true,
            vec![
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('"')).set_ecapsulates_raw_text(true),
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('\''))
                    .set_ecapsulates_raw_text(true),
                SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('{', '}')),
            ],
        ) {
            if serialized_property.trim().len() < 1 {
                continue;
            }

            properties.push(Property::deserialize(serialized_property.trim()));
        }

        Some(Box::new(Self::new(properties)))
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(self.properties.clone()))
    }

    fn as_struct_structure(&self) -> HashMap<String, Box<dyn DataValue>> {
        let mut hashmap: HashMap<String, Box<dyn DataValue>> = HashMap::new();

        for property in self.properties.iter() {
            hashmap.insert(
                property.property_index.clone(),
                property.property_value.clone_data_value(),
            );
        }

        hashmap
    }
}
