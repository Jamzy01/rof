pub use rof_rs_core as rof;
pub use rof_rs_macros as macros;

#[cfg(test)]
mod tests {
    use rof_rs_core::{
        object_format::{
            data_value::{
                integer::DataValueInteger, string::DataValueString, struct_value::DataValueStruct,
            },
            property::Property,
            DataValue,
        },
        rof_compat::RofCompat,
    };

    #[derive(Default)]
    struct User {
        name: String,
        age: usize,
    }

    impl RofCompat for User {
        fn serialize(&self) -> Box<dyn DataValue> {
            Box::new(DataValueStruct::new(vec![
                Property::new(
                    String::from("name"),
                    Box::new(DataValueString::new(self.name.clone())),
                ),
                Property::new(
                    String::from("age"),
                    Box::new(DataValueInteger::USIZE(self.age)),
                ),
            ]))
        }

        fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
            let mut user = Self::default();

            let rof_object_struct = rof_object.as_struct_structure();

            if let Some(name) = rof_object_struct.get("name") {
                user.name = name.as_string();
            }

            if let Some(age) = rof_object_struct.get("age") {
                user.age = age.as_usize();
            }

            user
        }
    }

    #[test]
    fn deserializer_test() {
        let mut user = User::load_from_file(
            "J:\\Programming\\Rust\\rust-object-format\\example_objects\\user.rof",
        );

        user.age += 1;

        user.save_to_file(
            "J:\\Programming\\Rust\\rust-object-format\\example_objects\\user.rof",
            false,
        )
        .expect("Could not save user to file");
    }
}
