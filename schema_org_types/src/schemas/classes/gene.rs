use super::*;
/// <https://schema.org/Gene>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Gene {
	/// <https://schema.org/alternativeOf>
	#[cfg_attr(feature = "serde", serde(rename = "alternativeOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#alternative_of: Vec<AlternativeOfProperty>,
	/// <https://schema.org/encodesBioChemEntity>
	#[cfg_attr(feature = "serde", serde(rename = "encodesBioChemEntity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#encodes_bio_chem_entity: Vec<EncodesBioChemEntityProperty>,
	/// <https://schema.org/expressedIn>
	#[cfg_attr(feature = "serde", serde(rename = "expressedIn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#expressed_in: Vec<ExpressedInProperty>,
	/// <https://schema.org/hasBioPolymerSequence>
	#[cfg_attr(feature = "serde", serde(rename = "hasBioPolymerSequence"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_bio_polymer_sequence: Vec<HasBioPolymerSequenceProperty>,
	/// <https://schema.org/associatedDisease>
	#[cfg_attr(feature = "serde", serde(rename = "associatedDisease"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#associated_disease: Vec<AssociatedDiseaseProperty>,
	/// <https://schema.org/bioChemInteraction>
	#[cfg_attr(feature = "serde", serde(rename = "bioChemInteraction"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#bio_chem_interaction: Vec<BioChemInteractionProperty>,
	/// <https://schema.org/bioChemSimilarity>
	#[cfg_attr(feature = "serde", serde(rename = "bioChemSimilarity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#bio_chem_similarity: Vec<BioChemSimilarityProperty>,
	/// <https://schema.org/biologicalRole>
	#[cfg_attr(feature = "serde", serde(rename = "biologicalRole"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#biological_role: Vec<BiologicalRoleProperty>,
	/// <https://schema.org/funding>
	#[cfg_attr(feature = "serde", serde(rename = "funding"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#funding: Vec<FundingProperty>,
	/// <https://schema.org/hasBioChemEntityPart>
	#[cfg_attr(feature = "serde", serde(rename = "hasBioChemEntityPart"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_bio_chem_entity_part: Vec<HasBioChemEntityPartProperty>,
	/// <https://schema.org/hasMolecularFunction>
	#[cfg_attr(feature = "serde", serde(rename = "hasMolecularFunction"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_molecular_function: Vec<HasMolecularFunctionProperty>,
	/// <https://schema.org/hasRepresentation>
	#[cfg_attr(feature = "serde", serde(rename = "hasRepresentation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_representation: Vec<HasRepresentationProperty>,
	/// <https://schema.org/isEncodedByBioChemEntity>
	#[cfg_attr(feature = "serde", serde(rename = "isEncodedByBioChemEntity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_encoded_by_bio_chem_entity: Vec<IsEncodedByBioChemEntityProperty>,
	/// <https://schema.org/isInvolvedInBiologicalProcess>
	#[cfg_attr(feature = "serde", serde(rename = "isInvolvedInBiologicalProcess"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_involved_in_biological_process: Vec<IsInvolvedInBiologicalProcessProperty>,
	/// <https://schema.org/isLocatedInSubcellularLocation>
	#[cfg_attr(feature = "serde", serde(rename = "isLocatedInSubcellularLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_located_in_subcellular_location: Vec<IsLocatedInSubcellularLocationProperty>,
	/// <https://schema.org/isPartOfBioChemEntity>
	#[cfg_attr(feature = "serde", serde(rename = "isPartOfBioChemEntity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_part_of_bio_chem_entity: Vec<IsPartOfBioChemEntityProperty>,
	/// <https://schema.org/taxonomicRange>
	#[cfg_attr(feature = "serde", serde(rename = "taxonomicRange"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#taxonomic_range: Vec<TaxonomicRangeProperty>,
	/// <https://schema.org/additionalType>
	#[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	/// <https://schema.org/alternateName>
	#[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	/// <https://schema.org/description>
	#[cfg_attr(feature = "serde", serde(rename = "description"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#description: Vec<DescriptionProperty>,
	/// <https://schema.org/disambiguatingDescription>
	#[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	/// <https://schema.org/identifier>
	#[cfg_attr(feature = "serde", serde(rename = "identifier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#identifier: Vec<IdentifierProperty>,
	/// <https://schema.org/image>
	#[cfg_attr(feature = "serde", serde(rename = "image"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#image: Vec<ImageProperty>,
	/// <https://schema.org/mainEntityOfPage>
	#[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	/// <https://schema.org/name>
	#[cfg_attr(feature = "serde", serde(rename = "name"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#name: Vec<NameProperty>,
	/// <https://schema.org/owner>
	#[cfg_attr(feature = "serde", serde(rename = "owner"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#owner: Vec<OwnerProperty>,
	/// <https://schema.org/potentialAction>
	#[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#potential_action: Vec<PotentialActionProperty>,
	/// <https://schema.org/sameAs>
	#[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#same_as: Vec<SameAsProperty>,
	/// <https://schema.org/subjectOf>
	#[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#subject_of: Vec<SubjectOfProperty>,
	/// <https://schema.org/url>
	#[cfg_attr(feature = "serde", serde(rename = "url"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#url: Vec<UrlProperty>,
}
/// This trait is for properties from <https://schema.org/Gene>.
pub trait GeneTrait {
	/// Get <https://schema.org/alternativeOf> from [`Self`] as borrowed slice.
	fn get_alternative_of(&self) -> &[AlternativeOfProperty];
	/// Take <https://schema.org/alternativeOf> from [`Self`] as owned vector.
	fn take_alternative_of(&mut self) -> Vec<AlternativeOfProperty>;
	/// Get <https://schema.org/encodesBioChemEntity> from [`Self`] as borrowed slice.
	fn get_encodes_bio_chem_entity(&self) -> &[EncodesBioChemEntityProperty];
	/// Take <https://schema.org/encodesBioChemEntity> from [`Self`] as owned vector.
	fn take_encodes_bio_chem_entity(&mut self) -> Vec<EncodesBioChemEntityProperty>;
	/// Get <https://schema.org/expressedIn> from [`Self`] as borrowed slice.
	fn get_expressed_in(&self) -> &[ExpressedInProperty];
	/// Take <https://schema.org/expressedIn> from [`Self`] as owned vector.
	fn take_expressed_in(&mut self) -> Vec<ExpressedInProperty>;
	/// Get <https://schema.org/hasBioPolymerSequence> from [`Self`] as borrowed slice.
	fn get_has_bio_polymer_sequence(&self) -> &[HasBioPolymerSequenceProperty];
	/// Take <https://schema.org/hasBioPolymerSequence> from [`Self`] as owned vector.
	fn take_has_bio_polymer_sequence(&mut self) -> Vec<HasBioPolymerSequenceProperty>;
}
impl GeneTrait for Gene {
	fn get_alternative_of(&self) -> &[AlternativeOfProperty] {
		self.r#alternative_of.as_slice()
	}
	fn take_alternative_of(&mut self) -> Vec<AlternativeOfProperty> {
		std::mem::take(&mut self.r#alternative_of)
	}
	fn get_encodes_bio_chem_entity(&self) -> &[EncodesBioChemEntityProperty] {
		self.r#encodes_bio_chem_entity.as_slice()
	}
	fn take_encodes_bio_chem_entity(&mut self) -> Vec<EncodesBioChemEntityProperty> {
		std::mem::take(&mut self.r#encodes_bio_chem_entity)
	}
	fn get_expressed_in(&self) -> &[ExpressedInProperty] {
		self.r#expressed_in.as_slice()
	}
	fn take_expressed_in(&mut self) -> Vec<ExpressedInProperty> {
		std::mem::take(&mut self.r#expressed_in)
	}
	fn get_has_bio_polymer_sequence(&self) -> &[HasBioPolymerSequenceProperty] {
		self.r#has_bio_polymer_sequence.as_slice()
	}
	fn take_has_bio_polymer_sequence(&mut self) -> Vec<HasBioPolymerSequenceProperty> {
		std::mem::take(&mut self.r#has_bio_polymer_sequence)
	}
}
impl BioChemEntityTrait for Gene {
	fn get_associated_disease(&self) -> &[AssociatedDiseaseProperty] {
		self.r#associated_disease.as_slice()
	}
	fn take_associated_disease(&mut self) -> Vec<AssociatedDiseaseProperty> {
		std::mem::take(&mut self.r#associated_disease)
	}
	fn get_bio_chem_interaction(&self) -> &[BioChemInteractionProperty] {
		self.r#bio_chem_interaction.as_slice()
	}
	fn take_bio_chem_interaction(&mut self) -> Vec<BioChemInteractionProperty> {
		std::mem::take(&mut self.r#bio_chem_interaction)
	}
	fn get_bio_chem_similarity(&self) -> &[BioChemSimilarityProperty] {
		self.r#bio_chem_similarity.as_slice()
	}
	fn take_bio_chem_similarity(&mut self) -> Vec<BioChemSimilarityProperty> {
		std::mem::take(&mut self.r#bio_chem_similarity)
	}
	fn get_biological_role(&self) -> &[BiologicalRoleProperty] {
		self.r#biological_role.as_slice()
	}
	fn take_biological_role(&mut self) -> Vec<BiologicalRoleProperty> {
		std::mem::take(&mut self.r#biological_role)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_has_bio_chem_entity_part(&self) -> &[HasBioChemEntityPartProperty] {
		self.r#has_bio_chem_entity_part.as_slice()
	}
	fn take_has_bio_chem_entity_part(&mut self) -> Vec<HasBioChemEntityPartProperty> {
		std::mem::take(&mut self.r#has_bio_chem_entity_part)
	}
	fn get_has_molecular_function(&self) -> &[HasMolecularFunctionProperty] {
		self.r#has_molecular_function.as_slice()
	}
	fn take_has_molecular_function(&mut self) -> Vec<HasMolecularFunctionProperty> {
		std::mem::take(&mut self.r#has_molecular_function)
	}
	fn get_has_representation(&self) -> &[HasRepresentationProperty] {
		self.r#has_representation.as_slice()
	}
	fn take_has_representation(&mut self) -> Vec<HasRepresentationProperty> {
		std::mem::take(&mut self.r#has_representation)
	}
	fn get_is_encoded_by_bio_chem_entity(&self) -> &[IsEncodedByBioChemEntityProperty] {
		self.r#is_encoded_by_bio_chem_entity.as_slice()
	}
	fn take_is_encoded_by_bio_chem_entity(&mut self) -> Vec<IsEncodedByBioChemEntityProperty> {
		std::mem::take(&mut self.r#is_encoded_by_bio_chem_entity)
	}
	fn get_is_involved_in_biological_process(&self) -> &[IsInvolvedInBiologicalProcessProperty] {
		self.r#is_involved_in_biological_process.as_slice()
	}
	fn take_is_involved_in_biological_process(
		&mut self,
	) -> Vec<IsInvolvedInBiologicalProcessProperty> {
		std::mem::take(&mut self.r#is_involved_in_biological_process)
	}
	fn get_is_located_in_subcellular_location(&self) -> &[IsLocatedInSubcellularLocationProperty] {
		self.r#is_located_in_subcellular_location.as_slice()
	}
	fn take_is_located_in_subcellular_location(
		&mut self,
	) -> Vec<IsLocatedInSubcellularLocationProperty> {
		std::mem::take(&mut self.r#is_located_in_subcellular_location)
	}
	fn get_is_part_of_bio_chem_entity(&self) -> &[IsPartOfBioChemEntityProperty] {
		self.r#is_part_of_bio_chem_entity.as_slice()
	}
	fn take_is_part_of_bio_chem_entity(&mut self) -> Vec<IsPartOfBioChemEntityProperty> {
		std::mem::take(&mut self.r#is_part_of_bio_chem_entity)
	}
	fn get_taxonomic_range(&self) -> &[TaxonomicRangeProperty] {
		self.r#taxonomic_range.as_slice()
	}
	fn take_taxonomic_range(&mut self) -> Vec<TaxonomicRangeProperty> {
		std::mem::take(&mut self.r#taxonomic_range)
	}
}
impl ThingTrait for Gene {
	fn get_additional_type(&self) -> &[AdditionalTypeProperty] {
		self.r#additional_type.as_slice()
	}
	fn take_additional_type(&mut self) -> Vec<AdditionalTypeProperty> {
		std::mem::take(&mut self.r#additional_type)
	}
	fn get_alternate_name(&self) -> &[AlternateNameProperty] {
		self.r#alternate_name.as_slice()
	}
	fn take_alternate_name(&mut self) -> Vec<AlternateNameProperty> {
		std::mem::take(&mut self.r#alternate_name)
	}
	fn get_description(&self) -> &[DescriptionProperty] {
		self.r#description.as_slice()
	}
	fn take_description(&mut self) -> Vec<DescriptionProperty> {
		std::mem::take(&mut self.r#description)
	}
	fn get_disambiguating_description(&self) -> &[DisambiguatingDescriptionProperty] {
		self.r#disambiguating_description.as_slice()
	}
	fn take_disambiguating_description(&mut self) -> Vec<DisambiguatingDescriptionProperty> {
		std::mem::take(&mut self.r#disambiguating_description)
	}
	fn get_identifier(&self) -> &[IdentifierProperty] {
		self.r#identifier.as_slice()
	}
	fn take_identifier(&mut self) -> Vec<IdentifierProperty> {
		std::mem::take(&mut self.r#identifier)
	}
	fn get_image(&self) -> &[ImageProperty] {
		self.r#image.as_slice()
	}
	fn take_image(&mut self) -> Vec<ImageProperty> {
		std::mem::take(&mut self.r#image)
	}
	fn get_main_entity_of_page(&self) -> &[MainEntityOfPageProperty] {
		self.r#main_entity_of_page.as_slice()
	}
	fn take_main_entity_of_page(&mut self) -> Vec<MainEntityOfPageProperty> {
		std::mem::take(&mut self.r#main_entity_of_page)
	}
	fn get_name(&self) -> &[NameProperty] {
		self.r#name.as_slice()
	}
	fn take_name(&mut self) -> Vec<NameProperty> {
		std::mem::take(&mut self.r#name)
	}
	fn get_owner(&self) -> &[OwnerProperty] {
		self.r#owner.as_slice()
	}
	fn take_owner(&mut self) -> Vec<OwnerProperty> {
		std::mem::take(&mut self.r#owner)
	}
	fn get_potential_action(&self) -> &[PotentialActionProperty] {
		self.r#potential_action.as_slice()
	}
	fn take_potential_action(&mut self) -> Vec<PotentialActionProperty> {
		std::mem::take(&mut self.r#potential_action)
	}
	fn get_same_as(&self) -> &[SameAsProperty] {
		self.r#same_as.as_slice()
	}
	fn take_same_as(&mut self) -> Vec<SameAsProperty> {
		std::mem::take(&mut self.r#same_as)
	}
	fn get_subject_of(&self) -> &[SubjectOfProperty] {
		self.r#subject_of.as_slice()
	}
	fn take_subject_of(&mut self) -> Vec<SubjectOfProperty> {
		std::mem::take(&mut self.r#subject_of)
	}
	fn get_url(&self) -> &[UrlProperty] {
		self.r#url.as_slice()
	}
	fn take_url(&mut self) -> Vec<UrlProperty> {
		std::mem::take(&mut self.r#url)
	}
}
