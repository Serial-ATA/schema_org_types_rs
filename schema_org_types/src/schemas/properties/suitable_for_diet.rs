use super::*;
/// <https://schema.org/suitableForDiet>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SuitableForDietProperty {
	/// <https://schema.org/Diet>
	Diet(Diet),
	/// <https://schema.org/RestrictedDiet>
	RestrictedDiet(RestrictedDiet),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
