use core::fmt;
use std::fmt::Write;

use cssparser::serialize_string;

/// Serialises a value according to its CSS representation.
///
/// This trait is implemented for `str` and its friends, serialising the string
/// contents as a CSS quoted string.
pub trait ToCss {
    /// Serialize `self` in CSS syntax, writing to `dest`.
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write;

    /// Serialize `self` in CSS syntax and return a string.
    ///
    /// (This is a convenience wrapper for `to_css` and probably should not be overridden.)
    #[inline]
    fn to_css_string(&self) -> String {
        let mut s = String::new();
        self.to_css(&mut CssWriter::new(&mut s)).unwrap();
        s
    }
}

impl<'a, T> ToCss for &'a T
where
    T: ToCss + ?Sized,
{
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        (*self).to_css(dest)
    }
}

impl ToCss for str {
    #[inline]
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        serialize_string(self, dest)
    }
}

impl ToCss for String {
    #[inline]
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        serialize_string(self, dest)
    }
}

impl<T> ToCss for Option<T>
where
    T: ToCss,
{
    #[inline]
    fn to_css<W>(&self, dest: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        self.as_ref().map_or(Ok(()), |value| value.to_css(dest))
    }
}

impl ToCss for () {
    #[inline]
    fn to_css<W>(&self, _: &mut CssWriter<W>) -> fmt::Result
    where
        W: Write,
    {
        Ok(())
    }
}

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
