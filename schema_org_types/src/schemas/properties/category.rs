use super::*;
/// <https://schema.org/category>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CategoryProperty {
	/// <https://schema.org/CategoryCode>
	CategoryCode(CategoryCode),
	/// <https://schema.org/Thing>
	Thing(Thing),
	/// <https://schema.org/PhysicalActivityCategory>
	PhysicalActivityCategory(PhysicalActivityCategory),
	/// <https://schema.org/URL>
	Url(Url),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
