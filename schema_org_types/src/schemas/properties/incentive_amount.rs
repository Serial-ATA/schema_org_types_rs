use super::*;
/// <https://schema.org/incentiveAmount>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum IncentiveAmountProperty {
	/// <https://schema.org/LoanOrCredit>
	LoanOrCredit(LoanOrCredit),
	/// <https://schema.org/QuantitativeValue>
	QuantitativeValue(QuantitativeValue),
	/// <https://schema.org/UnitPriceSpecification>
	UnitPriceSpecification(UnitPriceSpecification),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
