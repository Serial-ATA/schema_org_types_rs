use super::*;
/// <https://schema.org/BusStation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct BusStation {
	/// <https://schema.org/openingHours>
	#[cfg_attr(feature = "serde", serde(rename = "openingHours"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#opening_hours: Vec<OpeningHoursProperty>,
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
	/// <https://schema.org/address>
	#[cfg_attr(feature = "serde", serde(rename = "address"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#address: Vec<AddressProperty>,
	/// <https://schema.org/aggregateRating>
	#[cfg_attr(feature = "serde", serde(rename = "aggregateRating"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
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
	/// <https://schema.org/branchCode>
	#[cfg_attr(feature = "serde", serde(rename = "branchCode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#branch_code: Vec<BranchCodeProperty>,
	/// <https://schema.org/containedIn>
	#[deprecated = "This schema is superseded by <https://schema.org/containedInPlace>."]
	#[cfg_attr(feature = "serde", serde(rename = "containedIn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contained_in: Vec<ContainedInProperty>,
	/// <https://schema.org/containedInPlace>
	#[cfg_attr(feature = "serde", serde(rename = "containedInPlace"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contained_in_place: Vec<ContainedInPlaceProperty>,
	/// <https://schema.org/containsPlace>
	#[cfg_attr(feature = "serde", serde(rename = "containsPlace"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contains_place: Vec<ContainsPlaceProperty>,
	/// <https://schema.org/event>
	#[cfg_attr(feature = "serde", serde(rename = "event"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#event: Vec<EventProperty>,
	/// <https://schema.org/events>
	#[deprecated = "This schema is superseded by <https://schema.org/event>."]
	#[cfg_attr(feature = "serde", serde(rename = "events"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#events: Vec<EventsProperty>,
	/// <https://schema.org/faxNumber>
	#[cfg_attr(feature = "serde", serde(rename = "faxNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#fax_number: Vec<FaxNumberProperty>,
	/// <https://schema.org/geo>
	#[cfg_attr(feature = "serde", serde(rename = "geo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo: Vec<GeoProperty>,
	/// <https://schema.org/geoContains>
	#[cfg_attr(feature = "serde", serde(rename = "geoContains"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_contains: Vec<GeoContainsProperty>,
	/// <https://schema.org/geoCoveredBy>
	#[cfg_attr(feature = "serde", serde(rename = "geoCoveredBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_covered_by: Vec<GeoCoveredByProperty>,
	/// <https://schema.org/geoCovers>
	#[cfg_attr(feature = "serde", serde(rename = "geoCovers"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_covers: Vec<GeoCoversProperty>,
	/// <https://schema.org/geoCrosses>
	#[cfg_attr(feature = "serde", serde(rename = "geoCrosses"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_crosses: Vec<GeoCrossesProperty>,
	/// <https://schema.org/geoDisjoint>
	#[cfg_attr(feature = "serde", serde(rename = "geoDisjoint"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_disjoint: Vec<GeoDisjointProperty>,
	/// <https://schema.org/geoEquals>
	#[cfg_attr(feature = "serde", serde(rename = "geoEquals"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_equals: Vec<GeoEqualsProperty>,
	/// <https://schema.org/geoIntersects>
	#[cfg_attr(feature = "serde", serde(rename = "geoIntersects"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_intersects: Vec<GeoIntersectsProperty>,
	/// <https://schema.org/geoOverlaps>
	#[cfg_attr(feature = "serde", serde(rename = "geoOverlaps"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_overlaps: Vec<GeoOverlapsProperty>,
	/// <https://schema.org/geoTouches>
	#[cfg_attr(feature = "serde", serde(rename = "geoTouches"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_touches: Vec<GeoTouchesProperty>,
	/// <https://schema.org/geoWithin>
	#[cfg_attr(feature = "serde", serde(rename = "geoWithin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_within: Vec<GeoWithinProperty>,
	/// <https://schema.org/globalLocationNumber>
	#[cfg_attr(feature = "serde", serde(rename = "globalLocationNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
	/// <https://schema.org/hasCertification>
	#[cfg_attr(feature = "serde", serde(rename = "hasCertification"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_certification: Vec<HasCertificationProperty>,
	/// <https://schema.org/hasDriveThroughService>
	#[cfg_attr(feature = "serde", serde(rename = "hasDriveThroughService"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_drive_through_service: Vec<HasDriveThroughServiceProperty>,
	/// <https://schema.org/hasGS1DigitalLink>
	#[cfg_attr(feature = "serde", serde(rename = "hasGS1DigitalLink"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_gs_1_digital_link: Vec<HasGs1DigitalLinkProperty>,
	/// <https://schema.org/hasMap>
	#[cfg_attr(feature = "serde", serde(rename = "hasMap"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_map: Vec<HasMapProperty>,
	/// <https://schema.org/isAccessibleForFree>
	#[cfg_attr(feature = "serde", serde(rename = "isAccessibleForFree"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	/// <https://schema.org/isicV4>
	#[cfg_attr(feature = "serde", serde(rename = "isicV4"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#isic_v_4: Vec<IsicV4Property>,
	/// <https://schema.org/keywords>
	#[cfg_attr(feature = "serde", serde(rename = "keywords"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#keywords: Vec<KeywordsProperty>,
	/// <https://schema.org/latitude>
	#[cfg_attr(feature = "serde", serde(rename = "latitude"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#latitude: Vec<LatitudeProperty>,
	/// <https://schema.org/logo>
	#[cfg_attr(feature = "serde", serde(rename = "logo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#logo: Vec<LogoProperty>,
	/// <https://schema.org/longitude>
	#[cfg_attr(feature = "serde", serde(rename = "longitude"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#longitude: Vec<LongitudeProperty>,
	/// <https://schema.org/map>
	#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
	#[cfg_attr(feature = "serde", serde(rename = "map"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#map: Vec<MapProperty>,
	/// <https://schema.org/maps>
	#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
	#[cfg_attr(feature = "serde", serde(rename = "maps"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#maps: Vec<MapsProperty>,
	/// <https://schema.org/maximumAttendeeCapacity>
	#[cfg_attr(feature = "serde", serde(rename = "maximumAttendeeCapacity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#maximum_attendee_capacity: Vec<MaximumAttendeeCapacityProperty>,
	/// <https://schema.org/openingHoursSpecification>
	#[cfg_attr(feature = "serde", serde(rename = "openingHoursSpecification"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#opening_hours_specification: Vec<OpeningHoursSpecificationProperty>,
	/// <https://schema.org/photo>
	#[cfg_attr(feature = "serde", serde(rename = "photo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#photo: Vec<PhotoProperty>,
	/// <https://schema.org/photos>
	#[deprecated = "This schema is superseded by <https://schema.org/photo>."]
	#[cfg_attr(feature = "serde", serde(rename = "photos"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#photos: Vec<PhotosProperty>,
	/// <https://schema.org/publicAccess>
	#[cfg_attr(feature = "serde", serde(rename = "publicAccess"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#public_access: Vec<PublicAccessProperty>,
	/// <https://schema.org/review>
	#[cfg_attr(feature = "serde", serde(rename = "review"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#review: Vec<ReviewProperty>,
	/// <https://schema.org/reviews>
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	#[cfg_attr(feature = "serde", serde(rename = "reviews"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#reviews: Vec<ReviewsProperty>,
	/// <https://schema.org/slogan>
	#[cfg_attr(feature = "serde", serde(rename = "slogan"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#slogan: Vec<SloganProperty>,
	/// <https://schema.org/smokingAllowed>
	#[cfg_attr(feature = "serde", serde(rename = "smokingAllowed"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#smoking_allowed: Vec<SmokingAllowedProperty>,
	/// <https://schema.org/specialOpeningHoursSpecification>
	#[cfg_attr(feature = "serde", serde(rename = "specialOpeningHoursSpecification"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#special_opening_hours_specification: Vec<SpecialOpeningHoursSpecificationProperty>,
	/// <https://schema.org/telephone>
	#[cfg_attr(feature = "serde", serde(rename = "telephone"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#telephone: Vec<TelephoneProperty>,
	/// <https://schema.org/tourBookingPage>
	#[cfg_attr(feature = "serde", serde(rename = "tourBookingPage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#tour_booking_page: Vec<TourBookingPageProperty>,
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
/// This trait is for properties from <https://schema.org/BusStation>.
pub trait BusStationTrait {}
impl BusStationTrait for BusStation {}
impl CivicStructureTrait for BusStation {
	fn r#opening_hours(&self) -> &[OpeningHoursProperty] {
		self.r#opening_hours.as_slice()
	}
}
impl PlaceTrait for BusStation {
	fn r#additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn r#address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn r#aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn r#amenity_feature(&self) -> &[AmenityFeatureProperty] {
		self.r#amenity_feature.as_slice()
	}
	fn r#branch_code(&self) -> &[BranchCodeProperty] {
		self.r#branch_code.as_slice()
	}
	fn r#contained_in(&self) -> &[ContainedInProperty] {
		self.r#contained_in.as_slice()
	}
	fn r#contained_in_place(&self) -> &[ContainedInPlaceProperty] {
		self.r#contained_in_place.as_slice()
	}
	fn r#contains_place(&self) -> &[ContainsPlaceProperty] {
		self.r#contains_place.as_slice()
	}
	fn r#event(&self) -> &[EventProperty] {
		self.r#event.as_slice()
	}
	fn r#events(&self) -> &[EventsProperty] {
		self.r#events.as_slice()
	}
	fn r#fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn r#geo(&self) -> &[GeoProperty] {
		self.r#geo.as_slice()
	}
	fn r#geo_contains(&self) -> &[GeoContainsProperty] {
		self.r#geo_contains.as_slice()
	}
	fn r#geo_covered_by(&self) -> &[GeoCoveredByProperty] {
		self.r#geo_covered_by.as_slice()
	}
	fn r#geo_covers(&self) -> &[GeoCoversProperty] {
		self.r#geo_covers.as_slice()
	}
	fn r#geo_crosses(&self) -> &[GeoCrossesProperty] {
		self.r#geo_crosses.as_slice()
	}
	fn r#geo_disjoint(&self) -> &[GeoDisjointProperty] {
		self.r#geo_disjoint.as_slice()
	}
	fn r#geo_equals(&self) -> &[GeoEqualsProperty] {
		self.r#geo_equals.as_slice()
	}
	fn r#geo_intersects(&self) -> &[GeoIntersectsProperty] {
		self.r#geo_intersects.as_slice()
	}
	fn r#geo_overlaps(&self) -> &[GeoOverlapsProperty] {
		self.r#geo_overlaps.as_slice()
	}
	fn r#geo_touches(&self) -> &[GeoTouchesProperty] {
		self.r#geo_touches.as_slice()
	}
	fn r#geo_within(&self) -> &[GeoWithinProperty] {
		self.r#geo_within.as_slice()
	}
	fn r#global_location_number(&self) -> &[GlobalLocationNumberProperty] {
		self.r#global_location_number.as_slice()
	}
	fn r#has_certification(&self) -> &[HasCertificationProperty] {
		self.r#has_certification.as_slice()
	}
	fn r#has_drive_through_service(&self) -> &[HasDriveThroughServiceProperty] {
		self.r#has_drive_through_service.as_slice()
	}
	fn r#has_gs_1_digital_link(&self) -> &[HasGs1DigitalLinkProperty] {
		self.r#has_gs_1_digital_link.as_slice()
	}
	fn r#has_map(&self) -> &[HasMapProperty] {
		self.r#has_map.as_slice()
	}
	fn r#is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn r#isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn r#keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn r#latitude(&self) -> &[LatitudeProperty] {
		self.r#latitude.as_slice()
	}
	fn r#logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn r#longitude(&self) -> &[LongitudeProperty] {
		self.r#longitude.as_slice()
	}
	fn r#map(&self) -> &[MapProperty] {
		self.r#map.as_slice()
	}
	fn r#maps(&self) -> &[MapsProperty] {
		self.r#maps.as_slice()
	}
	fn r#maximum_attendee_capacity(&self) -> &[MaximumAttendeeCapacityProperty] {
		self.r#maximum_attendee_capacity.as_slice()
	}
	fn r#opening_hours_specification(&self) -> &[OpeningHoursSpecificationProperty] {
		self.r#opening_hours_specification.as_slice()
	}
	fn r#photo(&self) -> &[PhotoProperty] {
		self.r#photo.as_slice()
	}
	fn r#photos(&self) -> &[PhotosProperty] {
		self.r#photos.as_slice()
	}
	fn r#public_access(&self) -> &[PublicAccessProperty] {
		self.r#public_access.as_slice()
	}
	fn r#review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn r#reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn r#slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn r#smoking_allowed(&self) -> &[SmokingAllowedProperty] {
		self.r#smoking_allowed.as_slice()
	}
	fn r#special_opening_hours_specification(&self) -> &[SpecialOpeningHoursSpecificationProperty] {
		self.r#special_opening_hours_specification.as_slice()
	}
	fn r#telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn r#tour_booking_page(&self) -> &[TourBookingPageProperty] {
		self.r#tour_booking_page.as_slice()
	}
}
impl ThingTrait for BusStation {
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
