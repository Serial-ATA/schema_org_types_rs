use super::*;
/// <https://schema.org/video>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum VideoProperty {
	/// <https://schema.org/Clip>
	Clip(Clip),
	/// <https://schema.org/VideoObject>
	VideoObject(VideoObject),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
