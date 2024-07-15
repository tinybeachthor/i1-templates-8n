use std::env;
use std::fs;
use std::path::PathBuf;

use strum::IntoEnumIterator;
use proc_macro2::Span;

use typed_langid::LangIdUnion;

pub struct Templates {
    templates: Vec<(LangIdUnion, String)>,
}
impl Templates {
    pub fn find_by_name(name: &str) -> Self {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let mut templates = Vec::new();

        for langid in LangIdUnion::iter() {
            let path = root.join("templates").join(langid.as_str()).join(name);

            if path.is_file() {
                let template = fs::read_to_string(path)
                    .expect("Could not read template from file");

                templates.push((langid, template));
            }
        }

        Self {
            templates,
        }
    }
}
impl IntoIterator for Templates {
    type Item = (syn::Ident, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let transformed: Vec<Self::Item> = self.templates
            .into_iter()
            .map(|(langid, template)| {
                let langid = capitalize(langid.as_str());
                let ident = syn::Ident::new(&langid, Span::call_site());
                (ident, template)
            })
            .collect();

        transformed.into_iter()
    }
}

fn capitalize(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars[0] = chars[0].to_uppercase().nth(0).unwrap();
    chars.into_iter().collect()
}
