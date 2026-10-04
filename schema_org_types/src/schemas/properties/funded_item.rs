use super::*;
/// <https://schema.org/fundedItem>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FundedItemProperty {
	/// <https://schema.org/BioChemEntity>
	BioChemEntity(BioChemEntity),
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/Event>
	Event(Event),
	/// <https://schema.org/MedicalEntity>
	MedicalEntity(MedicalEntity),
	/// <https://schema.org/Organization>
	Organization(Organization),
	/// <https://schema.org/Person>
	Person(Person),
	/// <https://schema.org/Product>
	Product(Product),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
