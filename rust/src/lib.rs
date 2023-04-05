pub use rof_rs_core as rof;
pub use rof_rs_macros as macros;

#[cfg(test)]
mod tests {
    use rof_rs_core::{
        object_format::{
            data_value::{
                enum_value::DataValueEnum, integer::DataValueInteger, string::DataValueString,
                struct_value::DataValueStruct,
            },
            property::Property,
            DataValue,
        },
        rof_compat::RofCompat,
    };
    use rof_rs_macros::RofCompat;

    #[derive(Debug, Default, RofCompat)]
    struct User {
        name: String,
        age: usize,
        state: UserState,
    }

    #[derive(Debug, Default)]
    enum UserState {
        Walking(f32),
        #[default]
        Sleeping,
        Jumping,
        Eating(String),
    }

    impl RofCompat for UserState {
        fn serialize(&self) -> Box<dyn DataValue> {
            Box::new(match self {
                Self::Walking(arg_0) => DataValueEnum::new(
                    String::from("Walking"),
                    vec![Property::unnamed(f32::serialize(arg_0))],
                ),
                Self::Sleeping => DataValueEnum::simple(String::from("Walking")),
                Self::Jumping => DataValueEnum::simple(String::from("Jumping")),
                Self::Eating(arg_0) => DataValueEnum::new(
                    String::from("Eating"),
                    vec![Property::unnamed(String::serialize(arg_0))],
                ),
            })
        }

        fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
            let (enum_name, enum_args) = rof_object.as_enum_structure();

            match enum_name.as_str() {
                "Walking" => Self::Walking(f32::deserialize(
                    enum_args
                        .get(0)
                        .unwrap_or(&Box::new(usize::default().serialize()))
                        .clone_data_value(),
                )),
                "Sleeping" => Self::Sleeping,
                "Jumping" => Self::Jumping,
                "Eating" => Self::Eating(String::deserialize(
                    enum_args
                        .get(0)
                        .unwrap_or(&Box::new(String::default().serialize()))
                        .clone_data_value(),
                )),
                _ => Self::default(),
            }
        }
    }

    /*

    impl RofCompat for User {
        fn serialize(&self) -> Box<dyn DataValue> {
            Box::new(DataValueStruct::new(vec![
                Property::new(String::from("name"), self.name.serialize()),
                Property::new(String::from("age"), self.age.serialize()),
            ]))
        }

        fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
            let mut user = Self::default();

            let rof_object_struct = rof_object.as_struct_structure();

            if let Some(name) = rof_object_struct.get("name") {
                user.name = String::deserialize(name.clone_data_value());
            }

            if let Some(age) = rof_object_struct.get("age") {
                user.age = usize::deserialize(age.clone_data_value());
            }

            user
        }
    }

    */

    #[test]
    fn deserializer_test() {
        let mut user = User::load_from_file(
            "C:\\Users\\JamesG\\Documents\\Programming\\rof\\example_objects\\user.rof",
        );

        user.age += 1;

        println!("{:?}", user);

        user.save_to_file(
            "C:\\Users\\JamesG\\Documents\\Programming\\rof\\example_objects\\user.rof",
            true,
        )
        .expect("Could not save user to file");
    }
}
