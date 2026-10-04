use super::*;
/// <https://schema.org/PropertyValueSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PropertyValueSpecification {
	/// <https://schema.org/defaultValue>
	#[cfg_attr(feature = "serde", serde(rename = "defaultValue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#default_value: Vec<DefaultValueProperty>,
	/// <https://schema.org/maxValue>
	#[cfg_attr(feature = "serde", serde(rename = "maxValue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#max_value: Vec<MaxValueProperty>,
	/// <https://schema.org/minValue>
	#[cfg_attr(feature = "serde", serde(rename = "minValue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#min_value: Vec<MinValueProperty>,
	/// <https://schema.org/multipleValues>
	#[cfg_attr(feature = "serde", serde(rename = "multipleValues"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#multiple_values: Vec<MultipleValuesProperty>,
	/// <https://schema.org/readonlyValue>
	#[cfg_attr(feature = "serde", serde(rename = "readonlyValue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#readonly_value: Vec<ReadonlyValueProperty>,
	/// <https://schema.org/stepValue>
	#[cfg_attr(feature = "serde", serde(rename = "stepValue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#step_value: Vec<StepValueProperty>,
	/// <https://schema.org/valueMaxLength>
	#[cfg_attr(feature = "serde", serde(rename = "valueMaxLength"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_max_length: Vec<ValueMaxLengthProperty>,
	/// <https://schema.org/valueMinLength>
	#[cfg_attr(feature = "serde", serde(rename = "valueMinLength"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_min_length: Vec<ValueMinLengthProperty>,
	/// <https://schema.org/valueName>
	#[cfg_attr(feature = "serde", serde(rename = "valueName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_name: Vec<ValueNameProperty>,
	/// <https://schema.org/valuePattern>
	#[cfg_attr(feature = "serde", serde(rename = "valuePattern"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_pattern: Vec<ValuePatternProperty>,
	/// <https://schema.org/valueRequired>
	#[cfg_attr(feature = "serde", serde(rename = "valueRequired"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_required: Vec<ValueRequiredProperty>,
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
/// This trait is for properties from <https://schema.org/PropertyValueSpecification>.
pub trait PropertyValueSpecificationTrait {
	/// Get <https://schema.org/defaultValue> from [`Self`] as borrowed slice.
	fn get_default_value(&self) -> &[DefaultValueProperty];
	/// Take <https://schema.org/defaultValue> from [`Self`] as owned vector.
	fn take_default_value(&mut self) -> Vec<DefaultValueProperty>;
	/// Get <https://schema.org/maxValue> from [`Self`] as borrowed slice.
	fn get_max_value(&self) -> &[MaxValueProperty];
	/// Take <https://schema.org/maxValue> from [`Self`] as owned vector.
	fn take_max_value(&mut self) -> Vec<MaxValueProperty>;
	/// Get <https://schema.org/minValue> from [`Self`] as borrowed slice.
	fn get_min_value(&self) -> &[MinValueProperty];
	/// Take <https://schema.org/minValue> from [`Self`] as owned vector.
	fn take_min_value(&mut self) -> Vec<MinValueProperty>;
	/// Get <https://schema.org/multipleValues> from [`Self`] as borrowed slice.
	fn get_multiple_values(&self) -> &[MultipleValuesProperty];
	/// Take <https://schema.org/multipleValues> from [`Self`] as owned vector.
	fn take_multiple_values(&mut self) -> Vec<MultipleValuesProperty>;
	/// Get <https://schema.org/readonlyValue> from [`Self`] as borrowed slice.
	fn get_readonly_value(&self) -> &[ReadonlyValueProperty];
	/// Take <https://schema.org/readonlyValue> from [`Self`] as owned vector.
	fn take_readonly_value(&mut self) -> Vec<ReadonlyValueProperty>;
	/// Get <https://schema.org/stepValue> from [`Self`] as borrowed slice.
	fn get_step_value(&self) -> &[StepValueProperty];
	/// Take <https://schema.org/stepValue> from [`Self`] as owned vector.
	fn take_step_value(&mut self) -> Vec<StepValueProperty>;
	/// Get <https://schema.org/valueMaxLength> from [`Self`] as borrowed slice.
	fn get_value_max_length(&self) -> &[ValueMaxLengthProperty];
	/// Take <https://schema.org/valueMaxLength> from [`Self`] as owned vector.
	fn take_value_max_length(&mut self) -> Vec<ValueMaxLengthProperty>;
	/// Get <https://schema.org/valueMinLength> from [`Self`] as borrowed slice.
	fn get_value_min_length(&self) -> &[ValueMinLengthProperty];
	/// Take <https://schema.org/valueMinLength> from [`Self`] as owned vector.
	fn take_value_min_length(&mut self) -> Vec<ValueMinLengthProperty>;
	/// Get <https://schema.org/valueName> from [`Self`] as borrowed slice.
	fn get_value_name(&self) -> &[ValueNameProperty];
	/// Take <https://schema.org/valueName> from [`Self`] as owned vector.
	fn take_value_name(&mut self) -> Vec<ValueNameProperty>;
	/// Get <https://schema.org/valuePattern> from [`Self`] as borrowed slice.
	fn get_value_pattern(&self) -> &[ValuePatternProperty];
	/// Take <https://schema.org/valuePattern> from [`Self`] as owned vector.
	fn take_value_pattern(&mut self) -> Vec<ValuePatternProperty>;
	/// Get <https://schema.org/valueRequired> from [`Self`] as borrowed slice.
	fn get_value_required(&self) -> &[ValueRequiredProperty];
	/// Take <https://schema.org/valueRequired> from [`Self`] as owned vector.
	fn take_value_required(&mut self) -> Vec<ValueRequiredProperty>;
}
impl PropertyValueSpecificationTrait for PropertyValueSpecification {
	fn get_default_value(&self) -> &[DefaultValueProperty] {
		self.r#default_value.as_slice()
	}
	fn take_default_value(&mut self) -> Vec<DefaultValueProperty> {
		std::mem::take(&mut self.r#default_value)
	}
	fn get_max_value(&self) -> &[MaxValueProperty] {
		self.r#max_value.as_slice()
	}
	fn take_max_value(&mut self) -> Vec<MaxValueProperty> {
		std::mem::take(&mut self.r#max_value)
	}
	fn get_min_value(&self) -> &[MinValueProperty] {
		self.r#min_value.as_slice()
	}
	fn take_min_value(&mut self) -> Vec<MinValueProperty> {
		std::mem::take(&mut self.r#min_value)
	}
	fn get_multiple_values(&self) -> &[MultipleValuesProperty] {
		self.r#multiple_values.as_slice()
	}
	fn take_multiple_values(&mut self) -> Vec<MultipleValuesProperty> {
		std::mem::take(&mut self.r#multiple_values)
	}
	fn get_readonly_value(&self) -> &[ReadonlyValueProperty] {
		self.r#readonly_value.as_slice()
	}
	fn take_readonly_value(&mut self) -> Vec<ReadonlyValueProperty> {
		std::mem::take(&mut self.r#readonly_value)
	}
	fn get_step_value(&self) -> &[StepValueProperty] {
		self.r#step_value.as_slice()
	}
	fn take_step_value(&mut self) -> Vec<StepValueProperty> {
		std::mem::take(&mut self.r#step_value)
	}
	fn get_value_max_length(&self) -> &[ValueMaxLengthProperty] {
		self.r#value_max_length.as_slice()
	}
	fn take_value_max_length(&mut self) -> Vec<ValueMaxLengthProperty> {
		std::mem::take(&mut self.r#value_max_length)
	}
	fn get_value_min_length(&self) -> &[ValueMinLengthProperty] {
		self.r#value_min_length.as_slice()
	}
	fn take_value_min_length(&mut self) -> Vec<ValueMinLengthProperty> {
		std::mem::take(&mut self.r#value_min_length)
	}
	fn get_value_name(&self) -> &[ValueNameProperty] {
		self.r#value_name.as_slice()
	}
	fn take_value_name(&mut self) -> Vec<ValueNameProperty> {
		std::mem::take(&mut self.r#value_name)
	}
	fn get_value_pattern(&self) -> &[ValuePatternProperty] {
		self.r#value_pattern.as_slice()
	}
	fn take_value_pattern(&mut self) -> Vec<ValuePatternProperty> {
		std::mem::take(&mut self.r#value_pattern)
	}
	fn get_value_required(&self) -> &[ValueRequiredProperty] {
		self.r#value_required.as_slice()
	}
	fn take_value_required(&mut self) -> Vec<ValueRequiredProperty> {
		std::mem::take(&mut self.r#value_required)
	}
}
impl ThingTrait for PropertyValueSpecification {
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
