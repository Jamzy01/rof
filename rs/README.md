# Rust Object Format (.rof)

Rust object format allows rust objects to be serialized to a file in a text format very closely resembling the original object. It also allows easy transfer of objects between different programming langauges, as the objects can be created in their respective language, serialized to string, send to another programming language, deserialzed and used once again.

TL;DR A library that can serialize and deserialize rust objects to string which allows simple file saving and transmission of rust objects between separate programming languages.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![Github][github-badge]][github-url]
[![ISC licensed][isc-badge]][isc-url]

[crates-badge]: https://img.shields.io/badge/crates.io-v0.1.6-blue
[crates-url]: https://crates.io/crates/rof_rs
[docs-badge]: https://img.shields.io/badge/docs.rs-v0.1.6-blue
[docs-url]: https://docs.rs/rof_rs/0.1.6/rof_rs/
[github-badge]: https://img.shields.io/badge/repo-github-blue
[github-url]: https://github.com/Jamzy01/rof/tree/main/rs
[isc-badge]: https://img.shields.io/badge/license-ISC-blue.svg
[isc-url]: https://github.com/Jamzy01/rof/blob/main/LICENSE

### High Level API

```rust
#[derive(RofCompatDerive)]
enum SongGenre {
    ROCK,
    POP,
    HIPHOP,
    JAZZ,
    COUNTRY,
    HEAVYMETAL,
    EDM
}

#[derive(RofCompatDerive)]
struct Song {
    song_title: String,
    song_author: String,
    timestamp: usize,
    song_genre: SongGenre
}

fn main() {
    let mut song =
            Song::load_from_file("C:\\Songs\\Song32.rof");

    song.timestamp += 1; // Increment the timestamp by 1

    song.save_to_file(
            "C:\\Songs\\Song32.rof",
            true /* pretty print option, adds tabs, spaces and newlines to make the file more human-readable, but will not change the data itself in any way */,
        )
        .expect("Could not save song to a file");
}
```

The high level API is as simple as implementing the ```RofCompat``` trait using the ```RofCompatDerive``` macro. This ```RofCompat``` trait allows you to serialize the object back to a low level ```DataValue``` strucutre, which can then be saved to a file. The ```RofCompat``` trait can also deserialize low level data value structures back into it's original form. The ```RofCompat``` trait also provides other utility functions such as

* serialize_to_string(&self, pretty_print: bool) -> String {}
* deserialize_from_string(serialized_rof: &str) -> Self {}
* as_rof(&self) -> Rof {}
* load_from_file(file_path: &str) -> Self {}
* save_to_file(&self, file_path: &str, pretty_print: bool) -> Result<(), ()> {}

Almost any struct or enum can implement the ```RofCompat``` trait by using it's derive macro under these requirements

* All RofCompat objects must implement the ```Default``` attribute
* Due to technical reasons that may be solved in the future, non "object" like struct and enum values cannot be serialized, such as arrays (not vecs), slices, tuples and others. These unserializable properties will simply be ignored by the derive macro, and if you want them to be implemented then you will unfortunately have to implement the ```RofCompat``` trait manually.

As explained above you can implement ```RofCompat``` manually, as shown in the example below, it is recommended that you read over the low level api before trying this for yourself. This allows for more fine tuned control over how exactly your struct/enum is represented in a low level form.

```rust
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
```

The utility functions are implemented for you based on those two functions. As you can see, now the stored data is much more concise and this is a good example of even though not always necessary, sometimes implementing ```RofCompat``` manually is a good idea.

Unlike the low level api, using the ```RofCompat``` trait requires little understanding of how the internal system actually works, and can quickly be setup.

### Low Level API

The ```Rof``` object is responsible for all serializing and deserializing to files in Rof, and backs the higher level ```RofCompat``` trait. You can define a new ```Rof``` as follows.

```rust
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
```

The low level api is there for anybody who wants to use it, although using the high level ```RofCompatDerive``` api is recommended because it is more readable, requires much less boilerplate and is more beginner friendly.