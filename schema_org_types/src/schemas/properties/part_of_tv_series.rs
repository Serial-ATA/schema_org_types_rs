use super::*;
/// <https://schema.org/partOfTVSeries>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[deprecated = "This schema is superseded by <https://schema.org/partOfSeries>."]
pub enum PartOfTvSeriesProperty {
	/// <https://schema.org/TVSeries>
	TvSeries(TvSeries),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
