use crate::{
    object_format::{data_value::array::DataValueArray, property::Property},
    rof_compat::RofCompat,
};

impl<T: RofCompat> RofCompat for Vec<T> {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(DataValueArray::new(
            T::default().serialize().get_type(),
            self.iter()
                .map(|item| Property::unnamed(item.serialize()))
                .collect(),
        ))
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        rof_object
            .as_vec()
            .into_iter()
            .map(|item| T::deserialize(item))
            .collect()
    }
}
