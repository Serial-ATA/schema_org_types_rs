use super::*;
/// <https://schema.org/itemOffered>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ItemOfferedProperty {
	/// <https://schema.org/AggregateOffer>
	AggregateOffer(AggregateOffer),
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/Event>
	Event(Event),
	/// <https://schema.org/MenuItem>
	MenuItem(MenuItem),
	/// <https://schema.org/Product>
	Product(Product),
	/// <https://schema.org/Service>
	Service(Service),
	/// <https://schema.org/Trip>
	Trip(Trip),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
