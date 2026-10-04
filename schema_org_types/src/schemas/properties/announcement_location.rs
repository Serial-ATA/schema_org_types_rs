use super::*;
/// <https://schema.org/announcementLocation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AnnouncementLocationProperty {
	/// <https://schema.org/CivicStructure>
	CivicStructure(CivicStructure),
	/// <https://schema.org/LocalBusiness>
	LocalBusiness(LocalBusiness),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
