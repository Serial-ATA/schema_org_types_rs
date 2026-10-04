use super::*;
/// <https://schema.org/itemListElement>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemListElementProperty {
	/// <https://schema.org/HowToSection>
	HowToSection(HowToSection),
	/// <https://schema.org/HowToStep>
	HowToStep(HowToStep),
	/// <https://schema.org/ListItem>
	ListItem(ListItem),
	/// <https://schema.org/Thing>
	Thing(Thing),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
