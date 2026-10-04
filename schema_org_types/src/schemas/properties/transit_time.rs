use super::*;
/// <https://schema.org/transitTime>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TransitTimeProperty {
	/// <https://schema.org/QuantitativeValue>
	QuantitativeValue(QuantitativeValue),
	/// <https://schema.org/ServicePeriod>
	ServicePeriod(ServicePeriod),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
