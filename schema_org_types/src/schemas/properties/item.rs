use super::*;
/// <https://schema.org/item>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemProperty {
	/// <https://schema.org/HowToSection>
	HowToSection(HowToSection),
	/// <https://schema.org/HowToStep>
	HowToStep(HowToStep),
	/// <https://schema.org/Thing>
	Thing(Thing),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
