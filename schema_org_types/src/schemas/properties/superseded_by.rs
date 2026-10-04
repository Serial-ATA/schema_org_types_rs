use super::*;
/// <https://schema.org/supersededBy>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SupersededByProperty {
	/// <https://schema.org/Class>
	Class(Class),
	/// <https://schema.org/Enumeration>
	Enumeration(Enumeration),
	/// <https://schema.org/Property>
	Property(Property),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
