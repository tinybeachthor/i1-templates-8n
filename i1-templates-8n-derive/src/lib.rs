mod field;
mod receiver;
mod templates;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    receiver::Receiver::from_derive_input(&parse_macro_input!(input as DeriveInput))
        .map(|receiver| quote!(#receiver))
        .unwrap_or_else(|err| err.write_errors())
        .into()
}
