// Enum

use crate::object_format::{
    ignore_str_split::{ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType},
    property::{data_value_from_string, property_type::PropertyType, Property},
    DataValue,
};

#[derive(Debug)]
pub struct DataValueEnum {
    enum_name: String,
    enum_values: Vec<Property>,
}

impl DataValueEnum {
    pub fn new(enum_name: String, enum_values: Vec<Property>) -> Self {
        Self {
            enum_name,
            enum_values,
        }
    }

    pub fn simple(enum_name: String) -> Self {
        Self::new(enum_name, Vec::new())
    }

    pub fn none() -> Self {
        Self::new(String::from("None"), Vec::new())
    }
}

impl DataValue for DataValueEnum {
    fn serialize(
        &self,
        pretty_print: bool,
        tab_index: usize,
    ) -> (
        PropertyType, /* serialized type */
        String,       /* serialized value */
    ) {
        (
            PropertyType::new(
                String::from("enum"),
                self.enum_values
                    .iter()
                    .map(|enum_value| enum_value.property_value.get_type())
                    .collect(),
            ),
            match self.enum_values.is_empty() {
                true => format!("{}", self.enum_name),
                false => format!(
                    "{}({})",
                    self.enum_name,
                    self.enum_values
                        .iter()
                        .map(|property| property
                            .property_value
                            .serialize(pretty_print, tab_index)
                            .1)
                        .collect::<Vec<String>>()
                        .join(match pretty_print {
                            true => ", ",
                            false => ",",
                        })
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
        if !(serialized_type.get_base_type() == "enum"
            || serialized_type.get_base_type() == "option")
        {
            return None;
        }

        match serialized_value.split_once("(") {
            Some((enum_name, enum_data)) => {
                let mut properties: Vec<Property> = Vec::new();

                let mut sub_type_iter = serialized_type.get_sub_types().iter();

                for serialized_property in ignoring_compliant_split_str(
                    &enum_data[..enum_data.len() - 1].trim(),
                    ',',
                    vec![
                        SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('"'))
                            .set_ecapsulates_raw_text(true),
                        SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('\''))
                            .set_ecapsulates_raw_text(true),
                        SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('(', ')')),
                        SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('<', '>')),
                    ],
                ) {
                    if serialized_property.trim().len() < 1 {
                        continue;
                    }

                    if let Some(sub_type) = sub_type_iter.next() {
                        properties.push(Property::unnamed(data_value_from_string(
                            sub_type,
                            serialized_property.trim(),
                        )));
                    } else {
                        break;
                    }
                }

                Some(Box::new(Self::new(enum_name.to_string(), properties)))
            }
            None => Some(Box::new(Self::new(
                serialized_value.to_string(),
                Vec::new(),
            ))),
        }
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(self.enum_name.clone(), self.enum_values.clone()))
    }

    fn as_enum_structure(
        &self,
    ) -> (
        String,                  /* enum name */
        Vec<Box<dyn DataValue>>, /* enum values */
    ) {
        (
            self.enum_name.clone(),
            self.enum_values
                .iter()
                .map(|enum_value| enum_value.property_value.clone_data_value())
                .collect(),
        )
    }

    fn as_bool(&self) -> bool {
        self.enum_name == "Some"
    }

    fn as_string(&self) -> String {
        self.enum_name.clone()
    }
}
