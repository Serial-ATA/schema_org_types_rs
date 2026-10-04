use super::*;
/// <https://schema.org/seasonalOverride>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SeasonalOverrideProperty {
	/// <https://schema.org/OpeningHoursSpecification>
	OpeningHoursSpecification(OpeningHoursSpecification),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
