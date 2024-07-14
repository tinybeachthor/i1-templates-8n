pub use typed_langid::{self, LangId, LangIdUnion};
pub use i1_templates_8n_derive::Template;

pub trait Template<L: LangId> {
    type Output;

    fn render(&self, lang: L) -> Self::Output;
}

impl<T: Template<L> + ?Sized, L: LangId> Template<L> for &T {
    type Output = T::Output;

    #[inline]
    fn render(&self, lang: L) -> Self::Output {
        T::render(self, lang)
    }
}
