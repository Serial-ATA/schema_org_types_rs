use super::*;
/// <https://schema.org/hasTierRequirement>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasTierRequirementProperty {
	/// <https://schema.org/CreditCard>
	CreditCard(CreditCard),
	/// <https://schema.org/MonetaryAmount>
	MonetaryAmount(MonetaryAmount),
	/// <https://schema.org/UnitPriceSpecification>
	UnitPriceSpecification(UnitPriceSpecification),
	/// <https://schema.org/Text>
	Text(Text),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
