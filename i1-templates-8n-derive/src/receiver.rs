use darling::{ast, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

#[derive(FromDeriveInput)]
#[darling(attributes(template), supports(struct_named, struct_unit))]
pub struct Receiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: ast::Data<(), FieldsReceiver>,
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
}
impl ToTokens for Receiver {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let fields = self.get_fields();
        let fields_len = fields.len();

        tokens.extend(quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                const FIELDS: [&'static str; #fields_len] = [
                    #(#fields),*
                ];
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
