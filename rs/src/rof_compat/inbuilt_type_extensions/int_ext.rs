use crate::{object_format::data_value::integer::DataValueInteger, rof_compat::RofCompat};

impl RofCompat for u8 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::U8(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_u8()
    }
}

impl RofCompat for i8 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::I8(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_i8()
    }
}

impl RofCompat for u16 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::U16(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_u16()
    }
}

impl RofCompat for i16 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::I16(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_i16()
    }
}

impl RofCompat for u32 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::U32(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_u32()
    }
}

impl RofCompat for i32 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::I32(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_i32()
    }
}

impl RofCompat for u64 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::U64(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_u64()
    }
}

impl RofCompat for i64 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::I64(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_i64()
    }
}

impl RofCompat for u128 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::U128(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_u128()
    }
}

impl RofCompat for i128 {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::I128(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_i128()
    }
}

impl RofCompat for usize {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::USIZE(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_usize()
    }
}

impl RofCompat for isize {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueInteger::ISIZE(*self))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object.as_isize()
    }
}
