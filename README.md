# Rust Object Format (.rof)

Rust object format allows rust objects to be serialized to a file in a text format very closely resembling the original object. It also allows easy transfer of objects between different programming langauges, as the objects can be created in their respective language, serialized to string, send to another programming language, deserialzed and used once again.

TL;DR A library that can serialize and deserialize rust objects to string which allows simple file saving and transmission of rust objects between separate programming languages.

## Table of contents

[Design Goals](#design-goals)  
[Language](#language)

## Design Goals

* Human readable (easy to write and supports inline comments, and the file is in a text format)
* Does not require any conversion when serializing and deserializing in rust
* Easy to understand for somebody who has a good understanding of rust
* Inspired by JSON, as a language that closely resembled another language's data structure
* An expansive set of inbuilt data types such as enums, tuples, options, arrays, hashmaps and more

## Use case

* Saving and loading data
* Sharing data easily between

## Language

### Structs

Encapsulated by a pair of curly brackets

```js
{
    title: string = "Ferris the crab";
}
```

Each object property is defined as follows, and always ended by a semicolon.

* Explicit type annotation

_property_name_**:** _property_type_ **=** _property_value_**;**

* Implicit type annotation (only allowed on types where type annotation is optional)

_property_name_ **=** _property_value_**;**

### Simple Property Types

A simple property type only includes a base type

> Boolean Types

* falsy bool > ```false```
* truthy bool > ```true```

_type annotation optional, can be implied_

```rust
is_swimming: bool = false;
```

> Text Types

* string > ```"string"``` (strictly needs double quotes)
* char > ```'c'``` (strictly needs single quotes)

_type annotation optional, can be implied_

> Number Types

_All integer type values are written the same_

* u8 (8-bit unsigned integer) > ```year: u8 = 2023;```
* i8 (8-bit signed integer) > ```distance: i8 = -32;```
* u16 (16-bit unsigned integer) > ```year: u16 = 2023;```
* i16 (16-bit signed integer) > ```distance: i16 = -32;```
* u32 (32-bit unsigned integer) > ```year: u32 = 2023;```
* i32 (32-bit signed integer) > ```distance: i32 = -32;```
* u64 (64-bit unsigned integer) > ```year: u64 = 2023;```
* i64 (64-bit signed integer) > ```distance: i64 = -32;```
* u128 (128-bit unsigned integer) > ```year: u128 = 2023;```
* i128 (128-bit signed integer) > ```distance: i128 = -32;```
* usize (platform dependent unsigned integer) > ```year: usize = 2023;```
* isize (platform dependent signed integer) > ```distance: isize = -32;```

_All floating type values are written the same_

* f32 (32-bit floating point number) > `temperature: f32 = 5.3;`
* f64 (64-bit floating point number) > `weight: f64 = 7;` (unlike rust, floating point numbers must not strictly include a decimal point, in rust this would be written as ```7.``` or ```7.0```)

_type annotation mandatory, cannot be implied_

### Complex Property Types

A complex/compound property includes a base type along with x number of sub types which can be mix of simple and complex property types

> Tuples and Arrays

* tuple > ```position: tuple<f64, f64, f64> = (3, 5.4, 7);```

_type annotation optional if tuple parameter types are optional_

* array > ```pet_names: array<string> = ["Archie", "Charlie", "Luna", "Milo", "Rocky"];```

_type annotation optional if array object type is optional_

> Structs and Hashmaps

* hashmap

```rust
passwords: hashmap<u16, string> = {
    739341: "abc123";
    210405: "football32";
    826135: "dragon97";
};
```

_type annotation optional if hashmap key and value object types are optional_

* struct

```rust
passwords = {
    739341: string = "abc123",
    "210405": string = "football32",
    826135: enum<string> = None;
};
```

_Hashmaps and structs are not interchangeable as there are some key differences_

* struct property index names must be a string unlike hashmap keys which can have any type of value as a key.
* hashmap keys must all have the same type, unlike structs which require you to explicitly annotate the type for each property.

_Type annotation mandatory; If you want your data to be loaded in as a hashmap, then you must annotate the value as a hashmap, or it will be loaded in as a struct, and vice-versa._

> Enums

* enum > ```action: enum = IDLE;```
* enum > ```action: enum = SLEEPING;```

***

* enum > ```action: enum<f64> = JUMP(3.6);```
* enum > ```action: enum<f64, f64> = RUN(50, 31.2);```
* enum > ```action: enum<string> = EAT("Hamburger");```

* Same enum in rust

```rust
enum action {
    IDLE,
    SLEEPING,
    JUMP(f64 /* jump power */),
    RUN(f64 /* x-velocity */, f64 /* y-velocity */),
    EAT(String /* food item */),
}
```

_Type annotation optional if enum parameter types are optional_

_The keyword "option" can also be used to subsitute "enum" and works exactly the same as the "enum" keyword; When using option enums you can then use the "option" keyword instead of "enum" to make it more readable. The "option" keyword will work fine for any other enum types, but is strongly discouraged._

### Formatting

```js
{
    title: string = "Ferris the crab";
    age: usize = 25;
    favourite_color: enum = ORANGE;
    accessories: array<enum<usize>> = [
        SUNGLASSES(3),
        GOLDEN_SHOES(1)
    ];
}
```

* Tabbing is recomended but not mandatory, and is one tab level is expected each nested value from the root object

* Snake case property indexes is recommended but not mandatory

* New lines are recommended but not mandatory and is expected after the first character in each object, and after the end of each property (after the semicolon)