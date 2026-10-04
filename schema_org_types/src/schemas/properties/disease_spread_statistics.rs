use super::*;
/// <https://schema.org/diseaseSpreadStatistics>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DiseaseSpreadStatisticsProperty {
	/// <https://schema.org/Dataset>
	Dataset(Dataset),
	/// <https://schema.org/Observation>
	Observation(Observation),
	/// <https://schema.org/WebContent>
	WebContent(WebContent),
	/// <https://schema.org/URL>
	Url(Url),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
