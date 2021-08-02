use core::fmt;
use std::fmt::Write;

// A writer tailored for serialising CSS.
///
/// Coupled with SequenceWriter, this allows callers to transparently handle
/// things like comma-separated values etc.
pub struct CssWriter<'w, W: 'w> {
    inner: &'w mut W,
    prefix: Option<&'static str>,
}

impl<'w, W> CssWriter<'w, W>
where
    W: Write,
{
    /// Creates a new `CssWriter`.
    #[inline]
    pub fn new(inner: &'w mut W) -> Self {
        Self {
            inner,
            prefix: Some(""),
        }
    }
}

impl<'w, W> Write for CssWriter<'w, W>
where
    W: Write,
{
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if s.is_empty() {
            return Ok(());
        }
        if let Some(prefix) = self.prefix.take() {
            // We are going to write things, but first we need to write
            // the prefix that was set by `SequenceWriter::item`.
            if !prefix.is_empty() {
                self.inner.write_str(prefix)?;
            }
        }
        self.inner.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        if let Some(prefix) = self.prefix.take() {
            // See comment in `write_str`.
            if !prefix.is_empty() {
                self.inner.write_str(prefix)?;
            }
        }
        self.inner.write_char(c)
    }
}
