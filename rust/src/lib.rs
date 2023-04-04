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
    use rof_rs_macros::RofCompat;

    #[derive(Default, RofCompat)]
    struct User {
        name: String,
        age: usize,
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
        let mut user =
            User::load_from_file("J:\\Programming\\Rust\\rof\\example_objects\\user.rof");

        user.age += 1;

        user.save_to_file(
            "J:\\Programming\\Rust\\rof\\example_objects\\user.rof",
            false,
        )
        .expect("Could not save user to file");
    }
}
