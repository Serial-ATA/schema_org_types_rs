use super::*;
/// <https://schema.org/audio>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AudioProperty {
	/// <https://schema.org/AudioObject>
	AudioObject(AudioObject),
	/// <https://schema.org/Clip>
	Clip(Clip),
	/// <https://schema.org/MusicRecording>
	MusicRecording(MusicRecording),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
