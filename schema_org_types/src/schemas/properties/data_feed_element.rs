use super::*;
/// <https://schema.org/dataFeedElement>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataFeedElementProperty {
	/// <https://schema.org/DataFeedItem>
	DataFeedItem(DataFeedItem),
	/// <https://schema.org/Thing>
	Thing(Thing),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
