use super::*;
/// <https://schema.org/repeatFrequency>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RepeatFrequencyProperty {
	/// <https://schema.org/Text>
	Text(Text),
	/// <https://schema.org/Duration>
	Duration(Duration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
