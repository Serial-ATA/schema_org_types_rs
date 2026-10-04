use super::*;
/// <https://schema.org/expressedIn>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ExpressedInProperty {
	/// <https://schema.org/AnatomicalStructure>
	AnatomicalStructure(AnatomicalStructure),
	/// <https://schema.org/AnatomicalSystem>
	AnatomicalSystem(AnatomicalSystem),
	/// <https://schema.org/BioChemEntity>
	BioChemEntity(BioChemEntity),
	/// <https://schema.org/DefinedTerm>
	DefinedTerm(DefinedTerm),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
