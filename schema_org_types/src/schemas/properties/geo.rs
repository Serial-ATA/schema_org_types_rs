use super::*;
/// <https://schema.org/geo>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoProperty {
	/// <https://schema.org/GeoCoordinates>
	GeoCoordinates(GeoCoordinates),
	/// <https://schema.org/GeoShape>
	GeoShape(GeoShape),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
