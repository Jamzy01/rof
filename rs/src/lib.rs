pub use rof_rs_core as rof;
pub use rof_rs_macros as macros;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, default};

    use rof_rs_core::{
        object_format::{
            data_value::{
                enum_value::DataValueEnum, integer::DataValueInteger, string::DataValueString,
                struct_value::DataValueStruct,
            },
            property::Property,
            rof::Rof,
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
    fn rof_compat_derive_test() {
        let mut user =
            User::load_from_file("J:\\Programming\\Rust\\rof\\example_objects\\user.rof");

        user.age += 1;

        user.save_to_file(
            "J:\\Programming\\Rust\\rof\\example_objects\\user.rof",
            true,
        )
        .expect("Could not save user to file");
    }

    #[derive(Default)]
    struct Color {
        r: u8,
        g: u8,
        b: u8,
    }

    impl RofCompat for Color {
        fn serialize(&self) -> Box<dyn DataValue> {
            Box::new(DataValueInteger::U32(
                65536 * (self.r as u32) + 256 * (self.g as u32) + (self.b as u32),
            ))
        }

        fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
            let color_int: u32 = rof_object.as_u32();

            Self {
                r: (color_int % 16_777_216 / 65_536) as u8,
                g: (color_int % 65_536 / 256) as u8,
                b: (color_int % 256) as u8,
            }
        }
    }

    #[test]
    fn rof_compat_custom_impl_test() {
        let mut color =
            Color::load_from_file("J:\\Programming\\Rust\\rof\\example_objects\\color.rof");

        color.r = (color.r + 1) % 255;

        color
            .save_to_file(
                "J:\\Programming\\Rust\\rof\\example_objects\\color.rof",
                true,
            )
            .expect("Could not save color to file");
    }

    #[derive(Debug, Default)]
    enum ComputerType {
        WINDOWS,
        LINUX,
        MACOS,
        #[default]
        REACTOS,
    }

    #[test]
    fn low_level_rof_test() {
        // Load computer file as a rof

        let computer_rof =
            Rof::load_from_file("J:\\Programming\\Rust\\rof\\example_objects\\computer.rof");

        // Convert the rof to a struct structure

        let mut computer_structure = computer_rof.get_object().as_struct_structure();

        let computer_name: String = match computer_structure.get("computer_name") {
            Some(loaded_computer_name) => loaded_computer_name.as_string(),
            None => String::default(), // ""
        };

        let mut computer_ram: usize = match computer_structure.get("computer_ram") {
            Some(loaded_computer_ram) => loaded_computer_ram.as_usize(),
            None => usize::default(), // 0
        };

        let computer_type: ComputerType = match computer_structure.get("computer_type") {
            Some(loaded_computer_type) => match loaded_computer_type.as_enum_structure().0.as_ref()
            {
                "windows" => ComputerType::WINDOWS,
                "linux" => ComputerType::LINUX,
                "macos" => ComputerType::MACOS,
                "reactos" => ComputerType::REACTOS,
                _ => ComputerType::REACTOS,
            },
            None => ComputerType::default(),
        };

        // Print out the parsed computer

        println!(
            "Loaded {:?} Computer, named {} with {}gb of ram",
            computer_type, computer_name, computer_ram
        );

        // Modify the computer

        computer_ram += 1;

        // Convert the computer back into a struct structure

        let computer_struct_properties: Vec<Property> = vec![
            Property::new(
                String::from("computer_name"),
                Box::new(DataValueString::new(computer_name)), // OR computer_name.serialize()
            ),
            Property::new(
                String::from("computer_ram"),
                Box::new(DataValueInteger::USIZE(computer_ram)), // OR computer_ram.serialize()
            ),
            Property::new(
                String::from("computer_type"),
                Box::new(DataValueEnum::new(
                    match computer_type {
                        ComputerType::WINDOWS => "windows",
                        ComputerType::LINUX => "linux",
                        ComputerType::MACOS => "macos",
                        ComputerType::REACTOS => "reactos",
                    }
                    .to_string(),
                    Vec::new(),
                )),
            ),
        ];

        let computer_struct_structure = DataValueStruct::new(computer_struct_properties);

        // Save rof to computer file

        // Must create a new rof object, becausue the .get_object() function returns an immutable reference

        Rof::new(Box::new(computer_struct_structure))
            .save_to_file(
                "J:\\Programming\\Rust\\rof\\example_objects\\computer.rof",
                true,
            )
            .expect("Could not save computer to file");
    }
}
