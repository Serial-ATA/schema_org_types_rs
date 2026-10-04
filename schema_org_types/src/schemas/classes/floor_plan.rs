use super::*;
/// <https://schema.org/FloorPlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct FloorPlan {
	/// <https://schema.org/amenityFeature>
	#[cfg_attr(feature = "serde", serde(rename = "amenityFeature"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#amenity_feature: Vec<AmenityFeatureProperty>,
	/// <https://schema.org/floorSize>
	#[cfg_attr(feature = "serde", serde(rename = "floorSize"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#floor_size: Vec<FloorSizeProperty>,
	/// <https://schema.org/isPlanForApartment>
	#[cfg_attr(feature = "serde", serde(rename = "isPlanForApartment"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_plan_for_apartment: Vec<IsPlanForApartmentProperty>,
	/// <https://schema.org/layoutImage>
	#[cfg_attr(feature = "serde", serde(rename = "layoutImage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#layout_image: Vec<LayoutImageProperty>,
	/// <https://schema.org/numberOfAccommodationUnits>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfAccommodationUnits"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_accommodation_units: Vec<NumberOfAccommodationUnitsProperty>,
	/// <https://schema.org/numberOfAvailableAccommodationUnits>
	#[cfg_attr(
		feature = "serde",
		serde(rename = "numberOfAvailableAccommodationUnits")
	)]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_available_accommodation_units: Vec<NumberOfAvailableAccommodationUnitsProperty>,
	/// <https://schema.org/numberOfBathroomsTotal>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfBathroomsTotal"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_bathrooms_total: Vec<NumberOfBathroomsTotalProperty>,
	/// <https://schema.org/numberOfBedrooms>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfBedrooms"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_bedrooms: Vec<NumberOfBedroomsProperty>,
	/// <https://schema.org/numberOfFullBathrooms>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfFullBathrooms"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_full_bathrooms: Vec<NumberOfFullBathroomsProperty>,
	/// <https://schema.org/numberOfPartialBathrooms>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfPartialBathrooms"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_partial_bathrooms: Vec<NumberOfPartialBathroomsProperty>,
	/// <https://schema.org/numberOfRooms>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfRooms"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_rooms: Vec<NumberOfRoomsProperty>,
	/// <https://schema.org/petsAllowed>
	#[cfg_attr(feature = "serde", serde(rename = "petsAllowed"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#pets_allowed: Vec<PetsAllowedProperty>,
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
/// This trait is for properties from <https://schema.org/FloorPlan>.
pub trait FloorPlanTrait {
	/// Get <https://schema.org/amenityFeature> from [`Self`] as borrowed slice.
	fn get_amenity_feature(&self) -> &[AmenityFeatureProperty];
	/// Take <https://schema.org/amenityFeature> from [`Self`] as owned vector.
	fn take_amenity_feature(&mut self) -> Vec<AmenityFeatureProperty>;
	/// Get <https://schema.org/floorSize> from [`Self`] as borrowed slice.
	fn get_floor_size(&self) -> &[FloorSizeProperty];
	/// Take <https://schema.org/floorSize> from [`Self`] as owned vector.
	fn take_floor_size(&mut self) -> Vec<FloorSizeProperty>;
	/// Get <https://schema.org/isPlanForApartment> from [`Self`] as borrowed slice.
	fn get_is_plan_for_apartment(&self) -> &[IsPlanForApartmentProperty];
	/// Take <https://schema.org/isPlanForApartment> from [`Self`] as owned vector.
	fn take_is_plan_for_apartment(&mut self) -> Vec<IsPlanForApartmentProperty>;
	/// Get <https://schema.org/layoutImage> from [`Self`] as borrowed slice.
	fn get_layout_image(&self) -> &[LayoutImageProperty];
	/// Take <https://schema.org/layoutImage> from [`Self`] as owned vector.
	fn take_layout_image(&mut self) -> Vec<LayoutImageProperty>;
	/// Get <https://schema.org/numberOfAccommodationUnits> from [`Self`] as borrowed slice.
	fn get_number_of_accommodation_units(&self) -> &[NumberOfAccommodationUnitsProperty];
	/// Take <https://schema.org/numberOfAccommodationUnits> from [`Self`] as owned vector.
	fn take_number_of_accommodation_units(&mut self) -> Vec<NumberOfAccommodationUnitsProperty>;
	/// Get <https://schema.org/numberOfAvailableAccommodationUnits> from [`Self`] as borrowed slice.
	fn get_number_of_available_accommodation_units(
		&self,
	) -> &[NumberOfAvailableAccommodationUnitsProperty];
	/// Take <https://schema.org/numberOfAvailableAccommodationUnits> from [`Self`] as owned vector.
	fn take_number_of_available_accommodation_units(
		&mut self,
	) -> Vec<NumberOfAvailableAccommodationUnitsProperty>;
	/// Get <https://schema.org/numberOfBathroomsTotal> from [`Self`] as borrowed slice.
	fn get_number_of_bathrooms_total(&self) -> &[NumberOfBathroomsTotalProperty];
	/// Take <https://schema.org/numberOfBathroomsTotal> from [`Self`] as owned vector.
	fn take_number_of_bathrooms_total(&mut self) -> Vec<NumberOfBathroomsTotalProperty>;
	/// Get <https://schema.org/numberOfBedrooms> from [`Self`] as borrowed slice.
	fn get_number_of_bedrooms(&self) -> &[NumberOfBedroomsProperty];
	/// Take <https://schema.org/numberOfBedrooms> from [`Self`] as owned vector.
	fn take_number_of_bedrooms(&mut self) -> Vec<NumberOfBedroomsProperty>;
	/// Get <https://schema.org/numberOfFullBathrooms> from [`Self`] as borrowed slice.
	fn get_number_of_full_bathrooms(&self) -> &[NumberOfFullBathroomsProperty];
	/// Take <https://schema.org/numberOfFullBathrooms> from [`Self`] as owned vector.
	fn take_number_of_full_bathrooms(&mut self) -> Vec<NumberOfFullBathroomsProperty>;
	/// Get <https://schema.org/numberOfPartialBathrooms> from [`Self`] as borrowed slice.
	fn get_number_of_partial_bathrooms(&self) -> &[NumberOfPartialBathroomsProperty];
	/// Take <https://schema.org/numberOfPartialBathrooms> from [`Self`] as owned vector.
	fn take_number_of_partial_bathrooms(&mut self) -> Vec<NumberOfPartialBathroomsProperty>;
	/// Get <https://schema.org/numberOfRooms> from [`Self`] as borrowed slice.
	fn get_number_of_rooms(&self) -> &[NumberOfRoomsProperty];
	/// Take <https://schema.org/numberOfRooms> from [`Self`] as owned vector.
	fn take_number_of_rooms(&mut self) -> Vec<NumberOfRoomsProperty>;
	/// Get <https://schema.org/petsAllowed> from [`Self`] as borrowed slice.
	fn get_pets_allowed(&self) -> &[PetsAllowedProperty];
	/// Take <https://schema.org/petsAllowed> from [`Self`] as owned vector.
	fn take_pets_allowed(&mut self) -> Vec<PetsAllowedProperty>;
}
impl FloorPlanTrait for FloorPlan {
	fn get_amenity_feature(&self) -> &[AmenityFeatureProperty] {
		self.r#amenity_feature.as_slice()
	}
	fn take_amenity_feature(&mut self) -> Vec<AmenityFeatureProperty> {
		std::mem::take(&mut self.r#amenity_feature)
	}
	fn get_floor_size(&self) -> &[FloorSizeProperty] {
		self.r#floor_size.as_slice()
	}
	fn take_floor_size(&mut self) -> Vec<FloorSizeProperty> {
		std::mem::take(&mut self.r#floor_size)
	}
	fn get_is_plan_for_apartment(&self) -> &[IsPlanForApartmentProperty] {
		self.r#is_plan_for_apartment.as_slice()
	}
	fn take_is_plan_for_apartment(&mut self) -> Vec<IsPlanForApartmentProperty> {
		std::mem::take(&mut self.r#is_plan_for_apartment)
	}
	fn get_layout_image(&self) -> &[LayoutImageProperty] {
		self.r#layout_image.as_slice()
	}
	fn take_layout_image(&mut self) -> Vec<LayoutImageProperty> {
		std::mem::take(&mut self.r#layout_image)
	}
	fn get_number_of_accommodation_units(&self) -> &[NumberOfAccommodationUnitsProperty] {
		self.r#number_of_accommodation_units.as_slice()
	}
	fn take_number_of_accommodation_units(&mut self) -> Vec<NumberOfAccommodationUnitsProperty> {
		std::mem::take(&mut self.r#number_of_accommodation_units)
	}
	fn get_number_of_available_accommodation_units(
		&self,
	) -> &[NumberOfAvailableAccommodationUnitsProperty] {
		self.r#number_of_available_accommodation_units.as_slice()
	}
	fn take_number_of_available_accommodation_units(
		&mut self,
	) -> Vec<NumberOfAvailableAccommodationUnitsProperty> {
		std::mem::take(&mut self.r#number_of_available_accommodation_units)
	}
	fn get_number_of_bathrooms_total(&self) -> &[NumberOfBathroomsTotalProperty] {
		self.r#number_of_bathrooms_total.as_slice()
	}
	fn take_number_of_bathrooms_total(&mut self) -> Vec<NumberOfBathroomsTotalProperty> {
		std::mem::take(&mut self.r#number_of_bathrooms_total)
	}
	fn get_number_of_bedrooms(&self) -> &[NumberOfBedroomsProperty] {
		self.r#number_of_bedrooms.as_slice()
	}
	fn take_number_of_bedrooms(&mut self) -> Vec<NumberOfBedroomsProperty> {
		std::mem::take(&mut self.r#number_of_bedrooms)
	}
	fn get_number_of_full_bathrooms(&self) -> &[NumberOfFullBathroomsProperty] {
		self.r#number_of_full_bathrooms.as_slice()
	}
	fn take_number_of_full_bathrooms(&mut self) -> Vec<NumberOfFullBathroomsProperty> {
		std::mem::take(&mut self.r#number_of_full_bathrooms)
	}
	fn get_number_of_partial_bathrooms(&self) -> &[NumberOfPartialBathroomsProperty] {
		self.r#number_of_partial_bathrooms.as_slice()
	}
	fn take_number_of_partial_bathrooms(&mut self) -> Vec<NumberOfPartialBathroomsProperty> {
		std::mem::take(&mut self.r#number_of_partial_bathrooms)
	}
	fn get_number_of_rooms(&self) -> &[NumberOfRoomsProperty] {
		self.r#number_of_rooms.as_slice()
	}
	fn take_number_of_rooms(&mut self) -> Vec<NumberOfRoomsProperty> {
		std::mem::take(&mut self.r#number_of_rooms)
	}
	fn get_pets_allowed(&self) -> &[PetsAllowedProperty] {
		self.r#pets_allowed.as_slice()
	}
	fn take_pets_allowed(&mut self) -> Vec<PetsAllowedProperty> {
		std::mem::take(&mut self.r#pets_allowed)
	}
}
impl ThingTrait for FloorPlan {
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
