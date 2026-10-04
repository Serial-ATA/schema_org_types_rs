use super::*;
/// <https://schema.org/billingDuration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BillingDurationProperty {
	/// <https://schema.org/QuantitativeValue>
	QuantitativeValue(QuantitativeValue),
	/// <https://schema.org/Number>
	Number(Number),
	/// <https://schema.org/Duration>
	Duration(Duration),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
