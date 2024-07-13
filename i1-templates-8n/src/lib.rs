mod error;

pub use error::{Error, Result};
pub use i1_templates_8n_derive::Template;

pub trait Template {
    /// Helper method which allocates a new `String` and renders into it
    fn render(&self) -> Result<String> {
        let mut buf = String::new();
        let _ = buf.try_reserve(Self::SIZE_HINT);
        self.render_into(&mut buf)?;
        Ok(buf)
    }

    /// Renders the template to the given `writer` fmt buffer
    fn render_into(&self, writer: &mut (impl std::fmt::Write + ?Sized)) -> Result<()>;

    const SIZE_HINT: usize;
}

impl<T: Template + ?Sized> Template for &T {
    #[inline]
    fn render_into(&self, writer: &mut (impl std::fmt::Write + ?Sized)) -> Result<()> {
        T::render_into(self, writer)
    }

    const SIZE_HINT: usize = T::SIZE_HINT;
}
