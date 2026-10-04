use super::*;
/// <https://schema.org/photos>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[deprecated = "This schema is superseded by <https://schema.org/photo>."]
pub enum PhotosProperty {
	/// <https://schema.org/ImageObject>
	ImageObject(ImageObject),
	/// <https://schema.org/Photograph>
	Photograph(Photograph),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
