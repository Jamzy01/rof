use crate::{
    object_format::{data_value::enum_value::DataValueEnum, property::Property},
    rof_compat::RofCompat,
};

impl<T: RofCompat> RofCompat for Option<T> {
    fn serialize(&self) -> Box<dyn crate::object_format::DataValue> {
        Box::new(match self {
            Self::None => DataValueEnum::none(),
            Self::Some(inner) => DataValueEnum::new(
                String::from("Some"),
                vec![Property::unnamed(inner.serialize())],
            ),
        })
    }

    fn deserialize(rof_object: Box<dyn crate::object_format::DataValue>) -> Self {
        let enum_structure = rof_object.as_enum_structure();

        match enum_structure.0.as_ref() {
            "None" => Self::None,
            "Some" => match enum_structure.1.get(0) {
                Some(inner) => Some(T::deserialize(inner.clone_data_value())),
                None => Self::None,
            },
            _ => Self::None,
        }
    }
}
