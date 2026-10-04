use super::*;
/// <https://schema.org/OnDemandEvent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct OnDemandEvent {
	/// <https://schema.org/about>
	#[cfg_attr(feature = "serde", serde(rename = "about"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#about: Vec<AboutProperty>,
	/// <https://schema.org/actor>
	#[cfg_attr(feature = "serde", serde(rename = "actor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#actor: Vec<ActorProperty>,
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
	/// <https://schema.org/attendee>
	#[cfg_attr(feature = "serde", serde(rename = "attendee"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#attendee: Vec<AttendeeProperty>,
	/// <https://schema.org/attendees>
	#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
	#[cfg_attr(feature = "serde", serde(rename = "attendees"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#attendees: Vec<AttendeesProperty>,
	/// <https://schema.org/audience>
	#[cfg_attr(feature = "serde", serde(rename = "audience"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#audience: Vec<AudienceProperty>,
	/// <https://schema.org/composer>
	#[cfg_attr(feature = "serde", serde(rename = "composer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#composer: Vec<ComposerProperty>,
	/// <https://schema.org/contributor>
	#[cfg_attr(feature = "serde", serde(rename = "contributor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contributor: Vec<ContributorProperty>,
	/// <https://schema.org/director>
	#[cfg_attr(feature = "serde", serde(rename = "director"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#director: Vec<DirectorProperty>,
	/// <https://schema.org/doorTime>
	#[cfg_attr(feature = "serde", serde(rename = "doorTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#door_time: Vec<DoorTimeProperty>,
	/// <https://schema.org/duration>
	#[cfg_attr(feature = "serde", serde(rename = "duration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#duration: Vec<DurationProperty>,
	/// <https://schema.org/endDate>
	#[cfg_attr(feature = "serde", serde(rename = "endDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#end_date: Vec<EndDateProperty>,
	/// <https://schema.org/eventAttendanceMode>
	#[cfg_attr(feature = "serde", serde(rename = "eventAttendanceMode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#event_attendance_mode: Vec<EventAttendanceModeProperty>,
	/// <https://schema.org/eventSchedule>
	#[cfg_attr(feature = "serde", serde(rename = "eventSchedule"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#event_schedule: Vec<EventScheduleProperty>,
	/// <https://schema.org/eventStatus>
	#[cfg_attr(feature = "serde", serde(rename = "eventStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#event_status: Vec<EventStatusProperty>,
	/// <https://schema.org/funder>
	#[cfg_attr(feature = "serde", serde(rename = "funder"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#funder: Vec<FunderProperty>,
	/// <https://schema.org/funding>
	#[cfg_attr(feature = "serde", serde(rename = "funding"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#funding: Vec<FundingProperty>,
	/// <https://schema.org/hasParticipationOffer>
	#[cfg_attr(feature = "serde", serde(rename = "hasParticipationOffer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_participation_offer: Vec<HasParticipationOfferProperty>,
	/// <https://schema.org/hasSponsorshipOffer>
	#[cfg_attr(feature = "serde", serde(rename = "hasSponsorshipOffer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_sponsorship_offer: Vec<HasSponsorshipOfferProperty>,
	/// <https://schema.org/inLanguage>
	#[cfg_attr(feature = "serde", serde(rename = "inLanguage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#in_language: Vec<InLanguageProperty>,
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
	/// <https://schema.org/location>
	#[cfg_attr(feature = "serde", serde(rename = "location"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#location: Vec<LocationProperty>,
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
	/// <https://schema.org/maximumPhysicalAttendeeCapacity>
	#[cfg_attr(feature = "serde", serde(rename = "maximumPhysicalAttendeeCapacity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#maximum_physical_attendee_capacity: Vec<MaximumPhysicalAttendeeCapacityProperty>,
	/// <https://schema.org/maximumVirtualAttendeeCapacity>
	#[cfg_attr(feature = "serde", serde(rename = "maximumVirtualAttendeeCapacity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#maximum_virtual_attendee_capacity: Vec<MaximumVirtualAttendeeCapacityProperty>,
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
	/// <https://schema.org/organizer>
	#[cfg_attr(feature = "serde", serde(rename = "organizer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#organizer: Vec<OrganizerProperty>,
	/// <https://schema.org/performer>
	#[cfg_attr(feature = "serde", serde(rename = "performer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#performer: Vec<PerformerProperty>,
	/// <https://schema.org/performers>
	#[deprecated = "This schema is superseded by <https://schema.org/performer>."]
	#[cfg_attr(feature = "serde", serde(rename = "performers"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#performers: Vec<PerformersProperty>,
	/// <https://schema.org/previousStartDate>
	#[cfg_attr(feature = "serde", serde(rename = "previousStartDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#previous_start_date: Vec<PreviousStartDateProperty>,
	/// <https://schema.org/recordedIn>
	#[cfg_attr(feature = "serde", serde(rename = "recordedIn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#recorded_in: Vec<RecordedInProperty>,
	/// <https://schema.org/remainingAttendeeCapacity>
	#[cfg_attr(feature = "serde", serde(rename = "remainingAttendeeCapacity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#remaining_attendee_capacity: Vec<RemainingAttendeeCapacityProperty>,
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
	/// <https://schema.org/sponsor>
	#[cfg_attr(feature = "serde", serde(rename = "sponsor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sponsor: Vec<SponsorProperty>,
	/// <https://schema.org/startDate>
	#[cfg_attr(feature = "serde", serde(rename = "startDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#start_date: Vec<StartDateProperty>,
	/// <https://schema.org/subEvent>
	#[cfg_attr(feature = "serde", serde(rename = "subEvent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sub_event: Vec<SubEventProperty>,
	/// <https://schema.org/subEvents>
	#[deprecated = "This schema is superseded by <https://schema.org/subEvent>."]
	#[cfg_attr(feature = "serde", serde(rename = "subEvents"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sub_events: Vec<SubEventsProperty>,
	/// <https://schema.org/superEvent>
	#[cfg_attr(feature = "serde", serde(rename = "superEvent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#super_event: Vec<SuperEventProperty>,
	/// <https://schema.org/translator>
	#[cfg_attr(feature = "serde", serde(rename = "translator"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#translator: Vec<TranslatorProperty>,
	/// <https://schema.org/typicalAgeRange>
	#[cfg_attr(feature = "serde", serde(rename = "typicalAgeRange"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
	/// <https://schema.org/workFeatured>
	#[cfg_attr(feature = "serde", serde(rename = "workFeatured"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#work_featured: Vec<WorkFeaturedProperty>,
	/// <https://schema.org/workPerformed>
	#[cfg_attr(feature = "serde", serde(rename = "workPerformed"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#work_performed: Vec<WorkPerformedProperty>,
	/// <https://schema.org/free>
	#[deprecated = "This schema is superseded by <https://schema.org/isAccessibleForFree>."]
	#[cfg_attr(feature = "serde", serde(rename = "free"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#free: Vec<FreeProperty>,
	/// <https://schema.org/publishedBy>
	#[cfg_attr(feature = "serde", serde(rename = "publishedBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#published_by: Vec<PublishedByProperty>,
	/// <https://schema.org/publishedOn>
	#[cfg_attr(feature = "serde", serde(rename = "publishedOn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#published_on: Vec<PublishedOnProperty>,
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
/// This trait is for properties from <https://schema.org/OnDemandEvent>.
pub trait OnDemandEventTrait {}
impl OnDemandEventTrait for OnDemandEvent {}
impl EventTrait for OnDemandEvent {
	fn get_about(&self) -> &[AboutProperty] {
		self.r#about.as_slice()
	}
	fn take_about(&mut self) -> Vec<AboutProperty> {
		std::mem::take(&mut self.r#about)
	}
	fn get_actor(&self) -> &[ActorProperty] {
		self.r#actor.as_slice()
	}
	fn take_actor(&mut self) -> Vec<ActorProperty> {
		std::mem::take(&mut self.r#actor)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_attendee(&self) -> &[AttendeeProperty] {
		self.r#attendee.as_slice()
	}
	fn take_attendee(&mut self) -> Vec<AttendeeProperty> {
		std::mem::take(&mut self.r#attendee)
	}
	fn get_attendees(&self) -> &[AttendeesProperty] {
		self.r#attendees.as_slice()
	}
	fn take_attendees(&mut self) -> Vec<AttendeesProperty> {
		std::mem::take(&mut self.r#attendees)
	}
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
	}
	fn get_composer(&self) -> &[ComposerProperty] {
		self.r#composer.as_slice()
	}
	fn take_composer(&mut self) -> Vec<ComposerProperty> {
		std::mem::take(&mut self.r#composer)
	}
	fn get_contributor(&self) -> &[ContributorProperty] {
		self.r#contributor.as_slice()
	}
	fn take_contributor(&mut self) -> Vec<ContributorProperty> {
		std::mem::take(&mut self.r#contributor)
	}
	fn get_director(&self) -> &[DirectorProperty] {
		self.r#director.as_slice()
	}
	fn take_director(&mut self) -> Vec<DirectorProperty> {
		std::mem::take(&mut self.r#director)
	}
	fn get_door_time(&self) -> &[DoorTimeProperty] {
		self.r#door_time.as_slice()
	}
	fn take_door_time(&mut self) -> Vec<DoorTimeProperty> {
		std::mem::take(&mut self.r#door_time)
	}
	fn get_duration(&self) -> &[DurationProperty] {
		self.r#duration.as_slice()
	}
	fn take_duration(&mut self) -> Vec<DurationProperty> {
		std::mem::take(&mut self.r#duration)
	}
	fn get_end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn take_end_date(&mut self) -> Vec<EndDateProperty> {
		std::mem::take(&mut self.r#end_date)
	}
	fn get_event_attendance_mode(&self) -> &[EventAttendanceModeProperty] {
		self.r#event_attendance_mode.as_slice()
	}
	fn take_event_attendance_mode(&mut self) -> Vec<EventAttendanceModeProperty> {
		std::mem::take(&mut self.r#event_attendance_mode)
	}
	fn get_event_schedule(&self) -> &[EventScheduleProperty] {
		self.r#event_schedule.as_slice()
	}
	fn take_event_schedule(&mut self) -> Vec<EventScheduleProperty> {
		std::mem::take(&mut self.r#event_schedule)
	}
	fn get_event_status(&self) -> &[EventStatusProperty] {
		self.r#event_status.as_slice()
	}
	fn take_event_status(&mut self) -> Vec<EventStatusProperty> {
		std::mem::take(&mut self.r#event_status)
	}
	fn get_funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn take_funder(&mut self) -> Vec<FunderProperty> {
		std::mem::take(&mut self.r#funder)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_has_participation_offer(&self) -> &[HasParticipationOfferProperty] {
		self.r#has_participation_offer.as_slice()
	}
	fn take_has_participation_offer(&mut self) -> Vec<HasParticipationOfferProperty> {
		std::mem::take(&mut self.r#has_participation_offer)
	}
	fn get_has_sponsorship_offer(&self) -> &[HasSponsorshipOfferProperty] {
		self.r#has_sponsorship_offer.as_slice()
	}
	fn take_has_sponsorship_offer(&mut self) -> Vec<HasSponsorshipOfferProperty> {
		std::mem::take(&mut self.r#has_sponsorship_offer)
	}
	fn get_in_language(&self) -> &[InLanguageProperty] {
		self.r#in_language.as_slice()
	}
	fn take_in_language(&mut self) -> Vec<InLanguageProperty> {
		std::mem::take(&mut self.r#in_language)
	}
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty> {
		std::mem::take(&mut self.r#is_accessible_for_free)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_location(&self) -> &[LocationProperty] {
		self.r#location.as_slice()
	}
	fn take_location(&mut self) -> Vec<LocationProperty> {
		std::mem::take(&mut self.r#location)
	}
	fn get_maximum_attendee_capacity(&self) -> &[MaximumAttendeeCapacityProperty] {
		self.r#maximum_attendee_capacity.as_slice()
	}
	fn take_maximum_attendee_capacity(&mut self) -> Vec<MaximumAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_attendee_capacity)
	}
	fn get_maximum_physical_attendee_capacity(&self) -> &[MaximumPhysicalAttendeeCapacityProperty] {
		self.r#maximum_physical_attendee_capacity.as_slice()
	}
	fn take_maximum_physical_attendee_capacity(
		&mut self,
	) -> Vec<MaximumPhysicalAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_physical_attendee_capacity)
	}
	fn get_maximum_virtual_attendee_capacity(&self) -> &[MaximumVirtualAttendeeCapacityProperty] {
		self.r#maximum_virtual_attendee_capacity.as_slice()
	}
	fn take_maximum_virtual_attendee_capacity(
		&mut self,
	) -> Vec<MaximumVirtualAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_virtual_attendee_capacity)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_organizer(&self) -> &[OrganizerProperty] {
		self.r#organizer.as_slice()
	}
	fn take_organizer(&mut self) -> Vec<OrganizerProperty> {
		std::mem::take(&mut self.r#organizer)
	}
	fn get_performer(&self) -> &[PerformerProperty] {
		self.r#performer.as_slice()
	}
	fn take_performer(&mut self) -> Vec<PerformerProperty> {
		std::mem::take(&mut self.r#performer)
	}
	fn get_performers(&self) -> &[PerformersProperty] {
		self.r#performers.as_slice()
	}
	fn take_performers(&mut self) -> Vec<PerformersProperty> {
		std::mem::take(&mut self.r#performers)
	}
	fn get_previous_start_date(&self) -> &[PreviousStartDateProperty] {
		self.r#previous_start_date.as_slice()
	}
	fn take_previous_start_date(&mut self) -> Vec<PreviousStartDateProperty> {
		std::mem::take(&mut self.r#previous_start_date)
	}
	fn get_recorded_in(&self) -> &[RecordedInProperty] {
		self.r#recorded_in.as_slice()
	}
	fn take_recorded_in(&mut self) -> Vec<RecordedInProperty> {
		std::mem::take(&mut self.r#recorded_in)
	}
	fn get_remaining_attendee_capacity(&self) -> &[RemainingAttendeeCapacityProperty] {
		self.r#remaining_attendee_capacity.as_slice()
	}
	fn take_remaining_attendee_capacity(&mut self) -> Vec<RemainingAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#remaining_attendee_capacity)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn take_start_date(&mut self) -> Vec<StartDateProperty> {
		std::mem::take(&mut self.r#start_date)
	}
	fn get_sub_event(&self) -> &[SubEventProperty] {
		self.r#sub_event.as_slice()
	}
	fn take_sub_event(&mut self) -> Vec<SubEventProperty> {
		std::mem::take(&mut self.r#sub_event)
	}
	fn get_sub_events(&self) -> &[SubEventsProperty] {
		self.r#sub_events.as_slice()
	}
	fn take_sub_events(&mut self) -> Vec<SubEventsProperty> {
		std::mem::take(&mut self.r#sub_events)
	}
	fn get_super_event(&self) -> &[SuperEventProperty] {
		self.r#super_event.as_slice()
	}
	fn take_super_event(&mut self) -> Vec<SuperEventProperty> {
		std::mem::take(&mut self.r#super_event)
	}
	fn get_translator(&self) -> &[TranslatorProperty] {
		self.r#translator.as_slice()
	}
	fn take_translator(&mut self) -> Vec<TranslatorProperty> {
		std::mem::take(&mut self.r#translator)
	}
	fn get_typical_age_range(&self) -> &[TypicalAgeRangeProperty] {
		self.r#typical_age_range.as_slice()
	}
	fn take_typical_age_range(&mut self) -> Vec<TypicalAgeRangeProperty> {
		std::mem::take(&mut self.r#typical_age_range)
	}
	fn get_work_featured(&self) -> &[WorkFeaturedProperty] {
		self.r#work_featured.as_slice()
	}
	fn take_work_featured(&mut self) -> Vec<WorkFeaturedProperty> {
		std::mem::take(&mut self.r#work_featured)
	}
	fn get_work_performed(&self) -> &[WorkPerformedProperty] {
		self.r#work_performed.as_slice()
	}
	fn take_work_performed(&mut self) -> Vec<WorkPerformedProperty> {
		std::mem::take(&mut self.r#work_performed)
	}
}
impl PublicationEventTrait for OnDemandEvent {
	fn get_free(&self) -> &[FreeProperty] {
		self.r#free.as_slice()
	}
	fn take_free(&mut self) -> Vec<FreeProperty> {
		std::mem::take(&mut self.r#free)
	}
	fn get_published_by(&self) -> &[PublishedByProperty] {
		self.r#published_by.as_slice()
	}
	fn take_published_by(&mut self) -> Vec<PublishedByProperty> {
		std::mem::take(&mut self.r#published_by)
	}
	fn get_published_on(&self) -> &[PublishedOnProperty] {
		self.r#published_on.as_slice()
	}
	fn take_published_on(&mut self) -> Vec<PublishedOnProperty> {
		std::mem::take(&mut self.r#published_on)
	}
}
impl ThingTrait for OnDemandEvent {
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
