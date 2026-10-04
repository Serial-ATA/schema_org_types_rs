use super::*;
/// <https://schema.org/geoRadius>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoRadiusProperty {
	/// <https://schema.org/Number>
	Number(Number),
	/// <https://schema.org/Text>
	Text(Text),
	/// <https://schema.org/Distance>
	Distance(Distance),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
