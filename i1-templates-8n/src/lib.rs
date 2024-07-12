mod error;

use std::fmt;

pub use error::{Error, Result};

pub trait Template: fmt::Display {
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
