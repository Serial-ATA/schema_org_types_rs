use super::*;
/// <https://schema.org/maximumIntake>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MaximumIntakeProperty {
	/// <https://schema.org/MaximumDoseSchedule>
	MaximumDoseSchedule(MaximumDoseSchedule),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
