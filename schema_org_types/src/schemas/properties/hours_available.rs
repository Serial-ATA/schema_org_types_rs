use super::*;
/// <https://schema.org/hoursAvailable>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HoursAvailableProperty {
	/// <https://schema.org/OpeningHoursSpecification>
	OpeningHoursSpecification(OpeningHoursSpecification),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
