use super::*;
/// <https://schema.org/interactionService>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum InteractionServiceProperty {
	/// <https://schema.org/SoftwareApplication>
	SoftwareApplication(SoftwareApplication),
	/// <https://schema.org/WebSite>
	WebSite(WebSite),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
