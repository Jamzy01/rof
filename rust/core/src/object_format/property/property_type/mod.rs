use std::fmt;

use crate::object_format::ignore_str_split::{
    ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PropertyType {
    base_type: String,
    sub_types: Vec<Self>,
}

impl PropertyType {
    pub fn new(base_type: String, sub_types: Vec<Self>) -> Self {
        Self {
            base_type,
            sub_types,
        }
    }

    pub fn struct_type() -> Self {
        // Reserved type for structs

        Self::simple(String::from("struct"))
    }

    pub fn is_struct(&self) -> bool {
        self.base_type == Self::struct_type().base_type && self.sub_types.len() < 1
    }

    pub fn implicit() -> Self {
        // No property type, type is implied by value

        Self::simple(String::new())
    }

    pub fn is_implicit(&self) -> bool {
        self.base_type.is_empty()
            || ["bool", "char", "string", "struct"].contains(&self.base_type.as_ref())
    }

    pub fn simple(base_type: String) -> Self {
        Self {
            base_type,
            sub_types: Vec::new(),
        }
    }

    pub fn complex(base_type: String, sub_types: Vec<Self>) -> Self {
        Self::new(base_type, sub_types)
    }

    pub fn serialize(&self, pretty_print: bool) -> String {
        let mut serialized_sub_types = String::new();

        match self.sub_types.len() > 0 {
            true => {
                let mut sub_type_iterator = self.sub_types.iter();

                let mut sub_type = sub_type_iterator.next().unwrap();

                loop {
                    serialized_sub_types.push_str(&sub_type.serialize(pretty_print));

                    match sub_type_iterator.next() {
                        Some(next_sub_type) => {
                            sub_type = next_sub_type;

                            if pretty_print {
                                serialized_sub_types.push_str(", ");
                            } else {
                                serialized_sub_types.push_str(",");
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }

                format!("{}<{}>", self.base_type, serialized_sub_types)
            }
            false => self.base_type.clone(),
        }
    }

    pub fn deserialize(serialized_string: &str) -> Self {
        let mut base_type: &str = "";

        let mut sub_types: Vec<Self> = Vec::new();

        match serialized_string.split_once("<") {
            Some((deserialized_base_type, serialized_sub_types)) => {
                base_type = deserialized_base_type.trim();

                for serialized_sub_type in ignoring_compliant_split_str(
                    &serialized_sub_types
                        .strip_suffix(|_| true)
                        .unwrap_or(serialized_sub_types),
                    ',',
                    vec![SplitIgnoreRule::new(SplitIgnoreRuleType::NEST('<', '>'))],
                ) {
                    sub_types.push(Self::deserialize(&serialized_sub_type));
                }
            }
            None => {
                base_type = serialized_string.trim();
            }
        }

        Self {
            base_type: base_type.to_string(),
            sub_types,
        }
    }

    pub fn get_base_type(&self) -> &str {
        &self.base_type
    }

    pub fn get_sub_types(&self) -> &Vec<Self> {
        &self.sub_types
    }

    pub fn sub_types_included(&self) -> bool {
        self.sub_types.len() > 0
    }
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.serialize(true))
    }
}
