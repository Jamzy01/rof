use crate::object_format::property::property_type::PropertyType;

use super::super::DataValue;

// Integer

#[derive(Clone, Debug)]
pub enum DataValueInteger {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    USIZE(usize),
    ISIZE(isize),
}

impl DataValue for DataValueInteger {
    fn serialize(&self, _: bool, _: usize) -> (PropertyType, String) {
        match self {
            Self::U8(inner) => (PropertyType::simple(String::from("u8")), inner.to_string()),
            Self::I8(inner) => (PropertyType::simple(String::from("i8")), inner.to_string()),

            Self::U16(inner) => (PropertyType::simple(String::from("u16")), inner.to_string()),
            Self::I16(inner) => (PropertyType::simple(String::from("i16")), inner.to_string()),

            Self::U32(inner) => (PropertyType::simple(String::from("u32")), inner.to_string()),
            Self::I32(inner) => (PropertyType::simple(String::from("i32")), inner.to_string()),

            Self::U64(inner) => (PropertyType::simple(String::from("u64")), inner.to_string()),
            Self::I64(inner) => (PropertyType::simple(String::from("i64")), inner.to_string()),

            Self::U128(inner) => (
                PropertyType::simple(String::from("u128")),
                inner.to_string(),
            ),
            Self::I128(inner) => (
                PropertyType::simple(String::from("i128")),
                inner.to_string(),
            ),

            Self::USIZE(inner) => (
                PropertyType::simple(String::from("usize")),
                inner.to_string(),
            ),
            Self::ISIZE(inner) => (
                PropertyType::simple(String::from("isize")),
                inner.to_string(),
            ),
        }
    }

    fn deserialize(
        serialized_type: &crate::object_format::property::property_type::PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.sub_types_included() {
            return None;
        }

        match serialized_type.get_base_type() {
            "u8" => {
                if let Ok(parsed_int) = serialized_value.parse::<u8>() {
                    return Some(Box::new(Self::U8(parsed_int)));
                } else {
                    return None;
                }
            }
            "i8" => {
                if let Ok(parsed_int) = serialized_value.parse::<i8>() {
                    return Some(Box::new(Self::I8(parsed_int)));
                } else {
                    return None;
                }
            }

            "u16" => {
                if let Ok(parsed_int) = serialized_value.parse::<u16>() {
                    return Some(Box::new(Self::U16(parsed_int)));
                } else {
                    return None;
                }
            }
            "i16" => {
                if let Ok(parsed_int) = serialized_value.parse::<i16>() {
                    return Some(Box::new(Self::I16(parsed_int)));
                } else {
                    return None;
                }
            }

            "u32" => {
                if let Ok(parsed_int) = serialized_value.parse::<u32>() {
                    return Some(Box::new(Self::U32(parsed_int)));
                } else {
                    return None;
                }
            }
            "i32" => {
                if let Ok(parsed_int) = serialized_value.parse::<i32>() {
                    return Some(Box::new(Self::I32(parsed_int)));
                } else {
                    return None;
                }
            }

            "u64" => {
                if let Ok(parsed_int) = serialized_value.parse::<u64>() {
                    return Some(Box::new(Self::U64(parsed_int)));
                } else {
                    return None;
                }
            }
            "i64" => {
                if let Ok(parsed_int) = serialized_value.parse::<i64>() {
                    return Some(Box::new(Self::I64(parsed_int)));
                } else {
                    return None;
                }
            }

            "u128" => {
                if let Ok(parsed_int) = serialized_value.parse::<u128>() {
                    return Some(Box::new(Self::U128(parsed_int)));
                } else {
                    return None;
                }
            }
            "i128" => {
                if let Ok(parsed_int) = serialized_value.parse::<i128>() {
                    return Some(Box::new(Self::I128(parsed_int)));
                } else {
                    return None;
                }
            }

            "usize" => {
                if let Ok(parsed_int) = serialized_value.parse::<usize>() {
                    return Some(Box::new(Self::USIZE(parsed_int)));
                } else {
                    return None;
                }
            }
            "isize" => {
                if let Ok(parsed_int) = serialized_value.parse::<isize>() {
                    return Some(Box::new(Self::ISIZE(parsed_int)));
                } else {
                    return None;
                }
            }

            _ => None,
        }
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(match self {
            Self::U8(value) => Self::U8(value.clone()),
            Self::I8(value) => Self::I8(value.clone()),

            Self::U16(value) => Self::U16(value.clone()),
            Self::I16(value) => Self::I16(value.clone()),

            Self::U32(value) => Self::U32(value.clone()),
            Self::I32(value) => Self::I32(value.clone()),

            Self::U64(value) => Self::U64(value.clone()),
            Self::I64(value) => Self::I64(value.clone()),

            Self::U128(value) => Self::U128(value.clone()),
            Self::I128(value) => Self::I128(value.clone()),

            Self::USIZE(value) => Self::USIZE(value.clone()),
            Self::ISIZE(value) => Self::ISIZE(value.clone()),
        })
    }

