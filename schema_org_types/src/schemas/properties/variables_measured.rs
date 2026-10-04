use super::*;
/// <https://schema.org/variablesMeasured>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[deprecated = "This schema is archived, see <https://schema.org/docs/attic.home.html>."]
pub enum VariablesMeasuredProperty {
	/// <https://schema.org/PropertyValue>
	PropertyValue(PropertyValue),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
