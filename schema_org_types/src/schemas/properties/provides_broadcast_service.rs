use super::*;
/// <https://schema.org/providesBroadcastService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ProvidesBroadcastServiceProperty {
	/// <https://schema.org/BroadcastService>
	BroadcastService(BroadcastService),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
