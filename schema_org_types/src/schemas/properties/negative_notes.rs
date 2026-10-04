use super::*;
/// <https://schema.org/negativeNotes>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NegativeNotesProperty {
	/// <https://schema.org/ItemList>
	ItemList(ItemList),
	/// <https://schema.org/ListItem>
	ListItem(ListItem),
	/// <https://schema.org/WebContent>
	WebContent(WebContent),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
