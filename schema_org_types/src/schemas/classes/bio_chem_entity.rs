use super::*;
/// <https://schema.org/BioChemEntity>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct BioChemEntity {
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
/// This trait is for properties from <https://schema.org/BioChemEntity>.
pub trait BioChemEntityTrait {
	/// Get <https://schema.org/associatedDisease> from [`Self`] as borrowed slice.
	fn get_associated_disease(&self) -> &[AssociatedDiseaseProperty];
	/// Take <https://schema.org/associatedDisease> from [`Self`] as owned vector.
	fn take_associated_disease(&mut self) -> Vec<AssociatedDiseaseProperty>;
	/// Get <https://schema.org/bioChemInteraction> from [`Self`] as borrowed slice.
	fn get_bio_chem_interaction(&self) -> &[BioChemInteractionProperty];
	/// Take <https://schema.org/bioChemInteraction> from [`Self`] as owned vector.
	fn take_bio_chem_interaction(&mut self) -> Vec<BioChemInteractionProperty>;
	/// Get <https://schema.org/bioChemSimilarity> from [`Self`] as borrowed slice.
	fn get_bio_chem_similarity(&self) -> &[BioChemSimilarityProperty];
	/// Take <https://schema.org/bioChemSimilarity> from [`Self`] as owned vector.
	fn take_bio_chem_similarity(&mut self) -> Vec<BioChemSimilarityProperty>;
	/// Get <https://schema.org/biologicalRole> from [`Self`] as borrowed slice.
	fn get_biological_role(&self) -> &[BiologicalRoleProperty];
	/// Take <https://schema.org/biologicalRole> from [`Self`] as owned vector.
	fn take_biological_role(&mut self) -> Vec<BiologicalRoleProperty>;
	/// Get <https://schema.org/funding> from [`Self`] as borrowed slice.
	fn get_funding(&self) -> &[FundingProperty];
	/// Take <https://schema.org/funding> from [`Self`] as owned vector.
	fn take_funding(&mut self) -> Vec<FundingProperty>;
	/// Get <https://schema.org/hasBioChemEntityPart> from [`Self`] as borrowed slice.
	fn get_has_bio_chem_entity_part(&self) -> &[HasBioChemEntityPartProperty];
	/// Take <https://schema.org/hasBioChemEntityPart> from [`Self`] as owned vector.
	fn take_has_bio_chem_entity_part(&mut self) -> Vec<HasBioChemEntityPartProperty>;
	/// Get <https://schema.org/hasMolecularFunction> from [`Self`] as borrowed slice.
	fn get_has_molecular_function(&self) -> &[HasMolecularFunctionProperty];
	/// Take <https://schema.org/hasMolecularFunction> from [`Self`] as owned vector.
	fn take_has_molecular_function(&mut self) -> Vec<HasMolecularFunctionProperty>;
	/// Get <https://schema.org/hasRepresentation> from [`Self`] as borrowed slice.
	fn get_has_representation(&self) -> &[HasRepresentationProperty];
	/// Take <https://schema.org/hasRepresentation> from [`Self`] as owned vector.
	fn take_has_representation(&mut self) -> Vec<HasRepresentationProperty>;
	/// Get <https://schema.org/isEncodedByBioChemEntity> from [`Self`] as borrowed slice.
	fn get_is_encoded_by_bio_chem_entity(&self) -> &[IsEncodedByBioChemEntityProperty];
	/// Take <https://schema.org/isEncodedByBioChemEntity> from [`Self`] as owned vector.
	fn take_is_encoded_by_bio_chem_entity(&mut self) -> Vec<IsEncodedByBioChemEntityProperty>;
	/// Get <https://schema.org/isInvolvedInBiologicalProcess> from [`Self`] as borrowed slice.
	fn get_is_involved_in_biological_process(&self) -> &[IsInvolvedInBiologicalProcessProperty];
	/// Take <https://schema.org/isInvolvedInBiologicalProcess> from [`Self`] as owned vector.
	fn take_is_involved_in_biological_process(
		&mut self,
	) -> Vec<IsInvolvedInBiologicalProcessProperty>;
	/// Get <https://schema.org/isLocatedInSubcellularLocation> from [`Self`] as borrowed slice.
	fn get_is_located_in_subcellular_location(&self) -> &[IsLocatedInSubcellularLocationProperty];
	/// Take <https://schema.org/isLocatedInSubcellularLocation> from [`Self`] as owned vector.
	fn take_is_located_in_subcellular_location(
		&mut self,
	) -> Vec<IsLocatedInSubcellularLocationProperty>;
	/// Get <https://schema.org/isPartOfBioChemEntity> from [`Self`] as borrowed slice.
	fn get_is_part_of_bio_chem_entity(&self) -> &[IsPartOfBioChemEntityProperty];
	/// Take <https://schema.org/isPartOfBioChemEntity> from [`Self`] as owned vector.
	fn take_is_part_of_bio_chem_entity(&mut self) -> Vec<IsPartOfBioChemEntityProperty>;
	/// Get <https://schema.org/taxonomicRange> from [`Self`] as borrowed slice.
	fn get_taxonomic_range(&self) -> &[TaxonomicRangeProperty];
	/// Take <https://schema.org/taxonomicRange> from [`Self`] as owned vector.
	fn take_taxonomic_range(&mut self) -> Vec<TaxonomicRangeProperty>;
}
impl BioChemEntityTrait for BioChemEntity {
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
impl ThingTrait for BioChemEntity {
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
