use core::fmt;

use cssparser::{match_ignore_ascii_case, ToCss, _cssparser_internal_to_lowercase};
use selectors::visitor::SelectorVisitor;

use super::select_impl::SelectorImpl;
use crate::values::CustomIdent;

macro_rules! apply_non_ts_list {
    ($apply_macro:ident) => {
        $apply_macro! {
            [
                ("link", Link, IN_UNVISITED_STATE, _),
                ("any-link", AnyLink, IN_VISITED_OR_UNVISITED_STATE, _),
                ("visited", Visited, IN_VISITED_STATE, _),
                ("active", Active, IN_ACTIVE_STATE, _),
                ("autofill", Autofill, IN_AUTOFILL_STATE, PSEUDO_CLASS_ENABLED_IN_UA_SHEETS_AND_CHROME),
                ("checked", Checked, IN_CHECKED_STATE, _),
                ("defined", Defined, IN_DEFINED_STATE, _),
                ("disabled", Disabled, IN_DISABLED_STATE, _),
                ("enabled", Enabled, IN_ENABLED_STATE, _),
                ("focus", Focus, IN_FOCUS_STATE, _),
                ("focus-within", FocusWithin, IN_FOCUS_WITHIN_STATE, _),
                ("focus-visible", FocusVisible, IN_FOCUSRING_STATE, _),
                ("hover", Hover, IN_HOVER_STATE, _),
                ("target", Target, IN_TARGET_STATE, _),
                ("indeterminate", Indeterminate, IN_INDETERMINATE_STATE, _),
                ("fullscreen", Fullscreen, IN_FULLSCREEN_STATE, _),

                ("required", Required, IN_REQUIRED_STATE, _),
                ("optional", Optional, IN_OPTIONAL_STATE, _),
                ("valid", Valid, IN_VALID_STATE, _),
                ("invalid", Invalid, IN_INVALID_STATE, _),
                ("in-range", InRange, IN_INRANGE_STATE, _),
                ("out-of-range", OutOfRange, IN_OUTOFRANGE_STATE, _),
                ("default", Default, IN_DEFAULT_STATE, _),
                ("placeholder-shown", PlaceholderShown, IN_PLACEHOLDER_SHOWN_STATE, _),
                ("read-only", ReadOnly, IN_READONLY_STATE, _),
                ("read-write", ReadWrite, IN_READWRITE_STATE, _),
                ("user-valid", UserValid, IN_MOZ_UI_VALID_STATE, _),
                ("user-invalid", UserInvalid, IN_MOZ_UI_INVALID_STATE, _),
            ]
        }
    }
}

macro_rules! pseudo_class_name {
    ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
        /// Our representation of a non tree-structural pseudo-class.
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum NonTSPseudoClass {
            $(
                #[doc = $css]
                $name,
            )*
            /// The `:lang` pseudo-class.
            Lang(CustomIdent),
        }
    }
}
apply_non_ts_list!(pseudo_class_name);

bitflags! {
    // See NonTSPseudoClass::is_enabled_in()
    pub struct NonTSPseudoClassFlag: u8 {
        const PSEUDO_CLASS_ENABLED_IN_UA_SHEETS = 1 << 0;
        const PSEUDO_CLASS_ENABLED_IN_CHROME = 1 << 1;
        const PSEUDO_CLASS_ENABLED_IN_UA_SHEETS_AND_CHROME =
            NonTSPseudoClassFlag::PSEUDO_CLASS_ENABLED_IN_UA_SHEETS.bits |
            NonTSPseudoClassFlag::PSEUDO_CLASS_ENABLED_IN_CHROME.bits;
    }
}

impl ::selectors::parser::NonTSPseudoClass for NonTSPseudoClass {
    type Impl = SelectorImpl;

    #[inline]
    fn is_active_or_hover(&self) -> bool {
        matches!(*self, NonTSPseudoClass::Active | NonTSPseudoClass::Hover)
    }

    #[inline]
    fn is_user_action_state(&self) -> bool {
        matches!(
            *self,
            NonTSPseudoClass::Active | NonTSPseudoClass::Hover | NonTSPseudoClass::Focus
        )
    }

    fn visit<V>(&self, _: &mut V) -> bool
    where
        V: SelectorVisitor<Impl = Self::Impl>,
    {
        true
    }
}

impl ToCss for NonTSPseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        macro_rules! pseudo_class_serialize {
            ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
                match *self {
                    $(NonTSPseudoClass::$name => concat!(":", $css),)*
                    NonTSPseudoClass::Lang(ref s) => {
                        dest.write_str(":lang(")?;
                        s.to_css(dest)?;
                        return dest.write_char(')');
                    },
                }
            }
        }
        let ser = apply_non_ts_list!(pseudo_class_serialize);
        dest.write_str(ser)
    }
}

impl NonTSPseudoClass {
    /// Parses the name and returns a non-ts-pseudo-class if succeeds.
    /// None otherwise. It doesn't check whether the pseudo-class is enabled
    /// in a particular state.
    pub fn parse_non_functional(name: &str) -> Option<Self> {
        macro_rules! pseudo_class_parse {
            ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
                match_ignore_ascii_case! { &name,
                    $($css => Some(NonTSPseudoClass::$name),)*
                    _ => None,
                }
            }
        }
        apply_non_ts_list!(pseudo_class_parse)
    }

    /// Returns true if this pseudo-class has any of the given flags set.
    pub fn has_any_flag(&self, flags: NonTSPseudoClassFlag) -> bool {
        macro_rules! check_flag {
            (_) => {
                false
            };
            ($flags:ident) => {
                NonTSPseudoClassFlag::$flags.intersects(flags)
            };
        }
        macro_rules! pseudo_class_check_is_enabled_in {
            ([$(($css:expr, $name:ident, $state:tt, $flags:tt),)*]) => {
                match *self {
                    $(NonTSPseudoClass::$name => check_flag!($flags),)*
                    NonTSPseudoClass::Lang(_) => false,
                }
            }
        }
        apply_non_ts_list!(pseudo_class_check_is_enabled_in)
    }

    /// Returns whether the pseudo-class is enabled in content sheets.
    #[inline]
    pub fn is_enabled_in_content(&self) -> bool {
        !self.has_any_flag(NonTSPseudoClassFlag::PSEUDO_CLASS_ENABLED_IN_UA_SHEETS_AND_CHROME)
    }
}
