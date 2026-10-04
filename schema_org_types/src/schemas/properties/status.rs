use super::*;
/// <https://schema.org/status>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StatusProperty {
	/// <https://schema.org/EventStatusType>
	EventStatusType(EventStatusType),
	/// <https://schema.org/MedicalStudyStatus>
	MedicalStudyStatus(MedicalStudyStatus),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
