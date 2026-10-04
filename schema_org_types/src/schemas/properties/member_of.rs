use super::*;
/// <https://schema.org/memberOf>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MemberOfProperty {
	/// <https://schema.org/MemberProgramTier>
	MemberProgramTier(MemberProgramTier),
	/// <https://schema.org/Organization>
	Organization(Organization),
	/// <https://schema.org/ProgramMembership>
	ProgramMembership(ProgramMembership),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