    // TODO: Number conversions

    fn as_f32(&self) -> f32 {
        match self {
            Self::U8(inner) => *inner as f32,
            Self::I8(inner) => *inner as f32,
            Self::U16(inner) => *inner as f32,
            Self::I16(inner) => *inner as f32,
            Self::U32(inner) => *inner as f32,
            Self::I32(inner) => *inner as f32,
            Self::U64(inner) => *inner as f32,
            Self::I64(inner) => *inner as f32,
            Self::U128(inner) => *inner as f32,
            Self::I128(inner) => *inner as f32,
            Self::USIZE(inner) => *inner as f32,
            Self::ISIZE(inner) => *inner as f32,
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Self::U8(inner) => *inner as f64,
            Self::I8(inner) => *inner as f64,
            Self::U16(inner) => *inner as f64,
            Self::I16(inner) => *inner as f64,
            Self::U32(inner) => *inner as f64,
            Self::I32(inner) => *inner as f64,
            Self::U64(inner) => *inner as f64,
            Self::I64(inner) => *inner as f64,
            Self::U128(inner) => *inner as f64,
            Self::I128(inner) => *inner as f64,
            Self::USIZE(inner) => *inner as f64,
            Self::ISIZE(inner) => *inner as f64,
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            Self::U8(inner) => *inner,
            Self::I8(inner) => *inner as u8,
            Self::U16(inner) => *inner as u8,
            Self::I16(inner) => *inner as u8,
            Self::U32(inner) => *inner as u8,
            Self::I32(inner) => *inner as u8,
            Self::U64(inner) => *inner as u8,
            Self::I64(inner) => *inner as u8,
            Self::U128(inner) => *inner as u8,
            Self::I128(inner) => *inner as u8,
            Self::USIZE(inner) => *inner as u8,
            Self::ISIZE(inner) => *inner as u8,
        }
    }

    fn as_i8(&self) -> i8 {
        match self {
            Self::U8(inner) => *inner as i8,
            Self::I8(inner) => *inner,
            Self::U16(inner) => *inner as i8,
            Self::I16(inner) => *inner as i8,
            Self::U32(inner) => *inner as i8,
            Self::I32(inner) => *inner as i8,
            Self::U64(inner) => *inner as i8,
            Self::I64(inner) => *inner as i8,
            Self::U128(inner) => *inner as i8,
            Self::I128(inner) => *inner as i8,
            Self::USIZE(inner) => *inner as i8,
            Self::ISIZE(inner) => *inner as i8,
        }
    }

    fn as_u16(&self) -> u16 {
        match self {
            Self::U8(inner) => *inner as u16,
            Self::I8(inner) => *inner as u16,
            Self::U16(inner) => *inner,
            Self::I16(inner) => *inner as u16,
            Self::U32(inner) => *inner as u16,
            Self::I32(inner) => *inner as u16,
            Self::U64(inner) => *inner as u16,
            Self::I64(inner) => *inner as u16,
            Self::U128(inner) => *inner as u16,
            Self::I128(inner) => *inner as u16,
            Self::USIZE(inner) => *inner as u16,
            Self::ISIZE(inner) => *inner as u16,
        }
    }

    fn as_i16(&self) -> i16 {
        match self {
            Self::U8(inner) => *inner as i16,
            Self::I8(inner) => *inner as i16,
            Self::U16(inner) => *inner as i16,
            Self::I16(inner) => *inner,
            Self::U32(inner) => *inner as i16,
            Self::I32(inner) => *inner as i16,
            Self::U64(inner) => *inner as i16,
            Self::I64(inner) => *inner as i16,
            Self::U128(inner) => *inner as i16,
            Self::I128(inner) => *inner as i16,
            Self::USIZE(inner) => *inner as i16,
            Self::ISIZE(inner) => *inner as i16,
        }
    }

    fn as_u32(&self) -> u32 {
        match self {
            Self::U8(inner) => *inner as u32,
            Self::I8(inner) => *inner as u32,
            Self::U16(inner) => *inner as u32,
            Self::I16(inner) => *inner as u32,
            Self::U32(inner) => *inner,
            Self::I32(inner) => *inner as u32,
            Self::U64(inner) => *inner as u32,
            Self::I64(inner) => *inner as u32,
            Self::U128(inner) => *inner as u32,
            Self::I128(inner) => *inner as u32,
            Self::USIZE(inner) => *inner as u32,
            Self::ISIZE(inner) => *inner as u32,
        }
    }

