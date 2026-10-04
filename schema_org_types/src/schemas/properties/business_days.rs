use super::*;
/// <https://schema.org/businessDays>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BusinessDaysProperty {
	/// <https://schema.org/OpeningHoursSpecification>
	OpeningHoursSpecification(OpeningHoursSpecification),
	/// <https://schema.org/DayOfWeek>
	DayOfWeek(DayOfWeek),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
