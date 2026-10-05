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
	fn r#amenity_feature(&self) -> &[AmenityFeatureProperty];
	/// Get <https://schema.org/floorSize> from [`Self`] as borrowed slice.
	fn r#floor_size(&self) -> &[FloorSizeProperty];
	/// Get <https://schema.org/isPlanForApartment> from [`Self`] as borrowed slice.
	fn r#is_plan_for_apartment(&self) -> &[IsPlanForApartmentProperty];
	/// Get <https://schema.org/layoutImage> from [`Self`] as borrowed slice.
	fn r#layout_image(&self) -> &[LayoutImageProperty];
	/// Get <https://schema.org/numberOfAccommodationUnits> from [`Self`] as borrowed slice.
	fn r#number_of_accommodation_units(&self) -> &[NumberOfAccommodationUnitsProperty];
	/// Get <https://schema.org/numberOfAvailableAccommodationUnits> from [`Self`] as borrowed slice.
	fn r#number_of_available_accommodation_units(
		&self,
	) -> &[NumberOfAvailableAccommodationUnitsProperty];
	/// Get <https://schema.org/numberOfBathroomsTotal> from [`Self`] as borrowed slice.
	fn r#number_of_bathrooms_total(&self) -> &[NumberOfBathroomsTotalProperty];
	/// Get <https://schema.org/numberOfBedrooms> from [`Self`] as borrowed slice.
	fn r#number_of_bedrooms(&self) -> &[NumberOfBedroomsProperty];
	/// Get <https://schema.org/numberOfFullBathrooms> from [`Self`] as borrowed slice.
	fn r#number_of_full_bathrooms(&self) -> &[NumberOfFullBathroomsProperty];
	/// Get <https://schema.org/numberOfPartialBathrooms> from [`Self`] as borrowed slice.
	fn r#number_of_partial_bathrooms(&self) -> &[NumberOfPartialBathroomsProperty];
	/// Get <https://schema.org/numberOfRooms> from [`Self`] as borrowed slice.
	fn r#number_of_rooms(&self) -> &[NumberOfRoomsProperty];
	/// Get <https://schema.org/petsAllowed> from [`Self`] as borrowed slice.
	fn r#pets_allowed(&self) -> &[PetsAllowedProperty];
}
impl FloorPlanTrait for FloorPlan {
	fn r#amenity_feature(&self) -> &[AmenityFeatureProperty] {
		self.r#amenity_feature.as_slice()
	}
	fn r#floor_size(&self) -> &[FloorSizeProperty] {
		self.r#floor_size.as_slice()
	}
	fn r#is_plan_for_apartment(&self) -> &[IsPlanForApartmentProperty] {
		self.r#is_plan_for_apartment.as_slice()
	}
	fn r#layout_image(&self) -> &[LayoutImageProperty] {
		self.r#layout_image.as_slice()
	}
	fn r#number_of_accommodation_units(&self) -> &[NumberOfAccommodationUnitsProperty] {
		self.r#number_of_accommodation_units.as_slice()
	}
	fn r#number_of_available_accommodation_units(
		&self,
	) -> &[NumberOfAvailableAccommodationUnitsProperty] {
		self.r#number_of_available_accommodation_units.as_slice()
	}
	fn r#number_of_bathrooms_total(&self) -> &[NumberOfBathroomsTotalProperty] {
		self.r#number_of_bathrooms_total.as_slice()
	}
	fn r#number_of_bedrooms(&self) -> &[NumberOfBedroomsProperty] {
		self.r#number_of_bedrooms.as_slice()
	}
	fn r#number_of_full_bathrooms(&self) -> &[NumberOfFullBathroomsProperty] {
		self.r#number_of_full_bathrooms.as_slice()
	}
	fn r#number_of_partial_bathrooms(&self) -> &[NumberOfPartialBathroomsProperty] {
		self.r#number_of_partial_bathrooms.as_slice()
	}
	fn r#number_of_rooms(&self) -> &[NumberOfRoomsProperty] {
		self.r#number_of_rooms.as_slice()
	}
	fn r#pets_allowed(&self) -> &[PetsAllowedProperty] {
		self.r#pets_allowed.as_slice()
	}
}
impl ThingTrait for FloorPlan {
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
