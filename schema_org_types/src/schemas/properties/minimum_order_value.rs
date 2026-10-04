use super::*;
/// <https://schema.org/minimumOrderValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MinimumOrderValueProperty {
	/// <https://schema.org/MonetaryAmount>
	MonetaryAmount(MonetaryAmount),
	/// <https://schema.org/PriceSpecification>
	PriceSpecification(PriceSpecification),
	/// <https://schema.org/Number>
	Number(Number),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
