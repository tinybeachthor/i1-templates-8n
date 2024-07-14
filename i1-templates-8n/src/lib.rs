pub use i1_templates_8n_derive::Template;

pub trait Template {
    type Output;

    fn render(&self) -> Self::Output;
}

impl<T: Template + ?Sized> Template for &T {
    type Output = T::Output;

    #[inline]
    fn render(&self) -> Self::Output {
        T::render(self)
    }
}
