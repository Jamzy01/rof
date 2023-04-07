pub use rof_rs_core as rof;
pub use rof_rs_macros as macros;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use rof_rs_core::{
        object_format::{
            data_value::{enum_value::DataValueEnum, struct_value::DataValueStruct},
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
        friends: Vec<String>,
        passwords: HashMap<usize, String>,
        iq: usize,
    }

    #[derive(Debug, Default, RofCompat)]
    enum UserState {
        Walking(f32),
        #[default]
        Sleeping,
        Jumping,
        Eating(String),
    }

    #[test]
    fn deserializer_test() {
        let mut user =
            User::load_from_file("J:\\Programming\\Rust\\rof\\example_objects\\user.rof");

        user.age += 1;

        user.save_to_file(
            "J:\\Programming\\Rust\\rof\\example_objects\\user.rof",
            true,
        )
        .expect("Could not save user to file");
    }
}
