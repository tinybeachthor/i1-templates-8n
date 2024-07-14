use darling::FromField;
use syn::Ident;

#[derive(FromField)]
#[darling(attributes(template))]
pub struct Field {
    pub ident: Option<Ident>,
    #[darling(default)]
    pub skip: bool,
}
impl Field {
    pub fn ident(&self) -> syn::Ident {
        self.ident
            .clone()
            .expect("Template only supports named fields")
    }
}
