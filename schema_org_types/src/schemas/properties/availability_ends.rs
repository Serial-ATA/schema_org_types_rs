use super::*;
/// <https://schema.org/availabilityEnds>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailabilityEndsProperty {
	/// <https://schema.org/Date>
	Date(Date),
	/// <https://schema.org/Time>
	Time(Time),
	/// <https://schema.org/DateTime>
	DateTime(DateTime),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
