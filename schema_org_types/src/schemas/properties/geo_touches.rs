use super::*;
/// <https://schema.org/geoTouches>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum GeoTouchesProperty {
	/// <https://schema.org/GeospatialGeometry>
	GeospatialGeometry(GeospatialGeometry),
	/// <https://schema.org/Place>
	Place(Place),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
