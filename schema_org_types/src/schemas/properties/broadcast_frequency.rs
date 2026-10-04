use super::*;
/// <https://schema.org/broadcastFrequency>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BroadcastFrequencyProperty {
	/// <https://schema.org/BroadcastFrequencySpecification>
	BroadcastFrequencySpecification(BroadcastFrequencySpecification),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
