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
    ident: Ident,
    generics: syn::Generics,
    data: ast::Data<(), Field>,
    path: String,
}
impl Receiver {
    fn get_fields(&self) -> Vec<Ident> {
        self.data
            .as_ref()
            .take_struct()
            .expect("Template only supports named structs and unit structs")
            .into_iter()
            .filter(|field| !field.skip)
            .map(Field::ident)
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

        let template = self.get_template();
        let template_len = template.len();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                const TEMPLATE: &'static str = #template;
            }
            #[automatically_derived]
            impl #impl_generics ::i1_templates_8n::Template for #ident #ty_generics #where_clause {
                fn render_into(&self, writer: &mut (impl std::fmt::Write + ?Sized)) -> ::i1_templates_8n::Result<()> {
                    let #ident { #(#fields),* , .. } = self;
                    std::write!(writer, #template)?;
                    Ok(())
                }

                const SIZE_HINT: usize = #template_len;
            }
        })
    }
}

#[derive(FromField)]
#[darling(attributes(template))]
struct Field {
    ident: Option<Ident>,
    #[darling(default)]
    skip: bool,
}
impl Field {
    fn ident(&self) -> syn::Ident {
        self.ident
            .clone()
            .expect("Template only supports named fields")
    }
}
