use super::*;
/// <https://schema.org/medicalAudience>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MedicalAudienceProperty {
	/// <https://schema.org/MedicalAudience>
	MedicalAudience(MedicalAudience),
	/// <https://schema.org/MedicalAudienceType>
	MedicalAudienceType(MedicalAudienceType),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
