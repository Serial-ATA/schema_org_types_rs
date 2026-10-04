use super::*;
/// <https://schema.org/deliveryTime>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DeliveryTimeProperty {
	/// <https://schema.org/ShippingDeliveryTime>
	ShippingDeliveryTime(ShippingDeliveryTime),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
