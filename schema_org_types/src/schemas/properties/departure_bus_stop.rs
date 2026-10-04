use super::*;
/// <https://schema.org/departureBusStop>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DepartureBusStopProperty {
	/// <https://schema.org/BusStation>
	BusStation(BusStation),
	/// <https://schema.org/BusStop>
	BusStop(BusStop),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
