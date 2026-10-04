use super::*;
/// <https://schema.org/StatisticalVariable>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct StatisticalVariable {
	/// <https://schema.org/measuredProperty>
	#[cfg_attr(feature = "serde", serde(rename = "measuredProperty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#measured_property: Vec<MeasuredPropertyProperty>,
	/// <https://schema.org/measurementDenominator>
	#[cfg_attr(feature = "serde", serde(rename = "measurementDenominator"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#measurement_denominator: Vec<MeasurementDenominatorProperty>,
	/// <https://schema.org/measurementMethod>
	#[cfg_attr(feature = "serde", serde(rename = "measurementMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#measurement_method: Vec<MeasurementMethodProperty>,
	/// <https://schema.org/measurementQualifier>
	#[cfg_attr(feature = "serde", serde(rename = "measurementQualifier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#measurement_qualifier: Vec<MeasurementQualifierProperty>,
	/// <https://schema.org/measurementTechnique>
	#[cfg_attr(feature = "serde", serde(rename = "measurementTechnique"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#measurement_technique: Vec<MeasurementTechniqueProperty>,
	/// <https://schema.org/populationType>
	#[cfg_attr(feature = "serde", serde(rename = "populationType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#population_type: Vec<PopulationTypeProperty>,
	/// <https://schema.org/statType>
	#[cfg_attr(feature = "serde", serde(rename = "statType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#stat_type: Vec<StatTypeProperty>,
	/// <https://schema.org/constraintProperty>
	#[cfg_attr(feature = "serde", serde(rename = "constraintProperty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#constraint_property: Vec<ConstraintPropertyProperty>,
	/// <https://schema.org/numConstraints>
	#[cfg_attr(feature = "serde", serde(rename = "numConstraints"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#num_constraints: Vec<NumConstraintsProperty>,
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
/// This trait is for properties from <https://schema.org/StatisticalVariable>.
pub trait StatisticalVariableTrait {
	/// Get <https://schema.org/measuredProperty> from [`Self`] as borrowed slice.
	fn get_measured_property(&self) -> &[MeasuredPropertyProperty];
	/// Take <https://schema.org/measuredProperty> from [`Self`] as owned vector.
	fn take_measured_property(&mut self) -> Vec<MeasuredPropertyProperty>;
	/// Get <https://schema.org/measurementDenominator> from [`Self`] as borrowed slice.
	fn get_measurement_denominator(&self) -> &[MeasurementDenominatorProperty];
	/// Take <https://schema.org/measurementDenominator> from [`Self`] as owned vector.
	fn take_measurement_denominator(&mut self) -> Vec<MeasurementDenominatorProperty>;
	/// Get <https://schema.org/measurementMethod> from [`Self`] as borrowed slice.
	fn get_measurement_method(&self) -> &[MeasurementMethodProperty];
	/// Take <https://schema.org/measurementMethod> from [`Self`] as owned vector.
	fn take_measurement_method(&mut self) -> Vec<MeasurementMethodProperty>;
	/// Get <https://schema.org/measurementQualifier> from [`Self`] as borrowed slice.
	fn get_measurement_qualifier(&self) -> &[MeasurementQualifierProperty];
	/// Take <https://schema.org/measurementQualifier> from [`Self`] as owned vector.
	fn take_measurement_qualifier(&mut self) -> Vec<MeasurementQualifierProperty>;
	/// Get <https://schema.org/measurementTechnique> from [`Self`] as borrowed slice.
	fn get_measurement_technique(&self) -> &[MeasurementTechniqueProperty];
	/// Take <https://schema.org/measurementTechnique> from [`Self`] as owned vector.
	fn take_measurement_technique(&mut self) -> Vec<MeasurementTechniqueProperty>;
	/// Get <https://schema.org/populationType> from [`Self`] as borrowed slice.
	fn get_population_type(&self) -> &[PopulationTypeProperty];
	/// Take <https://schema.org/populationType> from [`Self`] as owned vector.
	fn take_population_type(&mut self) -> Vec<PopulationTypeProperty>;
	/// Get <https://schema.org/statType> from [`Self`] as borrowed slice.
	fn get_stat_type(&self) -> &[StatTypeProperty];
	/// Take <https://schema.org/statType> from [`Self`] as owned vector.
	fn take_stat_type(&mut self) -> Vec<StatTypeProperty>;
}
impl StatisticalVariableTrait for StatisticalVariable {
	fn get_measured_property(&self) -> &[MeasuredPropertyProperty] {
		self.r#measured_property.as_slice()
	}
	fn take_measured_property(&mut self) -> Vec<MeasuredPropertyProperty> {
		std::mem::take(&mut self.r#measured_property)
	}
	fn get_measurement_denominator(&self) -> &[MeasurementDenominatorProperty] {
		self.r#measurement_denominator.as_slice()
	}
	fn take_measurement_denominator(&mut self) -> Vec<MeasurementDenominatorProperty> {
		std::mem::take(&mut self.r#measurement_denominator)
	}
	fn get_measurement_method(&self) -> &[MeasurementMethodProperty] {
		self.r#measurement_method.as_slice()
	}
	fn take_measurement_method(&mut self) -> Vec<MeasurementMethodProperty> {
		std::mem::take(&mut self.r#measurement_method)
	}
	fn get_measurement_qualifier(&self) -> &[MeasurementQualifierProperty] {
		self.r#measurement_qualifier.as_slice()
	}
	fn take_measurement_qualifier(&mut self) -> Vec<MeasurementQualifierProperty> {
		std::mem::take(&mut self.r#measurement_qualifier)
	}
	fn get_measurement_technique(&self) -> &[MeasurementTechniqueProperty] {
		self.r#measurement_technique.as_slice()
	}
	fn take_measurement_technique(&mut self) -> Vec<MeasurementTechniqueProperty> {
		std::mem::take(&mut self.r#measurement_technique)
	}
	fn get_population_type(&self) -> &[PopulationTypeProperty] {
		self.r#population_type.as_slice()
	}
	fn take_population_type(&mut self) -> Vec<PopulationTypeProperty> {
		std::mem::take(&mut self.r#population_type)
	}
	fn get_stat_type(&self) -> &[StatTypeProperty] {
		self.r#stat_type.as_slice()
	}
	fn take_stat_type(&mut self) -> Vec<StatTypeProperty> {
		std::mem::take(&mut self.r#stat_type)
	}
}
impl ConstraintNodeTrait for StatisticalVariable {
	fn get_constraint_property(&self) -> &[ConstraintPropertyProperty] {
		self.r#constraint_property.as_slice()
	}
	fn take_constraint_property(&mut self) -> Vec<ConstraintPropertyProperty> {
		std::mem::take(&mut self.r#constraint_property)
	}
	fn get_num_constraints(&self) -> &[NumConstraintsProperty] {
		self.r#num_constraints.as_slice()
	}
	fn take_num_constraints(&mut self) -> Vec<NumConstraintsProperty> {
		std::mem::take(&mut self.r#num_constraints)
	}
}
impl ThingTrait for StatisticalVariable {
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
