// Tuple

use crate::object_format::{
    ignore_str_split::{ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType},
    property::{data_value_from_string, property_type::PropertyType, Property},
    DataValue,
};

#[derive(Debug)]
pub struct DataValueTuple {
    properties: Vec<Property>,
}

impl DataValueTuple {
    pub fn new(properties: Vec<Property>) -> Self {
        Self { properties }
    }
}

impl DataValue for DataValueTuple {
    fn serialize(
        &self,
        pretty_print: bool,
        tab_index: usize,
    ) -> (
        crate::object_format::property::property_type::PropertyType, /* serialized type */
        String,                                                      /* serialized value */
    ) {
        (
            PropertyType::complex(
                String::from("tuple"),
                self.properties
                    .iter()
                    .map(|property| property.get_value_type())
                    .collect(),
            ),
            format!(
                "({})",
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
        if serialized_type.get_base_type() != "tuple" || !serialized_type.sub_types_included() {
            return None;
        }

        if !(serialized_value.starts_with("(")
            && serialized_value.ends_with(")")
            && serialized_value.chars().count() > 2)
        {
            return None;
        }

        let mut properties: Vec<Property> = Vec::new();

        let mut sub_type_iter = serialized_type.get_sub_types().iter();

        for serialized_property in ignoring_compliant_split_str(
            &serialized_value[1..serialized_value.len() - 1].trim(),
            ',',
            true,
            vec![
                SplitIgnoreRule::new(SplitIgnoreRuleType::PAIR('"')).set_ecapsulates_raw_text(true),
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

        Some(Box::new(Self::new(properties)))
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(Self::new(self.properties.clone()))
    }

    fn as_tuple_structure(&self) -> Vec<Box<dyn DataValue>> {
        self.properties
            .iter()
            .map(|property| property.property_value.clone_data_value())
            .collect()
    }
}
