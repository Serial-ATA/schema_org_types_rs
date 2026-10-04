use super::*;
/// <https://schema.org/PropertyValue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PropertyValue {
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
	/// <https://schema.org/propertyID>
	#[cfg_attr(feature = "serde", serde(rename = "propertyID"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#property_id: Vec<PropertyIdProperty>,
	/// <https://schema.org/unitCode>
	#[cfg_attr(feature = "serde", serde(rename = "unitCode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#unit_code: Vec<UnitCodeProperty>,
	/// <https://schema.org/unitText>
	#[cfg_attr(feature = "serde", serde(rename = "unitText"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#unit_text: Vec<UnitTextProperty>,
	/// <https://schema.org/value>
	#[cfg_attr(feature = "serde", serde(rename = "value"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value: Vec<ValueProperty>,
	/// <https://schema.org/valueGroup>
	#[cfg_attr(feature = "serde", serde(rename = "valueGroup"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_group: Vec<ValueGroupProperty>,
	/// <https://schema.org/valueReference>
	#[cfg_attr(feature = "serde", serde(rename = "valueReference"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#value_reference: Vec<ValueReferenceProperty>,
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
/// This trait is for properties from <https://schema.org/PropertyValue>.
pub trait PropertyValueTrait {
	/// Get <https://schema.org/maxValue> from [`Self`] as borrowed slice.
	fn get_max_value(&self) -> &[MaxValueProperty];
	/// Take <https://schema.org/maxValue> from [`Self`] as owned vector.
	fn take_max_value(&mut self) -> Vec<MaxValueProperty>;
	/// Get <https://schema.org/measurementMethod> from [`Self`] as borrowed slice.
	fn get_measurement_method(&self) -> &[MeasurementMethodProperty];
	/// Take <https://schema.org/measurementMethod> from [`Self`] as owned vector.
	fn take_measurement_method(&mut self) -> Vec<MeasurementMethodProperty>;
	/// Get <https://schema.org/measurementTechnique> from [`Self`] as borrowed slice.
	fn get_measurement_technique(&self) -> &[MeasurementTechniqueProperty];
	/// Take <https://schema.org/measurementTechnique> from [`Self`] as owned vector.
	fn take_measurement_technique(&mut self) -> Vec<MeasurementTechniqueProperty>;
	/// Get <https://schema.org/minValue> from [`Self`] as borrowed slice.
	fn get_min_value(&self) -> &[MinValueProperty];
	/// Take <https://schema.org/minValue> from [`Self`] as owned vector.
	fn take_min_value(&mut self) -> Vec<MinValueProperty>;
	/// Get <https://schema.org/propertyID> from [`Self`] as borrowed slice.
	fn get_property_id(&self) -> &[PropertyIdProperty];
	/// Take <https://schema.org/propertyID> from [`Self`] as owned vector.
	fn take_property_id(&mut self) -> Vec<PropertyIdProperty>;
	/// Get <https://schema.org/unitCode> from [`Self`] as borrowed slice.
	fn get_unit_code(&self) -> &[UnitCodeProperty];
	/// Take <https://schema.org/unitCode> from [`Self`] as owned vector.
	fn take_unit_code(&mut self) -> Vec<UnitCodeProperty>;
	/// Get <https://schema.org/unitText> from [`Self`] as borrowed slice.
	fn get_unit_text(&self) -> &[UnitTextProperty];
	/// Take <https://schema.org/unitText> from [`Self`] as owned vector.
	fn take_unit_text(&mut self) -> Vec<UnitTextProperty>;
	/// Get <https://schema.org/value> from [`Self`] as borrowed slice.
	fn get_value(&self) -> &[ValueProperty];
	/// Take <https://schema.org/value> from [`Self`] as owned vector.
	fn take_value(&mut self) -> Vec<ValueProperty>;
	/// Get <https://schema.org/valueGroup> from [`Self`] as borrowed slice.
	fn get_value_group(&self) -> &[ValueGroupProperty];
	/// Take <https://schema.org/valueGroup> from [`Self`] as owned vector.
	fn take_value_group(&mut self) -> Vec<ValueGroupProperty>;
	/// Get <https://schema.org/valueReference> from [`Self`] as borrowed slice.
	fn get_value_reference(&self) -> &[ValueReferenceProperty];
	/// Take <https://schema.org/valueReference> from [`Self`] as owned vector.
	fn take_value_reference(&mut self) -> Vec<ValueReferenceProperty>;
}
impl PropertyValueTrait for PropertyValue {
	fn get_max_value(&self) -> &[MaxValueProperty] {
		self.r#max_value.as_slice()
	}
	fn take_max_value(&mut self) -> Vec<MaxValueProperty> {
		std::mem::take(&mut self.r#max_value)
	}
	fn get_measurement_method(&self) -> &[MeasurementMethodProperty] {
		self.r#measurement_method.as_slice()
	}
	fn take_measurement_method(&mut self) -> Vec<MeasurementMethodProperty> {
		std::mem::take(&mut self.r#measurement_method)
	}
	fn get_measurement_technique(&self) -> &[MeasurementTechniqueProperty] {
		self.r#measurement_technique.as_slice()
	}
	fn take_measurement_technique(&mut self) -> Vec<MeasurementTechniqueProperty> {
		std::mem::take(&mut self.r#measurement_technique)
	}
	fn get_min_value(&self) -> &[MinValueProperty] {
		self.r#min_value.as_slice()
	}
	fn take_min_value(&mut self) -> Vec<MinValueProperty> {
		std::mem::take(&mut self.r#min_value)
	}
	fn get_property_id(&self) -> &[PropertyIdProperty] {
		self.r#property_id.as_slice()
	}
	fn take_property_id(&mut self) -> Vec<PropertyIdProperty> {
		std::mem::take(&mut self.r#property_id)
	}
	fn get_unit_code(&self) -> &[UnitCodeProperty] {
		self.r#unit_code.as_slice()
	}
	fn take_unit_code(&mut self) -> Vec<UnitCodeProperty> {
		std::mem::take(&mut self.r#unit_code)
	}
	fn get_unit_text(&self) -> &[UnitTextProperty] {
		self.r#unit_text.as_slice()
	}
	fn take_unit_text(&mut self) -> Vec<UnitTextProperty> {
		std::mem::take(&mut self.r#unit_text)
	}
	fn get_value(&self) -> &[ValueProperty] {
		self.r#value.as_slice()
	}
	fn take_value(&mut self) -> Vec<ValueProperty> {
		std::mem::take(&mut self.r#value)
	}
	fn get_value_group(&self) -> &[ValueGroupProperty] {
		self.r#value_group.as_slice()
	}
	fn take_value_group(&mut self) -> Vec<ValueGroupProperty> {
		std::mem::take(&mut self.r#value_group)
	}
	fn get_value_reference(&self) -> &[ValueReferenceProperty] {
		self.r#value_reference.as_slice()
	}
	fn take_value_reference(&mut self) -> Vec<ValueReferenceProperty> {
		std::mem::take(&mut self.r#value_reference)
	}
}
impl StructuredValueTrait for PropertyValue {}
impl ThingTrait for PropertyValue {
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
