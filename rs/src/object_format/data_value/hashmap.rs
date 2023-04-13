// Hashmap

use std::collections::HashMap;

use crate::object_format::{
    ignore_str_split::{
        ignoring_compliant_split_once, ignoring_compliant_split_str, SplitIgnoreRule,
        SplitIgnoreRuleType,
    },
    property::{data_value_from_string, property_type::PropertyType},
    DataValue,
};

#[derive(Debug)]
pub struct DataValueHashmap {
    key_type: PropertyType,
    value_type: PropertyType,
    properties: HashMap<Box<dyn DataValue>, Box<dyn DataValue>>,
}

impl DataValueHashmap {
    pub fn new(
        key_type: PropertyType,
        value_type: PropertyType,
        properties: HashMap<Box<dyn DataValue>, Box<dyn DataValue>>,
    ) -> Self {
        Self {
            key_type,
            value_type,
            properties,
        }
    }
}

impl DataValue for DataValueHashmap {
    fn serialize(
        &self,
        pretty_print: bool,
        tab_index: usize,
    ) -> (
        PropertyType, /* serialized type */
        String,       /* serialized value */
    ) {
        (
            PropertyType::complex(
                String::from("hashmap"),
                vec![self.key_type.clone(), self.value_type.clone()],
            ),
            match pretty_print {
                true => format!(
                    "{{\n{}\n{}}}",
                    self.properties
                        .iter()
                        .map(|(property_key, property_value)| format!(
                            "{}{}: {};",
                            "\t".repeat(tab_index + 1),
                            property_key.serialize(pretty_print, tab_index).1,
                            property_value.serialize(pretty_print, tab_index).1
                        ))
                        .collect::<Vec<String>>()
                        .join("\n"),
                    "\t".repeat(tab_index)
                ),
                false => format!(
                    "{{{}}}",
                    self.properties
                        .iter()
                        .map(|(property_key, property_value)| format!(
                            "{}:{};",
                            property_key.serialize(pretty_print, tab_index).1,
                            property_value.serialize(pretty_print, tab_index).1
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
        if !(serialized_type.get_base_type() == "hashmap"
            && serialized_type.get_sub_types().len() == 2)
        {
            return None;
        }

        if !(serialized_value.starts_with("{")
            && serialized_value.ends_with("}")
            && serialized_value.chars().count() > 1)
        {
            return None;
        }

        let mut properties: HashMap<Box<dyn DataValue>, Box<dyn DataValue>> = HashMap::new();

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

            if let Some((serialized_key, serialized_value)) = ignoring_compliant_split_once(
                &serialized_property,
                ':',
                true,
                vec![
                    SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('"'))
                        .set_ecapsulates_raw_text(true),
                    SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('\''))
                        .set_ecapsulates_raw_text(true),
                    SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('{', '}')),
                ],
            ) {
                properties.insert(
                    data_value_from_string(
                        serialized_type.get_sub_types().get(0).unwrap(),
                        &serialized_key.trim(),
                    ),
                    data_value_from_string(
                        serialized_type.get_sub_types().get(1).unwrap(),
                        &serialized_value.trim(),
                    ),
                );
            }
        }

        Some(Box::new(Self::new(
            serialized_type.get_sub_types().get(0).unwrap().clone(),
            serialized_type.get_sub_types().get(1).unwrap().clone(),
            properties,
        )))
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        let mut cloned_properties: HashMap<Box<dyn DataValue>, Box<dyn DataValue>> = HashMap::new();

        for (property_key, property_value) in self.properties.iter() {
            cloned_properties.insert(
                property_key.clone_data_value(),
                property_value.clone_data_value(),
            );
        }

        Box::new(Self::new(
            self.key_type.clone(),
            self.value_type.clone(),
            cloned_properties,
        ))
    }

    fn as_hashmap(&self) -> HashMap<Box<dyn DataValue>, Box<dyn DataValue>> {
        let mut hashmap: HashMap<Box<dyn DataValue>, Box<dyn DataValue>> = HashMap::new();

        for (property_key, property_value) in self.properties.iter() {
            hashmap.insert(
                property_key.clone_data_value(),
                property_value.clone_data_value(),
            );
        }

        hashmap
    }
}
