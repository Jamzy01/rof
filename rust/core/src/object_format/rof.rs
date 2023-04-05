use crate::object_format::{DataObject, DataValue, PropertyType};
use std::fs::{File, OpenOptions};

use std::io::{BufReader, Read, Seek, Write};

use super::data_value::bool::DataValueBool;
use super::data_value::enum_value::DataValueEnum;
use super::ignore_str_split::{ignoring_compliant_split_str, SplitIgnoreRule, SplitIgnoreRuleType};
use super::property::data_value_from_string;

#[derive(Debug)]
pub struct Rof {
    inner: Box<dyn DataValue>,
}

impl Rof {
    pub fn new(inner: Box<dyn DataValue>) -> Self {
        Self { inner }
    }

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
            Err(_) => Self::new(Box::new(DataValueBool::new(false))),
        }
    }

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

    pub fn get_object(&self) -> Box<dyn DataValue> {
        self.inner.clone_data_value()
    }
}
