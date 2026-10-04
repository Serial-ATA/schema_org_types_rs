use super::*;
/// <https://schema.org/TrainTrip>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct TrainTrip {
	/// <https://schema.org/arrivalPlatform>
	#[cfg_attr(feature = "serde", serde(rename = "arrivalPlatform"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#arrival_platform: Vec<ArrivalPlatformProperty>,
	/// <https://schema.org/arrivalStation>
	#[cfg_attr(feature = "serde", serde(rename = "arrivalStation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#arrival_station: Vec<ArrivalStationProperty>,
	/// <https://schema.org/departurePlatform>
	#[cfg_attr(feature = "serde", serde(rename = "departurePlatform"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#departure_platform: Vec<DeparturePlatformProperty>,
	/// <https://schema.org/departureStation>
	#[cfg_attr(feature = "serde", serde(rename = "departureStation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#departure_station: Vec<DepartureStationProperty>,
	/// <https://schema.org/trainName>
	#[cfg_attr(feature = "serde", serde(rename = "trainName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#train_name: Vec<TrainNameProperty>,
	/// <https://schema.org/trainNumber>
	#[cfg_attr(feature = "serde", serde(rename = "trainNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#train_number: Vec<TrainNumberProperty>,
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
	/// <https://schema.org/arrivalTime>
	#[cfg_attr(feature = "serde", serde(rename = "arrivalTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#arrival_time: Vec<ArrivalTimeProperty>,
	/// <https://schema.org/departureTime>
	#[cfg_attr(feature = "serde", serde(rename = "departureTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#departure_time: Vec<DepartureTimeProperty>,
	/// <https://schema.org/itinerary>
	#[cfg_attr(feature = "serde", serde(rename = "itinerary"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#itinerary: Vec<ItineraryProperty>,
	/// <https://schema.org/offers>
	#[cfg_attr(feature = "serde", serde(rename = "offers"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#offers: Vec<OffersProperty>,
	/// <https://schema.org/partOfTrip>
	#[cfg_attr(feature = "serde", serde(rename = "partOfTrip"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#part_of_trip: Vec<PartOfTripProperty>,
	/// <https://schema.org/provider>
	#[cfg_attr(feature = "serde", serde(rename = "provider"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#provider: Vec<ProviderProperty>,
	/// <https://schema.org/subTrip>
	#[cfg_attr(feature = "serde", serde(rename = "subTrip"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sub_trip: Vec<SubTripProperty>,
	/// <https://schema.org/tripOrigin>
	#[cfg_attr(feature = "serde", serde(rename = "tripOrigin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#trip_origin: Vec<TripOriginProperty>,
}
/// This trait is for properties from <https://schema.org/TrainTrip>.
pub trait TrainTripTrait {
	/// Get <https://schema.org/arrivalPlatform> from [`Self`] as borrowed slice.
	fn get_arrival_platform(&self) -> &[ArrivalPlatformProperty];
	/// Take <https://schema.org/arrivalPlatform> from [`Self`] as owned vector.
	fn take_arrival_platform(&mut self) -> Vec<ArrivalPlatformProperty>;
	/// Get <https://schema.org/arrivalStation> from [`Self`] as borrowed slice.
	fn get_arrival_station(&self) -> &[ArrivalStationProperty];
	/// Take <https://schema.org/arrivalStation> from [`Self`] as owned vector.
	fn take_arrival_station(&mut self) -> Vec<ArrivalStationProperty>;
	/// Get <https://schema.org/departurePlatform> from [`Self`] as borrowed slice.
	fn get_departure_platform(&self) -> &[DeparturePlatformProperty];
	/// Take <https://schema.org/departurePlatform> from [`Self`] as owned vector.
	fn take_departure_platform(&mut self) -> Vec<DeparturePlatformProperty>;
	/// Get <https://schema.org/departureStation> from [`Self`] as borrowed slice.
	fn get_departure_station(&self) -> &[DepartureStationProperty];
	/// Take <https://schema.org/departureStation> from [`Self`] as owned vector.
	fn take_departure_station(&mut self) -> Vec<DepartureStationProperty>;
	/// Get <https://schema.org/trainName> from [`Self`] as borrowed slice.
	fn get_train_name(&self) -> &[TrainNameProperty];
	/// Take <https://schema.org/trainName> from [`Self`] as owned vector.
	fn take_train_name(&mut self) -> Vec<TrainNameProperty>;
	/// Get <https://schema.org/trainNumber> from [`Self`] as borrowed slice.
	fn get_train_number(&self) -> &[TrainNumberProperty];
	/// Take <https://schema.org/trainNumber> from [`Self`] as owned vector.
	fn take_train_number(&mut self) -> Vec<TrainNumberProperty>;
}
impl TrainTripTrait for TrainTrip {
	fn get_arrival_platform(&self) -> &[ArrivalPlatformProperty] {
		self.r#arrival_platform.as_slice()
	}
	fn take_arrival_platform(&mut self) -> Vec<ArrivalPlatformProperty> {
		std::mem::take(&mut self.r#arrival_platform)
	}
	fn get_arrival_station(&self) -> &[ArrivalStationProperty] {
		self.r#arrival_station.as_slice()
	}
	fn take_arrival_station(&mut self) -> Vec<ArrivalStationProperty> {
		std::mem::take(&mut self.r#arrival_station)
	}
	fn get_departure_platform(&self) -> &[DeparturePlatformProperty] {
		self.r#departure_platform.as_slice()
	}
	fn take_departure_platform(&mut self) -> Vec<DeparturePlatformProperty> {
		std::mem::take(&mut self.r#departure_platform)
	}
	fn get_departure_station(&self) -> &[DepartureStationProperty] {
		self.r#departure_station.as_slice()
	}
	fn take_departure_station(&mut self) -> Vec<DepartureStationProperty> {
		std::mem::take(&mut self.r#departure_station)
	}
	fn get_train_name(&self) -> &[TrainNameProperty] {
		self.r#train_name.as_slice()
	}
	fn take_train_name(&mut self) -> Vec<TrainNameProperty> {
		std::mem::take(&mut self.r#train_name)
	}
	fn get_train_number(&self) -> &[TrainNumberProperty] {
		self.r#train_number.as_slice()
	}
	fn take_train_number(&mut self) -> Vec<TrainNumberProperty> {
		std::mem::take(&mut self.r#train_number)
	}
}
impl ThingTrait for TrainTrip {
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
impl TripTrait for TrainTrip {
	fn get_arrival_time(&self) -> &[ArrivalTimeProperty] {
		self.r#arrival_time.as_slice()
	}
	fn take_arrival_time(&mut self) -> Vec<ArrivalTimeProperty> {
		std::mem::take(&mut self.r#arrival_time)
	}
	fn get_departure_time(&self) -> &[DepartureTimeProperty] {
		self.r#departure_time.as_slice()
	}
	fn take_departure_time(&mut self) -> Vec<DepartureTimeProperty> {
		std::mem::take(&mut self.r#departure_time)
	}
	fn get_itinerary(&self) -> &[ItineraryProperty] {
		self.r#itinerary.as_slice()
	}
	fn take_itinerary(&mut self) -> Vec<ItineraryProperty> {
		std::mem::take(&mut self.r#itinerary)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_part_of_trip(&self) -> &[PartOfTripProperty] {
		self.r#part_of_trip.as_slice()
	}
	fn take_part_of_trip(&mut self) -> Vec<PartOfTripProperty> {
		std::mem::take(&mut self.r#part_of_trip)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_sub_trip(&self) -> &[SubTripProperty] {
		self.r#sub_trip.as_slice()
	}
	fn take_sub_trip(&mut self) -> Vec<SubTripProperty> {
		std::mem::take(&mut self.r#sub_trip)
	}
	fn get_trip_origin(&self) -> &[TripOriginProperty] {
		self.r#trip_origin.as_slice()
	}
	fn take_trip_origin(&mut self) -> Vec<TripOriginProperty> {
		std::mem::take(&mut self.r#trip_origin)
	}
}
