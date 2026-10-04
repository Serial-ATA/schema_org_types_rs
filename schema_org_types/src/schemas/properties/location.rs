use super::*;
/// <https://schema.org/location>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LocationProperty {
	/// <https://schema.org/Place>
	Place(Place),
	/// <https://schema.org/PostalAddress>
	PostalAddress(PostalAddress),
	/// <https://schema.org/VirtualLocation>
	VirtualLocation(VirtualLocation),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
