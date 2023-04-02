use crate::object_format::property::property_type::PropertyType;

use super::super::DataValue;

// Float

#[derive(Debug)]
pub enum DataValueFloat {
    F32(f32),
    F64(f64),
}

impl DataValue for DataValueFloat {
    fn serialize(
        &self,
        _: bool,
        _: usize,
    ) -> (
        PropertyType, /* serialized type */
        String,       /* serialized value */
    ) {
        match self {
            Self::F32(inner) => (PropertyType::simple(String::from("f32")), inner.to_string()),
            Self::F64(inner) => (PropertyType::simple(String::from("f64")), inner.to_string()),
        }
    }

    fn deserialize(
        serialized_type: &PropertyType,
        serialized_value: &str,
    ) -> Option<Box<dyn DataValue>>
    where
        Self: Sized,
    {
        if serialized_type.sub_types_included() {
            return None;
        }

        match serialized_type.get_base_type() {
            "f32" => {
                if let Ok(parsed_float) = serialized_value.parse::<f32>() {
                    return Some(Box::new(Self::F32(parsed_float)));
                } else {
                    return None;
                }
            }

            "f64" => {
                if let Ok(parsed_float) = serialized_value.parse::<f64>() {
                    return Some(Box::new(Self::F64(parsed_float)));
                } else {
                    return None;
                }
            }

            _ => None,
        }
    }

    fn clone_data_value(&self) -> Box<dyn DataValue> {
        Box::new(match self {
            Self::F32(value) => Self::F32(value.clone()),
            Self::F64(value) => Self::F64(value.clone()),
        })
    }

    // Number conversions

    fn as_f32(&self) -> f32 {
        match self {
            Self::F32(inner) => *inner,
            Self::F64(inner) => *inner as f32,
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Self::F32(inner) => *inner as f64,
            Self::F64(inner) => *inner,
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            Self::F32(inner) => *inner as u8,
            Self::F64(inner) => *inner as u8,
        }
    }

    fn as_i8(&self) -> i8 {
        match self {
            Self::F32(inner) => *inner as i8,
            Self::F64(inner) => *inner as i8,
        }
    }

    fn as_u16(&self) -> u16 {
        match self {
            Self::F32(inner) => *inner as u16,
            Self::F64(inner) => *inner as u16,
        }
    }

    fn as_i16(&self) -> i16 {
        match self {
            Self::F32(inner) => *inner as i16,
            Self::F64(inner) => *inner as i16,
        }
    }

    fn as_u32(&self) -> u32 {
        match self {
            Self::F32(inner) => *inner as u32,
            Self::F64(inner) => *inner as u32,
        }
    }

    fn as_i32(&self) -> i32 {
        match self {
            Self::F32(inner) => *inner as i32,
            Self::F64(inner) => *inner as i32,
        }
    }

    fn as_u64(&self) -> u64 {
        match self {
            Self::F32(inner) => *inner as u64,
            Self::F64(inner) => *inner as u64,
        }
    }

    fn as_i64(&self) -> i64 {
        match self {
            Self::F32(inner) => *inner as i64,
            Self::F64(inner) => *inner as i64,
        }
    }

    fn as_u128(&self) -> u128 {
        match self {
            Self::F32(inner) => *inner as u128,
            Self::F64(inner) => *inner as u128,
        }
    }

    fn as_i128(&self) -> i128 {
        match self {
            Self::F32(inner) => *inner as i128,
            Self::F64(inner) => *inner as i128,
        }
    }

    fn as_usize(&self) -> usize {
        match self {
            Self::F32(inner) => *inner as usize,
            Self::F64(inner) => *inner as usize,
        }
    }

    fn as_isize(&self) -> isize {
        match self {
            Self::F32(inner) => *inner as isize,
            Self::F64(inner) => *inner as isize,
        }
    }
}
