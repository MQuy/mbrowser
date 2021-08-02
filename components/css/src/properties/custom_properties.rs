/// Parse a custom property name.
///
/// <https://drafts.csswg.org/css-variables/#typedef-custom-property-name>
pub fn parse_name(s: &str) -> Result<&str, ()> {
    if s.starts_with("--") {
        Ok(&s[2..])
    } else {
        Err(())
    }
}
