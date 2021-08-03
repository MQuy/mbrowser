use core::fmt;
use cssparser::serialize_string;
use std::fmt::Write;

/// Serialises a value according to its CSS representation.
///
/// This trait is implemented for `str` and its friends, serialising the string
/// contents as a CSS quoted string.
///
/// This trait is derivable with `#[derive(ToCss)]`, with the following behaviour:
/// * unit variants get serialised as the `snake-case` representation
///   of their name;
/// * unit variants whose name starts with "Moz" or "Webkit" are prepended
///   with a "-";
/// * if `#[css(comma)]` is found on a variant, its fields are separated by
///   commas, otherwise, by spaces;
/// * if `#[css(function)]` is found on a variant, the variant name gets
///   serialised like unit variants and its fields are surrounded by parentheses;
/// * if `#[css(iterable)]` is found on a function variant, that variant needs
///   to have a single member, and that member needs to be iterable. The
///   iterable will be serialized as the arguments for the function;
/// * an iterable field can also be annotated with `#[css(if_empty = "foo")]`
///   to print `"foo"` if the iterator is empty;
/// * if `#[css(dimension)]` is found on a variant, that variant needs
///   to have a single member. The variant would be serialized as a CSS
///   dimension token, like: <member><identifier>;
/// * if `#[css(skip)]` is found on a field, the `ToCss` call for that field
///   is skipped;
/// * if `#[css(skip_if = "function")]` is found on a field, the `ToCss` call
///   for that field is skipped if `function` returns true. This function is
///   provided the field as an argument;
/// * if `#[css(contextual_skip_if = "function")]` is found on a field, the
///   `ToCss` call for that field is skipped if `function` returns true. This
///   function is given all the fields in the current struct or variant as an
///   argument;
/// * `#[css(represents_keyword)]` can be used on bool fields in order to
///   serialize the field name if the field is true, or nothing otherwise.  It
///   also collects those keywords for `SpecifiedValueInfo`.
/// * finally, one can put `#[css(derive_debug)]` on the whole type, to
///   implement `Debug` by a single call to `ToCss::to_css`.
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
