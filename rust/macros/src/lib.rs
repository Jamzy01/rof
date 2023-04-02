use proc_macro::TokenStream;
use rof_rs_core as rof;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RofCompat)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident: struct_name_ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    input
}
