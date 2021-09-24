/// Each style rule has an origin, which determines where it enters the cascade.
///
/// <https://drafts.csswg.org/css-cascade/#cascading-origins>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Origin {
	/// <https://drafts.csswg.org/css-cascade/#cascade-origin-user-agent>
	UserAgent = 0x1,

	/// <https://drafts.csswg.org/css-cascade/#cascade-origin-author>
	Author = 0x2,
}
