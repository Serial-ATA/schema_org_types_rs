use super::*;
/// <https://schema.org/competitor>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CompetitorProperty {
	/// <https://schema.org/Person>
	Person(Person),
	/// <https://schema.org/SportsTeam>
	SportsTeam(SportsTeam),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
