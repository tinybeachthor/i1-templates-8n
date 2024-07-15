use darling::{ast, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

use crate::field::Field;
use crate::templates::Templates;

#[derive(FromDeriveInput)]
#[darling(attributes(template), supports(struct_named, struct_unit), forward_attrs(allow, doc, cfg))]
pub struct Receiver {
    ident: Ident,
    generics: syn::Generics,
    data: ast::Data<(), Field>,
    name: String,
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
}
impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.get_fields();
        let templates = Templates::find_by_name(&self.name);

        for (ident_lang, template) in templates {
            tokens.extend(quote! {
                #[automatically_derived]
                impl #impl_generics ::i1_templates_8n::Template<::i1_templates_8n::typed_langid::#ident_lang> for #ident #ty_generics #where_clause {
                    type Output = String;

                    fn render(&self, _lang: ::i1_templates_8n::typed_langid::#ident_lang) -> Self::Output {
                        let #ident { #(#fields),* , .. } = self;
                        std::format!(#template)
                    }
                }
            })
        }
    }
}
