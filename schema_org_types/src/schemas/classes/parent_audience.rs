use super::*;
/// <https://schema.org/ParentAudience>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ParentAudience {
	/// <https://schema.org/childMaxAge>
	#[cfg_attr(feature = "serde", serde(rename = "childMaxAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#child_max_age: Vec<ChildMaxAgeProperty>,
	/// <https://schema.org/childMinAge>
	#[cfg_attr(feature = "serde", serde(rename = "childMinAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#child_min_age: Vec<ChildMinAgeProperty>,
	/// <https://schema.org/audienceType>
	#[cfg_attr(feature = "serde", serde(rename = "audienceType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#audience_type: Vec<AudienceTypeProperty>,
	/// <https://schema.org/geographicArea>
	#[cfg_attr(feature = "serde", serde(rename = "geographicArea"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geographic_area: Vec<GeographicAreaProperty>,
	/// <https://schema.org/healthCondition>
	#[cfg_attr(feature = "serde", serde(rename = "healthCondition"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_condition: Vec<HealthConditionProperty>,
	/// <https://schema.org/requiredGender>
	#[cfg_attr(feature = "serde", serde(rename = "requiredGender"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#required_gender: Vec<RequiredGenderProperty>,
	/// <https://schema.org/requiredMaxAge>
	#[cfg_attr(feature = "serde", serde(rename = "requiredMaxAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#required_max_age: Vec<RequiredMaxAgeProperty>,
	/// <https://schema.org/requiredMinAge>
	#[cfg_attr(feature = "serde", serde(rename = "requiredMinAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#required_min_age: Vec<RequiredMinAgeProperty>,
	/// <https://schema.org/suggestedAge>
	#[cfg_attr(feature = "serde", serde(rename = "suggestedAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#suggested_age: Vec<SuggestedAgeProperty>,
	/// <https://schema.org/suggestedGender>
	#[cfg_attr(feature = "serde", serde(rename = "suggestedGender"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#suggested_gender: Vec<SuggestedGenderProperty>,
	/// <https://schema.org/suggestedMaxAge>
	#[cfg_attr(feature = "serde", serde(rename = "suggestedMaxAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#suggested_max_age: Vec<SuggestedMaxAgeProperty>,
	/// <https://schema.org/suggestedMeasurement>
	#[cfg_attr(feature = "serde", serde(rename = "suggestedMeasurement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#suggested_measurement: Vec<SuggestedMeasurementProperty>,
	/// <https://schema.org/suggestedMinAge>
	#[cfg_attr(feature = "serde", serde(rename = "suggestedMinAge"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#suggested_min_age: Vec<SuggestedMinAgeProperty>,
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
/// This trait is for properties from <https://schema.org/ParentAudience>.
pub trait ParentAudienceTrait {
	/// Get <https://schema.org/childMaxAge> from [`Self`] as borrowed slice.
	fn get_child_max_age(&self) -> &[ChildMaxAgeProperty];
	/// Take <https://schema.org/childMaxAge> from [`Self`] as owned vector.
	fn take_child_max_age(&mut self) -> Vec<ChildMaxAgeProperty>;
	/// Get <https://schema.org/childMinAge> from [`Self`] as borrowed slice.
	fn get_child_min_age(&self) -> &[ChildMinAgeProperty];
	/// Take <https://schema.org/childMinAge> from [`Self`] as owned vector.
	fn take_child_min_age(&mut self) -> Vec<ChildMinAgeProperty>;
}
impl ParentAudienceTrait for ParentAudience {
	fn get_child_max_age(&self) -> &[ChildMaxAgeProperty] {
		self.r#child_max_age.as_slice()
	}
	fn take_child_max_age(&mut self) -> Vec<ChildMaxAgeProperty> {
		std::mem::take(&mut self.r#child_max_age)
	}
	fn get_child_min_age(&self) -> &[ChildMinAgeProperty] {
		self.r#child_min_age.as_slice()
	}
	fn take_child_min_age(&mut self) -> Vec<ChildMinAgeProperty> {
		std::mem::take(&mut self.r#child_min_age)
	}
}
impl AudienceTrait for ParentAudience {
	fn get_audience_type(&self) -> &[AudienceTypeProperty] {
		self.r#audience_type.as_slice()
	}
	fn take_audience_type(&mut self) -> Vec<AudienceTypeProperty> {
		std::mem::take(&mut self.r#audience_type)
	}
	fn get_geographic_area(&self) -> &[GeographicAreaProperty] {
		self.r#geographic_area.as_slice()
	}
	fn take_geographic_area(&mut self) -> Vec<GeographicAreaProperty> {
		std::mem::take(&mut self.r#geographic_area)
	}
}
impl PeopleAudienceTrait for ParentAudience {
	fn get_health_condition(&self) -> &[HealthConditionProperty] {
		self.r#health_condition.as_slice()
	}
	fn take_health_condition(&mut self) -> Vec<HealthConditionProperty> {
		std::mem::take(&mut self.r#health_condition)
	}
	fn get_required_gender(&self) -> &[RequiredGenderProperty] {
		self.r#required_gender.as_slice()
	}
	fn take_required_gender(&mut self) -> Vec<RequiredGenderProperty> {
		std::mem::take(&mut self.r#required_gender)
	}
	fn get_required_max_age(&self) -> &[RequiredMaxAgeProperty] {
		self.r#required_max_age.as_slice()
	}
	fn take_required_max_age(&mut self) -> Vec<RequiredMaxAgeProperty> {
		std::mem::take(&mut self.r#required_max_age)
	}
	fn get_required_min_age(&self) -> &[RequiredMinAgeProperty] {
		self.r#required_min_age.as_slice()
	}
	fn take_required_min_age(&mut self) -> Vec<RequiredMinAgeProperty> {
		std::mem::take(&mut self.r#required_min_age)
	}
	fn get_suggested_age(&self) -> &[SuggestedAgeProperty] {
		self.r#suggested_age.as_slice()
	}
	fn take_suggested_age(&mut self) -> Vec<SuggestedAgeProperty> {
		std::mem::take(&mut self.r#suggested_age)
	}
	fn get_suggested_gender(&self) -> &[SuggestedGenderProperty] {
		self.r#suggested_gender.as_slice()
	}
	fn take_suggested_gender(&mut self) -> Vec<SuggestedGenderProperty> {
		std::mem::take(&mut self.r#suggested_gender)
	}
	fn get_suggested_max_age(&self) -> &[SuggestedMaxAgeProperty] {
		self.r#suggested_max_age.as_slice()
	}
	fn take_suggested_max_age(&mut self) -> Vec<SuggestedMaxAgeProperty> {
		std::mem::take(&mut self.r#suggested_max_age)
	}
	fn get_suggested_measurement(&self) -> &[SuggestedMeasurementProperty] {
		self.r#suggested_measurement.as_slice()
	}
	fn take_suggested_measurement(&mut self) -> Vec<SuggestedMeasurementProperty> {
		std::mem::take(&mut self.r#suggested_measurement)
	}
	fn get_suggested_min_age(&self) -> &[SuggestedMinAgeProperty] {
		self.r#suggested_min_age.as_slice()
	}
	fn take_suggested_min_age(&mut self) -> Vec<SuggestedMinAgeProperty> {
		std::mem::take(&mut self.r#suggested_min_age)
	}
}
impl ThingTrait for ParentAudience {
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
