use super::*;
/// <https://schema.org/hasTierBenefit>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasTierBenefitProperty {
	/// <https://schema.org/TierBenefitEnumeration>
	TierBenefitEnumeration(TierBenefitEnumeration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
