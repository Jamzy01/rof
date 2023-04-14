use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Type::Path;
use syn::{
    parse_macro_input,
    Data::{Enum, Struct},
    DeriveInput, Field, FieldsNamed, FieldsUnnamed,
};

/// # Rof Compat Derive Macro
///
/// Derive macro that automatically implements the RofCompat trait for any struct or enum where the struct/enum's fields all implement the RofCompat.
/// The inbuilt types that implement ```RofCompat``` include
/// * bool
/// * char
/// * f32, f64
/// * u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
/// * String (not str)
/// * Vec<T> where T: RofCompat
/// * Hashmap<K, V> where K: RofCompat, V: RofCompat
/// * Option<T> where T: RofCompat
///
/// ## Example
///
/// ```
/// #[derive(RofCompat)]
/// enum SongGenre {
///     ROCK,
///     POP,
///     HIPHOP,
///     RAP,
///     JAZZ,
///     COUNTRY,
///     HEAVYMETAL,
///     EDM,
///     CLASSICAL,
/// }
///
/// #[derive(RofCompat)]
/// struct Song {
///     song_title: String,
///     song_author: String,
///     timestamp: usize,
///     song_genre: SongGenre
/// }
///
/// fn main() {
///     let mut song =
///             Song::load_from_file("C:\\songs\\song_32.rof");
///
///     song.timestamp += 1; // Increment the timestamp by 1
///
///     song.save_to_file(
///             "C:\\songs\\song_32.rof",
///             true /* pretty print option, adds tabs, spaces and newlines to make the file more human-readable, but will not change the data itself in any way */,
///         )
///         .expect("Could not save song to a file");
/// }
/// ```
#[proc_macro_derive(RofCompat)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut serializer = quote! {
        fn serialize(&self) -> Box<dyn rof_rs::object_format::DataValue> {
            Box::new(rof_rs::object_format::data_value::enum_value::DataValueEnum::none())
        }
    };

    let mut deserializer = quote! {
        fn deserialize(rof_object: Box<dyn rof_rs::object_format::DataValue>) -> Self {
            Self::default()
        }
    };

    // Write Serializers And Deserializers

    match data {
        Struct(s) => {
            // Write a Serializer and Deserializer

            let mut serializing_fields: Vec<proc_macro2::TokenStream> = Vec::new();

            let mut deserializing_functions: Vec<proc_macro2::TokenStream> = Vec::new();

            match s.fields {
                syn::Fields::Named(FieldsNamed { named, .. }) => {
                    named.iter().for_each(|Field { ident, ty, .. }| {
                        if ident.is_none() {
                            return;
                        }

                        match ty {
                            syn::Type::Path(path) => {
                                if let Some(type_string) = path.path.segments.iter().next() {
                                    let field_type = &type_string.ident;

                                    let field_name =
                                    ident.as_ref().unwrap().to_string();

                                    serializing_fields.push(quote! {
                                        rof_rs::object_format::property::Property::new(
                                            String::from(#field_name),
                                            self.#ident.serialize(),
                                        )
                                    });

                                    deserializing_functions.push(quote! {
                                        if let Some(#ident) = rof_object_struct.get(#field_name) {
                                            deserialized_struct.#ident = #field_type::deserialize(#ident.clone_data_value());
                                        }
                                    });
                                }
                            }
                            _ => (),
                        }
                    });
                }
                _ => (),
            }

            serializer = quote! {
                fn serialize(&self) -> Box<dyn rof_rs::object_format::DataValue> {
                    Box::new(rof_rs::object_format::data_value::struct_value::DataValueStruct::new(vec![
                        #(#serializing_fields),*
                    ]))
                }
            };

            deserializer = quote! {
                fn deserialize(rof_object: Box<dyn rof_rs::object_format::DataValue>) -> Self {
                    let mut deserialized_struct = Self::default();

                    let rof_object_struct = rof_object.as_struct_structure();

                    #(#deserializing_functions);*

                    deserialized_struct
                }
            };
        }
        Enum(e) => {
            let mut serializing_matches: Vec<proc_macro2::TokenStream> = Vec::new();
            let mut deserializing_matches: Vec<proc_macro2::TokenStream> = Vec::new();

            for variant in e.variants {
                let variant_ident = variant.ident;
                let variant_name = variant_ident.to_string();

                match variant.fields {
                    syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let mut enum_args: Vec<Ident> = Vec::new();

                        let mut serializing_properties: Vec<proc_macro2::TokenStream> = Vec::new();
                        let mut deserializing_properties: Vec<proc_macro2::TokenStream> =
                            Vec::new();

                        for (field_index, field) in unnamed.iter().enumerate() {
                            let arg_name =
                                Ident::new(&format!("arg_{}", field_index), Span::call_site());

                            match &field.ty {
                                Path(path) => {
                                    if let Some(arg_type_string) = path.path.segments.iter().next()
                                    {
                                        let arg_type = &arg_type_string.ident;

                                        enum_args.push(arg_name.clone());

                                        serializing_properties.push(quote! {
                                            rof_rs::object_format::property::Property::unnamed(#arg_type::serialize(#arg_name))
                                        });

                                        deserializing_properties.push(quote! {
                                            #arg_type::deserialize(enum_args
                                                .get(0)
                                                .unwrap_or(&Box::new(#arg_type::default().serialize()))
                                                .clone_data_value())
                                        });
                                    }
                                }
                                _ => (),
                            }
                        }

                        serializing_matches.push(quote! {
                            Self::#variant_ident(#(#enum_args),*) => rof_rs::object_format::data_value::enum_value::DataValueEnum::new(String::from(#variant_name), vec![#(#serializing_properties),*])
                        });

                        deserializing_matches.push(quote! {
                            #variant_name => Self::#variant_ident(#(#deserializing_properties),*)
                        });
                    }
                    syn::Fields::Unit => {
                        serializing_matches.push(quote! {
                            Self::#variant_ident => rof_rs::object_format::data_value::enum_value::DataValueEnum::simple(String::from(#variant_name))
                        });

                        deserializing_matches.push(quote! {
                            #variant_name => Self::#variant_ident
                        })
                    }
                    _ => (),
                }
            }

            serializer = quote! {
                fn serialize(&self) -> Box<dyn rof_rs::object_format::DataValue> {
                    Box::new(match self {
                        #(#serializing_matches),*
                    })
                }
            };

            deserializer = quote! {
                fn deserialize(rof_object: Box<dyn rof_rs::object_format::DataValue>) -> Self {
                    let (enum_name, enum_args) = rof_object.as_enum_structure();

                    match enum_name.as_str() {
                        #(#deserializing_matches),*,
                        _ => Self::default(),
                    }
                }
            };
        }
        _ => (),
    }

    eprintln!(
        "{}",
        quote! {
            impl rof_rs::rof_compat::RofCompat for #ident {
                #serializer

                #deserializer
            }
        }
    );

    quote! {
        impl rof_rs::rof_compat::RofCompat for #ident {
            #serializer

            #deserializer
        }
    }
    .into()
}