    fn as_i32(&self) -> i32 {
        match self {
            Self::U8(inner) => *inner as i32,
            Self::I8(inner) => *inner as i32,
            Self::U16(inner) => *inner as i32,
            Self::I16(inner) => *inner as i32,
            Self::U32(inner) => *inner as i32,
            Self::I32(inner) => *inner,
            Self::U64(inner) => *inner as i32,
            Self::I64(inner) => *inner as i32,
            Self::U128(inner) => *inner as i32,
            Self::I128(inner) => *inner as i32,
            Self::USIZE(inner) => *inner as i32,
            Self::ISIZE(inner) => *inner as i32,
        }
    }

    fn as_u64(&self) -> u64 {
        match self {
            Self::U8(inner) => *inner as u64,
            Self::I8(inner) => *inner as u64,
            Self::U16(inner) => *inner as u64,
            Self::I16(inner) => *inner as u64,
            Self::U32(inner) => *inner as u64,
            Self::I32(inner) => *inner as u64,
            Self::U64(inner) => *inner,
            Self::I64(inner) => *inner as u64,
            Self::U128(inner) => *inner as u64,
            Self::I128(inner) => *inner as u64,
            Self::USIZE(inner) => *inner as u64,
            Self::ISIZE(inner) => *inner as u64,
        }
    }

    fn as_i64(&self) -> i64 {
        match self {
            Self::U8(inner) => *inner as i64,
            Self::I8(inner) => *inner as i64,
            Self::U16(inner) => *inner as i64,
            Self::I16(inner) => *inner as i64,
            Self::U32(inner) => *inner as i64,
            Self::I32(inner) => *inner as i64,
            Self::U64(inner) => *inner as i64,
            Self::I64(inner) => *inner,
            Self::U128(inner) => *inner as i64,
            Self::I128(inner) => *inner as i64,
            Self::USIZE(inner) => *inner as i64,
            Self::ISIZE(inner) => *inner as i64,
        }
    }

    fn as_u128(&self) -> u128 {
        match self {
            Self::U8(inner) => *inner as u128,
            Self::I8(inner) => *inner as u128,
            Self::U16(inner) => *inner as u128,
            Self::I16(inner) => *inner as u128,
            Self::U32(inner) => *inner as u128,
            Self::I32(inner) => *inner as u128,
            Self::U64(inner) => *inner as u128,
            Self::I64(inner) => *inner as u128,
            Self::U128(inner) => *inner,
            Self::I128(inner) => *inner as u128,
            Self::USIZE(inner) => *inner as u128,
            Self::ISIZE(inner) => *inner as u128,
        }
    }

    fn as_i128(&self) -> i128 {
        match self {
            Self::U8(inner) => *inner as i128,
            Self::I8(inner) => *inner as i128,
            Self::U16(inner) => *inner as i128,
            Self::I16(inner) => *inner as i128,
            Self::U32(inner) => *inner as i128,
            Self::I32(inner) => *inner as i128,
            Self::U64(inner) => *inner as i128,
            Self::I64(inner) => *inner as i128,
            Self::U128(inner) => *inner as i128,
            Self::I128(inner) => *inner,
            Self::USIZE(inner) => *inner as i128,
            Self::ISIZE(inner) => *inner as i128,
        }
    }

    fn as_usize(&self) -> usize {
        match self {
            Self::U8(inner) => *inner as usize,
            Self::I8(inner) => *inner as usize,
            Self::U16(inner) => *inner as usize,
            Self::I16(inner) => *inner as usize,
            Self::U32(inner) => *inner as usize,
            Self::I32(inner) => *inner as usize,
            Self::U64(inner) => *inner as usize,
            Self::I64(inner) => *inner as usize,
            Self::U128(inner) => *inner as usize,
            Self::I128(inner) => *inner as usize,
            Self::USIZE(inner) => *inner,
            Self::ISIZE(inner) => *inner as usize,
        }
    }

    fn as_isize(&self) -> isize {
        match self {
            Self::U8(inner) => *inner as isize,
            Self::I8(inner) => *inner as isize,
            Self::U16(inner) => *inner as isize,
            Self::I16(inner) => *inner as isize,
            Self::U32(inner) => *inner as isize,
            Self::I32(inner) => *inner as isize,
            Self::U64(inner) => *inner as isize,
            Self::I64(inner) => *inner as isize,
            Self::U128(inner) => *inner as isize,
            Self::I128(inner) => *inner as isize,
            Self::USIZE(inner) => *inner as isize,
            Self::ISIZE(inner) => *inner,
        }
    }
}
