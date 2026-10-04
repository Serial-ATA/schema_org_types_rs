use super::*;
/// <https://schema.org/shippingConditions>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ShippingConditionsProperty {
	/// <https://schema.org/ShippingConditions>
	ShippingConditions(ShippingConditions),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
