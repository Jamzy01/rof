use proc_macro::TokenStream;
use quote::quote;
use rof::object_format::data_value::{
    enum_value::DataValueEnum, integer::DataValueInteger, string::DataValueString,
};
use rof::rof_compat::RofCompat;
use rof_rs_core as rof;
use syn::{parse_macro_input, Data::Struct, DeriveInput, Field, FieldsNamed};

#[proc_macro_derive(RofCompat)]
pub fn derive(input: TokenStream) -> TokenStream {
    eprintln!("Derive Rof Compat");

    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let mut serializer = quote! {
        fn serialize(&self) -> Box<dyn DataValue> {
            Box::new(DataValueEnum::none())
        }
    };

    let mut deserializer = quote! {
        fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
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
                                        Property::new(
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
                fn serialize(&self) -> Box<dyn DataValue> {
                    Box::new(DataValueStruct::new(vec![
                        #(#serializing_fields),*
                    ]))
                }
            };

            deserializer = quote! {
                fn deserialize(rof_object: Box<dyn DataValue>) -> Self {
                    let mut deserialized_struct = Self::default();

                    let rof_object_struct = rof_object.as_struct_structure();

                    #(#deserializing_functions);*

                    deserialized_struct
                }
            };
        }
        _ => (),
    }

    quote! {
        impl RofCompat for #ident {
            #serializer

            #deserializer
        }
    }
    .into()
}
