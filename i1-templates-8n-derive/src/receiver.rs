use std::env;
use std::fs;
use std::path::PathBuf;

use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

#[derive(FromDeriveInput)]
#[darling(attributes(template), supports(struct_named, struct_unit), forward_attrs(allow, doc, cfg))]
pub struct Receiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), FieldsReceiver>,
    path: String,
}
impl Receiver {
    fn get_fields(&self) -> Vec<String> {
        self.data
            .as_ref()
            .take_struct()
            .expect("Template only supports named structs and unit structs")
            .into_iter()
            .filter(|field| !field.skip)
            .map(|field| field.name())
            .collect()
    }
    fn get_template(&self) -> String {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = root.join("templates").join(self.path.clone());

        let template = fs::read_to_string(path)
            .expect("Could not read template from file");

        template
    }
}
impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.get_fields();
        let fields_len = fields.len();

        let template = self.get_template();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                const FIELDS: [&'static str; #fields_len] = [
                    #(#fields),*
                ];
                const TEMPLATE: &'static str = #template;
            }
        })
    }
}

#[derive(FromField)]
#[darling(attributes(template))]
struct FieldsReceiver {
    ident: Option<Ident>,
    #[darling(default)]
    skip: bool,
}
impl FieldsReceiver {
    fn name(&self) -> String {
        self.ident
            .as_ref()
            .expect("Template only supports named fields")
            .to_string()
    }
}
