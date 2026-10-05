use super::*;
/// <https://schema.org/Flight>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Flight {
	/// <https://schema.org/aircraft>
	#[cfg_attr(feature = "serde", serde(rename = "aircraft"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#aircraft: Vec<AircraftProperty>,
	/// <https://schema.org/arrivalAirport>
	#[cfg_attr(feature = "serde", serde(rename = "arrivalAirport"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#arrival_airport: Vec<ArrivalAirportProperty>,
	/// <https://schema.org/arrivalGate>
	#[cfg_attr(feature = "serde", serde(rename = "arrivalGate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#arrival_gate: Vec<ArrivalGateProperty>,
	/// <https://schema.org/arrivalTerminal>
	#[cfg_attr(feature = "serde", serde(rename = "arrivalTerminal"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#arrival_terminal: Vec<ArrivalTerminalProperty>,
	/// <https://schema.org/boardingPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "boardingPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#boarding_policy: Vec<BoardingPolicyProperty>,
	/// <https://schema.org/carrier>
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	#[cfg_attr(feature = "serde", serde(rename = "carrier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#carrier: Vec<CarrierProperty>,
	/// <https://schema.org/departureAirport>
	#[cfg_attr(feature = "serde", serde(rename = "departureAirport"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#departure_airport: Vec<DepartureAirportProperty>,
	/// <https://schema.org/departureGate>
	#[cfg_attr(feature = "serde", serde(rename = "departureGate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#departure_gate: Vec<DepartureGateProperty>,
	/// <https://schema.org/departureTerminal>
	#[cfg_attr(feature = "serde", serde(rename = "departureTerminal"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#departure_terminal: Vec<DepartureTerminalProperty>,
	/// <https://schema.org/estimatedFlightDuration>
	#[cfg_attr(feature = "serde", serde(rename = "estimatedFlightDuration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#estimated_flight_duration: Vec<EstimatedFlightDurationProperty>,
	/// <https://schema.org/flightDistance>
	#[cfg_attr(feature = "serde", serde(rename = "flightDistance"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#flight_distance: Vec<FlightDistanceProperty>,
	/// <https://schema.org/flightNumber>
	#[cfg_attr(feature = "serde", serde(rename = "flightNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#flight_number: Vec<FlightNumberProperty>,
	/// <https://schema.org/mealService>
	#[cfg_attr(feature = "serde", serde(rename = "mealService"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#meal_service: Vec<MealServiceProperty>,
	/// <https://schema.org/seller>
	#[cfg_attr(feature = "serde", serde(rename = "seller"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#seller: Vec<SellerProperty>,
	/// <https://schema.org/webCheckinTime>
	#[cfg_attr(feature = "serde", serde(rename = "webCheckinTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#web_checkin_time: Vec<WebCheckinTimeProperty>,
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
/// This trait is for properties from <https://schema.org/Flight>.
pub trait FlightTrait {
	/// Get <https://schema.org/aircraft> from [`Self`] as borrowed slice.
	fn r#aircraft(&self) -> &[AircraftProperty];
	/// Get <https://schema.org/arrivalAirport> from [`Self`] as borrowed slice.
	fn r#arrival_airport(&self) -> &[ArrivalAirportProperty];
	/// Get <https://schema.org/arrivalGate> from [`Self`] as borrowed slice.
	fn r#arrival_gate(&self) -> &[ArrivalGateProperty];
	/// Get <https://schema.org/arrivalTerminal> from [`Self`] as borrowed slice.
	fn r#arrival_terminal(&self) -> &[ArrivalTerminalProperty];
	/// Get <https://schema.org/boardingPolicy> from [`Self`] as borrowed slice.
	fn r#boarding_policy(&self) -> &[BoardingPolicyProperty];
	/// Get <https://schema.org/carrier> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	fn r#carrier(&self) -> &[CarrierProperty];
	/// Get <https://schema.org/departureAirport> from [`Self`] as borrowed slice.
	fn r#departure_airport(&self) -> &[DepartureAirportProperty];
	/// Get <https://schema.org/departureGate> from [`Self`] as borrowed slice.
	fn r#departure_gate(&self) -> &[DepartureGateProperty];
	/// Get <https://schema.org/departureTerminal> from [`Self`] as borrowed slice.
	fn r#departure_terminal(&self) -> &[DepartureTerminalProperty];
	/// Get <https://schema.org/estimatedFlightDuration> from [`Self`] as borrowed slice.
	fn r#estimated_flight_duration(&self) -> &[EstimatedFlightDurationProperty];
	/// Get <https://schema.org/flightDistance> from [`Self`] as borrowed slice.
	fn r#flight_distance(&self) -> &[FlightDistanceProperty];
	/// Get <https://schema.org/flightNumber> from [`Self`] as borrowed slice.
	fn r#flight_number(&self) -> &[FlightNumberProperty];
	/// Get <https://schema.org/mealService> from [`Self`] as borrowed slice.
	fn r#meal_service(&self) -> &[MealServiceProperty];
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn r#seller(&self) -> &[SellerProperty];
	/// Get <https://schema.org/webCheckinTime> from [`Self`] as borrowed slice.
	fn r#web_checkin_time(&self) -> &[WebCheckinTimeProperty];
}
impl FlightTrait for Flight {
	fn r#aircraft(&self) -> &[AircraftProperty] {
		self.r#aircraft.as_slice()
	}
	fn r#arrival_airport(&self) -> &[ArrivalAirportProperty] {
		self.r#arrival_airport.as_slice()
	}
	fn r#arrival_gate(&self) -> &[ArrivalGateProperty] {
		self.r#arrival_gate.as_slice()
	}
	fn r#arrival_terminal(&self) -> &[ArrivalTerminalProperty] {
		self.r#arrival_terminal.as_slice()
	}
	fn r#boarding_policy(&self) -> &[BoardingPolicyProperty] {
		self.r#boarding_policy.as_slice()
	}
	fn r#carrier(&self) -> &[CarrierProperty] {
		self.r#carrier.as_slice()
	}
	fn r#departure_airport(&self) -> &[DepartureAirportProperty] {
		self.r#departure_airport.as_slice()
	}
	fn r#departure_gate(&self) -> &[DepartureGateProperty] {
		self.r#departure_gate.as_slice()
	}
	fn r#departure_terminal(&self) -> &[DepartureTerminalProperty] {
		self.r#departure_terminal.as_slice()
	}
	fn r#estimated_flight_duration(&self) -> &[EstimatedFlightDurationProperty] {
		self.r#estimated_flight_duration.as_slice()
	}
	fn r#flight_distance(&self) -> &[FlightDistanceProperty] {
		self.r#flight_distance.as_slice()
	}
	fn r#flight_number(&self) -> &[FlightNumberProperty] {
		self.r#flight_number.as_slice()
	}
	fn r#meal_service(&self) -> &[MealServiceProperty] {
		self.r#meal_service.as_slice()
	}
	fn r#seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn r#web_checkin_time(&self) -> &[WebCheckinTimeProperty] {
		self.r#web_checkin_time.as_slice()
	}
}
impl ThingTrait for Flight {
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
impl TripTrait for Flight {
	fn r#arrival_time(&self) -> &[ArrivalTimeProperty] {
		self.r#arrival_time.as_slice()
	}
	fn r#departure_time(&self) -> &[DepartureTimeProperty] {
		self.r#departure_time.as_slice()
	}
	fn r#itinerary(&self) -> &[ItineraryProperty] {
		self.r#itinerary.as_slice()
	}
	fn r#offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn r#part_of_trip(&self) -> &[PartOfTripProperty] {
		self.r#part_of_trip.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#sub_trip(&self) -> &[SubTripProperty] {
		self.r#sub_trip.as_slice()
	}
	fn r#trip_origin(&self) -> &[TripOriginProperty] {
		self.r#trip_origin.as_slice()
	}
}
