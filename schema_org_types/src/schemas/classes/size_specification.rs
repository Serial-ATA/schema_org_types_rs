use super::*;
/// <https://schema.org/SizeSpecification>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct SizeSpecification {
	/// <https://schema.org/hasMeasurement>
	#[cfg_attr(feature = "serde", serde(rename = "hasMeasurement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_measurement: Vec<HasMeasurementProperty>,
	/// <https://schema.org/sizeGroup>
	#[cfg_attr(feature = "serde", serde(rename = "sizeGroup"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#size_group: Vec<SizeGroupProperty>,
	/// <https://schema.org/sizeSystem>
	#[cfg_attr(feature = "serde", serde(rename = "sizeSystem"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#size_system: Vec<SizeSystemProperty>,
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
	/// <https://schema.org/supersededBy>
	#[cfg_attr(feature = "serde", serde(rename = "supersededBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#superseded_by: Vec<SupersededByProperty>,
	/// <https://schema.org/additionalProperty>
	#[cfg_attr(feature = "serde", serde(rename = "additionalProperty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	/// <https://schema.org/equal>
	#[cfg_attr(feature = "serde", serde(rename = "equal"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#equal: Vec<EqualProperty>,
	/// <https://schema.org/greater>
	#[cfg_attr(feature = "serde", serde(rename = "greater"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#greater: Vec<GreaterProperty>,
	/// <https://schema.org/greaterOrEqual>
	#[cfg_attr(feature = "serde", serde(rename = "greaterOrEqual"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#greater_or_equal: Vec<GreaterOrEqualProperty>,
	/// <https://schema.org/lesser>
	#[cfg_attr(feature = "serde", serde(rename = "lesser"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#lesser: Vec<LesserProperty>,
	/// <https://schema.org/lesserOrEqual>
	#[cfg_attr(feature = "serde", serde(rename = "lesserOrEqual"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#lesser_or_equal: Vec<LesserOrEqualProperty>,
	/// <https://schema.org/nonEqual>
	#[cfg_attr(feature = "serde", serde(rename = "nonEqual"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#non_equal: Vec<NonEqualProperty>,
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
/// This trait is for properties from <https://schema.org/SizeSpecification>.
pub trait SizeSpecificationTrait {
	/// Get <https://schema.org/hasMeasurement> from [`Self`] as borrowed slice.
	fn r#has_measurement(&self) -> &[HasMeasurementProperty];
	/// Get <https://schema.org/sizeGroup> from [`Self`] as borrowed slice.
	fn r#size_group(&self) -> &[SizeGroupProperty];
	/// Get <https://schema.org/sizeSystem> from [`Self`] as borrowed slice.
	fn r#size_system(&self) -> &[SizeSystemProperty];
	/// Get <https://schema.org/suggestedAge> from [`Self`] as borrowed slice.
	fn r#suggested_age(&self) -> &[SuggestedAgeProperty];
	/// Get <https://schema.org/suggestedGender> from [`Self`] as borrowed slice.
	fn r#suggested_gender(&self) -> &[SuggestedGenderProperty];
	/// Get <https://schema.org/suggestedMeasurement> from [`Self`] as borrowed slice.
	fn r#suggested_measurement(&self) -> &[SuggestedMeasurementProperty];
}
impl SizeSpecificationTrait for SizeSpecification {
	fn r#has_measurement(&self) -> &[HasMeasurementProperty] {
		self.r#has_measurement.as_slice()
	}
	fn r#size_group(&self) -> &[SizeGroupProperty] {
		self.r#size_group.as_slice()
	}
	fn r#size_system(&self) -> &[SizeSystemProperty] {
		self.r#size_system.as_slice()
	}
	fn r#suggested_age(&self) -> &[SuggestedAgeProperty] {
		self.r#suggested_age.as_slice()
	}
	fn r#suggested_gender(&self) -> &[SuggestedGenderProperty] {
		self.r#suggested_gender.as_slice()
	}
	fn r#suggested_measurement(&self) -> &[SuggestedMeasurementProperty] {
		self.r#suggested_measurement.as_slice()
	}
}
impl EnumerationTrait for SizeSpecification {
	fn r#superseded_by(&self) -> &[SupersededByProperty] {
		self.r#superseded_by.as_slice()
	}
}
impl QualitativeValueTrait for SizeSpecification {
	fn r#additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn r#equal(&self) -> &[EqualProperty] {
		self.r#equal.as_slice()
	}
	fn r#greater(&self) -> &[GreaterProperty] {
		self.r#greater.as_slice()
	}
	fn r#greater_or_equal(&self) -> &[GreaterOrEqualProperty] {
		self.r#greater_or_equal.as_slice()
	}
	fn r#lesser(&self) -> &[LesserProperty] {
		self.r#lesser.as_slice()
	}
	fn r#lesser_or_equal(&self) -> &[LesserOrEqualProperty] {
		self.r#lesser_or_equal.as_slice()
	}
	fn r#non_equal(&self) -> &[NonEqualProperty] {
		self.r#non_equal.as_slice()
	}
	fn r#value_reference(&self) -> &[ValueReferenceProperty] {
		self.r#value_reference.as_slice()
	}
}
impl ThingTrait for SizeSpecification {
	fn r#additional_type(&self) -> &[AdditionalTypeProperty] {
		self.r#additional_type.as_slice()
	}
	fn r#alternate_name(&self) -> &[AlternateNameProperty] {
		self.r#alternate_name.as_slice()
	}
	fn r#description(&self) -> &[DescriptionProperty] {
		self.r#description.as_slice()
	}
	fn r#disambiguating_description(&self) -> &[DisambiguatingDescriptionProperty] {
		self.r#disambiguating_description.as_slice()
	}
	fn r#identifier(&self) -> &[IdentifierProperty] {
		self.r#identifier.as_slice()
	}
	fn r#image(&self) -> &[ImageProperty] {
		self.r#image.as_slice()
	}
	fn r#main_entity_of_page(&self) -> &[MainEntityOfPageProperty] {
		self.r#main_entity_of_page.as_slice()
	}
	fn r#name(&self) -> &[NameProperty] {
		self.r#name.as_slice()
	}
	fn r#owner(&self) -> &[OwnerProperty] {
		self.r#owner.as_slice()
	}
	fn r#potential_action(&self) -> &[PotentialActionProperty] {
		self.r#potential_action.as_slice()
	}
	fn r#same_as(&self) -> &[SameAsProperty] {
		self.r#same_as.as_slice()
	}
	fn r#subject_of(&self) -> &[SubjectOfProperty] {
		self.r#subject_of.as_slice()
	}
	fn r#url(&self) -> &[UrlProperty] {
		self.r#url.as_slice()
	}
}
