use super::*;
/// <https://schema.org/Patient>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Patient {
	/// <https://schema.org/diagnosis>
	#[cfg_attr(feature = "serde", serde(rename = "diagnosis"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#diagnosis: Vec<DiagnosisProperty>,
	/// <https://schema.org/drug>
	#[cfg_attr(feature = "serde", serde(rename = "drug"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#drug: Vec<DrugProperty>,
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
	/// <https://schema.org/additionalName>
	#[cfg_attr(feature = "serde", serde(rename = "additionalName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#additional_name: Vec<AdditionalNameProperty>,
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
	/// <https://schema.org/affiliation>
	#[cfg_attr(feature = "serde", serde(rename = "affiliation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#affiliation: Vec<AffiliationProperty>,
	/// <https://schema.org/agentInteractionStatistic>
	#[cfg_attr(feature = "serde", serde(rename = "agentInteractionStatistic"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#agent_interaction_statistic: Vec<AgentInteractionStatisticProperty>,
	/// <https://schema.org/alumniOf>
	#[cfg_attr(feature = "serde", serde(rename = "alumniOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#alumni_of: Vec<AlumniOfProperty>,
	/// <https://schema.org/award>
	#[cfg_attr(feature = "serde", serde(rename = "award"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#award: Vec<AwardProperty>,
	/// <https://schema.org/awards>
	#[deprecated = "This schema is superseded by <https://schema.org/award>."]
	#[cfg_attr(feature = "serde", serde(rename = "awards"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#awards: Vec<AwardsProperty>,
	/// <https://schema.org/birthDate>
	#[cfg_attr(feature = "serde", serde(rename = "birthDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#birth_date: Vec<BirthDateProperty>,
	/// <https://schema.org/birthPlace>
	#[cfg_attr(feature = "serde", serde(rename = "birthPlace"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#birth_place: Vec<BirthPlaceProperty>,
	/// <https://schema.org/brand>
	#[cfg_attr(feature = "serde", serde(rename = "brand"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#brand: Vec<BrandProperty>,
	/// <https://schema.org/callSign>
	#[cfg_attr(feature = "serde", serde(rename = "callSign"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#call_sign: Vec<CallSignProperty>,
	/// <https://schema.org/children>
	#[cfg_attr(feature = "serde", serde(rename = "children"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#children: Vec<ChildrenProperty>,
	/// <https://schema.org/colleague>
	#[cfg_attr(feature = "serde", serde(rename = "colleague"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#colleague: Vec<ColleagueProperty>,
	/// <https://schema.org/colleagues>
	#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
	#[cfg_attr(feature = "serde", serde(rename = "colleagues"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#colleagues: Vec<ColleaguesProperty>,
	/// <https://schema.org/contactPoint>
	#[cfg_attr(feature = "serde", serde(rename = "contactPoint"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contact_point: Vec<ContactPointProperty>,
	/// <https://schema.org/contactPoints>
	#[deprecated = "This schema is superseded by <https://schema.org/contactPoint>."]
	#[cfg_attr(feature = "serde", serde(rename = "contactPoints"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contact_points: Vec<ContactPointsProperty>,
	/// <https://schema.org/deathDate>
	#[cfg_attr(feature = "serde", serde(rename = "deathDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#death_date: Vec<DeathDateProperty>,
	/// <https://schema.org/deathPlace>
	#[cfg_attr(feature = "serde", serde(rename = "deathPlace"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#death_place: Vec<DeathPlaceProperty>,
	/// <https://schema.org/duns>
	#[cfg_attr(feature = "serde", serde(rename = "duns"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#duns: Vec<DunsProperty>,
	/// <https://schema.org/email>
	#[cfg_attr(feature = "serde", serde(rename = "email"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#email: Vec<EmailProperty>,
	/// <https://schema.org/familyName>
	#[cfg_attr(feature = "serde", serde(rename = "familyName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#family_name: Vec<FamilyNameProperty>,
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
	/// <https://schema.org/follows>
	#[cfg_attr(feature = "serde", serde(rename = "follows"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#follows: Vec<FollowsProperty>,
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
	/// <https://schema.org/gender>
	#[cfg_attr(feature = "serde", serde(rename = "gender"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#gender: Vec<GenderProperty>,
	/// <https://schema.org/givenName>
	#[cfg_attr(feature = "serde", serde(rename = "givenName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#given_name: Vec<GivenNameProperty>,
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
	/// <https://schema.org/hasCredential>
	#[cfg_attr(feature = "serde", serde(rename = "hasCredential"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_credential: Vec<HasCredentialProperty>,
	/// <https://schema.org/hasOccupation>
	#[cfg_attr(feature = "serde", serde(rename = "hasOccupation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_occupation: Vec<HasOccupationProperty>,
	/// <https://schema.org/hasOfferCatalog>
	#[cfg_attr(feature = "serde", serde(rename = "hasOfferCatalog"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	/// <https://schema.org/hasPOS>
	#[cfg_attr(feature = "serde", serde(rename = "hasPOS"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_pos: Vec<HasPosProperty>,
	/// <https://schema.org/height>
	#[cfg_attr(feature = "serde", serde(rename = "height"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#height: Vec<HeightProperty>,
	/// <https://schema.org/homeLocation>
	#[cfg_attr(feature = "serde", serde(rename = "homeLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#home_location: Vec<HomeLocationProperty>,
	/// <https://schema.org/honorificPrefix>
	#[cfg_attr(feature = "serde", serde(rename = "honorificPrefix"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#honorific_prefix: Vec<HonorificPrefixProperty>,
	/// <https://schema.org/honorificSuffix>
	#[cfg_attr(feature = "serde", serde(rename = "honorificSuffix"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#honorific_suffix: Vec<HonorificSuffixProperty>,
	/// <https://schema.org/interactionStatistic>
	#[cfg_attr(feature = "serde", serde(rename = "interactionStatistic"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
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
	/// <https://schema.org/jobTitle>
	#[cfg_attr(feature = "serde", serde(rename = "jobTitle"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#job_title: Vec<JobTitleProperty>,
	/// <https://schema.org/knows>
	#[cfg_attr(feature = "serde", serde(rename = "knows"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#knows: Vec<KnowsProperty>,
	/// <https://schema.org/knowsAbout>
	#[cfg_attr(feature = "serde", serde(rename = "knowsAbout"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#knows_about: Vec<KnowsAboutProperty>,
	/// <https://schema.org/knowsLanguage>
	#[cfg_attr(feature = "serde", serde(rename = "knowsLanguage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#knows_language: Vec<KnowsLanguageProperty>,
	/// <https://schema.org/lifeEvent>
	#[cfg_attr(feature = "serde", serde(rename = "lifeEvent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#life_event: Vec<LifeEventProperty>,
	/// <https://schema.org/makesOffer>
	#[cfg_attr(feature = "serde", serde(rename = "makesOffer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#makes_offer: Vec<MakesOfferProperty>,
	/// <https://schema.org/memberOf>
	#[cfg_attr(feature = "serde", serde(rename = "memberOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#member_of: Vec<MemberOfProperty>,
	/// <https://schema.org/naics>
	#[cfg_attr(feature = "serde", serde(rename = "naics"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#naics: Vec<NaicsProperty>,
	/// <https://schema.org/nationality>
	#[cfg_attr(feature = "serde", serde(rename = "nationality"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#nationality: Vec<NationalityProperty>,
	/// <https://schema.org/netWorth>
	#[cfg_attr(feature = "serde", serde(rename = "netWorth"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#net_worth: Vec<NetWorthProperty>,
	/// <https://schema.org/owns>
	#[cfg_attr(feature = "serde", serde(rename = "owns"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#owns: Vec<OwnsProperty>,
	/// <https://schema.org/parent>
	#[cfg_attr(feature = "serde", serde(rename = "parent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#parent: Vec<ParentProperty>,
	/// <https://schema.org/parents>
	#[deprecated = "This schema is superseded by <https://schema.org/parent>."]
	#[cfg_attr(feature = "serde", serde(rename = "parents"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#parents: Vec<ParentsProperty>,
	/// <https://schema.org/performerIn>
	#[cfg_attr(feature = "serde", serde(rename = "performerIn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#performer_in: Vec<PerformerInProperty>,
	/// <https://schema.org/pronouns>
	#[cfg_attr(feature = "serde", serde(rename = "pronouns"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#pronouns: Vec<PronounsProperty>,
	/// <https://schema.org/publishingPrinciples>
	#[cfg_attr(feature = "serde", serde(rename = "publishingPrinciples"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	/// <https://schema.org/relatedTo>
	#[cfg_attr(feature = "serde", serde(rename = "relatedTo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#related_to: Vec<RelatedToProperty>,
	/// <https://schema.org/seeks>
	#[cfg_attr(feature = "serde", serde(rename = "seeks"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#seeks: Vec<SeeksProperty>,
	/// <https://schema.org/sibling>
	#[cfg_attr(feature = "serde", serde(rename = "sibling"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sibling: Vec<SiblingProperty>,
	/// <https://schema.org/siblings>
	#[deprecated = "This schema is superseded by <https://schema.org/sibling>."]
	#[cfg_attr(feature = "serde", serde(rename = "siblings"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#siblings: Vec<SiblingsProperty>,
	/// <https://schema.org/skills>
	#[cfg_attr(feature = "serde", serde(rename = "skills"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#skills: Vec<SkillsProperty>,
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
	/// <https://schema.org/spouse>
	#[cfg_attr(feature = "serde", serde(rename = "spouse"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#spouse: Vec<SpouseProperty>,
	/// <https://schema.org/taxID>
	#[cfg_attr(feature = "serde", serde(rename = "taxID"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#tax_id: Vec<TaxIdProperty>,
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
	/// <https://schema.org/vatID>
	#[cfg_attr(feature = "serde", serde(rename = "vatID"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#vat_id: Vec<VatIdProperty>,
	/// <https://schema.org/weight>
	#[cfg_attr(feature = "serde", serde(rename = "weight"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#weight: Vec<WeightProperty>,
	/// <https://schema.org/workLocation>
	#[cfg_attr(feature = "serde", serde(rename = "workLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#work_location: Vec<WorkLocationProperty>,
	/// <https://schema.org/worksFor>
	#[cfg_attr(feature = "serde", serde(rename = "worksFor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#works_for: Vec<WorksForProperty>,
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
/// This trait is for properties from <https://schema.org/Patient>.
pub trait PatientTrait {
	/// Get <https://schema.org/diagnosis> from [`Self`] as borrowed slice.
	fn get_diagnosis(&self) -> &[DiagnosisProperty];
	/// Take <https://schema.org/diagnosis> from [`Self`] as owned vector.
	fn take_diagnosis(&mut self) -> Vec<DiagnosisProperty>;
	/// Get <https://schema.org/drug> from [`Self`] as borrowed slice.
	fn get_drug(&self) -> &[DrugProperty];
	/// Take <https://schema.org/drug> from [`Self`] as owned vector.
	fn take_drug(&mut self) -> Vec<DrugProperty>;
	/// Get <https://schema.org/healthCondition> from [`Self`] as borrowed slice.
	fn get_health_condition(&self) -> &[HealthConditionProperty];
	/// Take <https://schema.org/healthCondition> from [`Self`] as owned vector.
	fn take_health_condition(&mut self) -> Vec<HealthConditionProperty>;
}
impl PatientTrait for Patient {
	fn get_diagnosis(&self) -> &[DiagnosisProperty] {
		self.r#diagnosis.as_slice()
	}
	fn take_diagnosis(&mut self) -> Vec<DiagnosisProperty> {
		std::mem::take(&mut self.r#diagnosis)
	}
	fn get_drug(&self) -> &[DrugProperty] {
		self.r#drug.as_slice()
	}
	fn take_drug(&mut self) -> Vec<DrugProperty> {
		std::mem::take(&mut self.r#drug)
	}
	fn get_health_condition(&self) -> &[HealthConditionProperty] {
		self.r#health_condition.as_slice()
	}
	fn take_health_condition(&mut self) -> Vec<HealthConditionProperty> {
		std::mem::take(&mut self.r#health_condition)
	}
}
impl AudienceTrait for Patient {
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
impl MedicalAudienceTrait for Patient {}
impl PeopleAudienceTrait for Patient {
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
impl PersonTrait for Patient {
	fn get_additional_name(&self) -> &[AdditionalNameProperty] {
		self.r#additional_name.as_slice()
	}
	fn take_additional_name(&mut self) -> Vec<AdditionalNameProperty> {
		std::mem::take(&mut self.r#additional_name)
	}
	fn get_address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn take_address(&mut self) -> Vec<AddressProperty> {
		std::mem::take(&mut self.r#address)
	}
	fn get_affiliation(&self) -> &[AffiliationProperty] {
		self.r#affiliation.as_slice()
	}
	fn take_affiliation(&mut self) -> Vec<AffiliationProperty> {
		std::mem::take(&mut self.r#affiliation)
	}
	fn get_agent_interaction_statistic(&self) -> &[AgentInteractionStatisticProperty] {
		self.r#agent_interaction_statistic.as_slice()
	}
	fn take_agent_interaction_statistic(&mut self) -> Vec<AgentInteractionStatisticProperty> {
		std::mem::take(&mut self.r#agent_interaction_statistic)
	}
	fn get_alumni_of(&self) -> &[AlumniOfProperty] {
		self.r#alumni_of.as_slice()
	}
	fn take_alumni_of(&mut self) -> Vec<AlumniOfProperty> {
		std::mem::take(&mut self.r#alumni_of)
	}
	fn get_award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn take_award(&mut self) -> Vec<AwardProperty> {
		std::mem::take(&mut self.r#award)
	}
	fn get_awards(&self) -> &[AwardsProperty] {
		self.r#awards.as_slice()
	}
	fn take_awards(&mut self) -> Vec<AwardsProperty> {
		std::mem::take(&mut self.r#awards)
	}
	fn get_birth_date(&self) -> &[BirthDateProperty] {
		self.r#birth_date.as_slice()
	}
	fn take_birth_date(&mut self) -> Vec<BirthDateProperty> {
		std::mem::take(&mut self.r#birth_date)
	}
	fn get_birth_place(&self) -> &[BirthPlaceProperty] {
		self.r#birth_place.as_slice()
	}
	fn take_birth_place(&mut self) -> Vec<BirthPlaceProperty> {
		std::mem::take(&mut self.r#birth_place)
	}
	fn get_brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn take_brand(&mut self) -> Vec<BrandProperty> {
		std::mem::take(&mut self.r#brand)
	}
	fn get_call_sign(&self) -> &[CallSignProperty] {
		self.r#call_sign.as_slice()
	}
	fn take_call_sign(&mut self) -> Vec<CallSignProperty> {
		std::mem::take(&mut self.r#call_sign)
	}
	fn get_children(&self) -> &[ChildrenProperty] {
		self.r#children.as_slice()
	}
	fn take_children(&mut self) -> Vec<ChildrenProperty> {
		std::mem::take(&mut self.r#children)
	}
	fn get_colleague(&self) -> &[ColleagueProperty] {
		self.r#colleague.as_slice()
	}
	fn take_colleague(&mut self) -> Vec<ColleagueProperty> {
		std::mem::take(&mut self.r#colleague)
	}
	fn get_colleagues(&self) -> &[ColleaguesProperty] {
		self.r#colleagues.as_slice()
	}
	fn take_colleagues(&mut self) -> Vec<ColleaguesProperty> {
		std::mem::take(&mut self.r#colleagues)
	}
	fn get_contact_point(&self) -> &[ContactPointProperty] {
		self.r#contact_point.as_slice()
	}
	fn take_contact_point(&mut self) -> Vec<ContactPointProperty> {
		std::mem::take(&mut self.r#contact_point)
	}
	fn get_contact_points(&self) -> &[ContactPointsProperty] {
		self.r#contact_points.as_slice()
	}
	fn take_contact_points(&mut self) -> Vec<ContactPointsProperty> {
		std::mem::take(&mut self.r#contact_points)
	}
	fn get_death_date(&self) -> &[DeathDateProperty] {
		self.r#death_date.as_slice()
	}
	fn take_death_date(&mut self) -> Vec<DeathDateProperty> {
		std::mem::take(&mut self.r#death_date)
	}
	fn get_death_place(&self) -> &[DeathPlaceProperty] {
		self.r#death_place.as_slice()
	}
	fn take_death_place(&mut self) -> Vec<DeathPlaceProperty> {
		std::mem::take(&mut self.r#death_place)
	}
	fn get_duns(&self) -> &[DunsProperty] {
		self.r#duns.as_slice()
	}
	fn take_duns(&mut self) -> Vec<DunsProperty> {
		std::mem::take(&mut self.r#duns)
	}
	fn get_email(&self) -> &[EmailProperty] {
		self.r#email.as_slice()
	}
	fn take_email(&mut self) -> Vec<EmailProperty> {
		std::mem::take(&mut self.r#email)
	}
	fn get_family_name(&self) -> &[FamilyNameProperty] {
		self.r#family_name.as_slice()
	}
	fn take_family_name(&mut self) -> Vec<FamilyNameProperty> {
		std::mem::take(&mut self.r#family_name)
	}
	fn get_fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty> {
		std::mem::take(&mut self.r#fax_number)
	}
	fn get_follows(&self) -> &[FollowsProperty] {
		self.r#follows.as_slice()
	}
	fn take_follows(&mut self) -> Vec<FollowsProperty> {
		std::mem::take(&mut self.r#follows)
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
	fn get_gender(&self) -> &[GenderProperty] {
		self.r#gender.as_slice()
	}
	fn take_gender(&mut self) -> Vec<GenderProperty> {
		std::mem::take(&mut self.r#gender)
	}
	fn get_given_name(&self) -> &[GivenNameProperty] {
		self.r#given_name.as_slice()
	}
	fn take_given_name(&mut self) -> Vec<GivenNameProperty> {
		std::mem::take(&mut self.r#given_name)
	}
	fn get_global_location_number(&self) -> &[GlobalLocationNumberProperty] {
		self.r#global_location_number.as_slice()
	}
	fn take_global_location_number(&mut self) -> Vec<GlobalLocationNumberProperty> {
		std::mem::take(&mut self.r#global_location_number)
	}
	fn get_has_certification(&self) -> &[HasCertificationProperty] {
		self.r#has_certification.as_slice()
	}
	fn take_has_certification(&mut self) -> Vec<HasCertificationProperty> {
		std::mem::take(&mut self.r#has_certification)
	}
	fn get_has_credential(&self) -> &[HasCredentialProperty] {
		self.r#has_credential.as_slice()
	}
	fn take_has_credential(&mut self) -> Vec<HasCredentialProperty> {
		std::mem::take(&mut self.r#has_credential)
	}
	fn get_has_occupation(&self) -> &[HasOccupationProperty] {
		self.r#has_occupation.as_slice()
	}
	fn take_has_occupation(&mut self) -> Vec<HasOccupationProperty> {
		std::mem::take(&mut self.r#has_occupation)
	}
	fn get_has_offer_catalog(&self) -> &[HasOfferCatalogProperty] {
		self.r#has_offer_catalog.as_slice()
	}
	fn take_has_offer_catalog(&mut self) -> Vec<HasOfferCatalogProperty> {
		std::mem::take(&mut self.r#has_offer_catalog)
	}
	fn get_has_pos(&self) -> &[HasPosProperty] {
		self.r#has_pos.as_slice()
	}
	fn take_has_pos(&mut self) -> Vec<HasPosProperty> {
		std::mem::take(&mut self.r#has_pos)
	}
	fn get_height(&self) -> &[HeightProperty] {
		self.r#height.as_slice()
	}
	fn take_height(&mut self) -> Vec<HeightProperty> {
		std::mem::take(&mut self.r#height)
	}
	fn get_home_location(&self) -> &[HomeLocationProperty] {
		self.r#home_location.as_slice()
	}
	fn take_home_location(&mut self) -> Vec<HomeLocationProperty> {
		std::mem::take(&mut self.r#home_location)
	}
	fn get_honorific_prefix(&self) -> &[HonorificPrefixProperty] {
		self.r#honorific_prefix.as_slice()
	}
	fn take_honorific_prefix(&mut self) -> Vec<HonorificPrefixProperty> {
		std::mem::take(&mut self.r#honorific_prefix)
	}
	fn get_honorific_suffix(&self) -> &[HonorificSuffixProperty] {
		self.r#honorific_suffix.as_slice()
	}
	fn take_honorific_suffix(&mut self) -> Vec<HonorificSuffixProperty> {
		std::mem::take(&mut self.r#honorific_suffix)
	}
	fn get_interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn take_interaction_statistic(&mut self) -> Vec<InteractionStatisticProperty> {
		std::mem::take(&mut self.r#interaction_statistic)
	}
	fn get_isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn take_isic_v_4(&mut self) -> Vec<IsicV4Property> {
		std::mem::take(&mut self.r#isic_v_4)
	}
	fn get_job_title(&self) -> &[JobTitleProperty] {
		self.r#job_title.as_slice()
	}
	fn take_job_title(&mut self) -> Vec<JobTitleProperty> {
		std::mem::take(&mut self.r#job_title)
	}
	fn get_knows(&self) -> &[KnowsProperty] {
		self.r#knows.as_slice()
	}
	fn take_knows(&mut self) -> Vec<KnowsProperty> {
		std::mem::take(&mut self.r#knows)
	}
	fn get_knows_about(&self) -> &[KnowsAboutProperty] {
		self.r#knows_about.as_slice()
	}
	fn take_knows_about(&mut self) -> Vec<KnowsAboutProperty> {
		std::mem::take(&mut self.r#knows_about)
	}
	fn get_knows_language(&self) -> &[KnowsLanguageProperty] {
		self.r#knows_language.as_slice()
	}
	fn take_knows_language(&mut self) -> Vec<KnowsLanguageProperty> {
		std::mem::take(&mut self.r#knows_language)
	}
	fn get_life_event(&self) -> &[LifeEventProperty] {
		self.r#life_event.as_slice()
	}
	fn take_life_event(&mut self) -> Vec<LifeEventProperty> {
		std::mem::take(&mut self.r#life_event)
	}
	fn get_makes_offer(&self) -> &[MakesOfferProperty] {
		self.r#makes_offer.as_slice()
	}
	fn take_makes_offer(&mut self) -> Vec<MakesOfferProperty> {
		std::mem::take(&mut self.r#makes_offer)
	}
	fn get_member_of(&self) -> &[MemberOfProperty] {
		self.r#member_of.as_slice()
	}
	fn take_member_of(&mut self) -> Vec<MemberOfProperty> {
		std::mem::take(&mut self.r#member_of)
	}
	fn get_naics(&self) -> &[NaicsProperty] {
		self.r#naics.as_slice()
	}
	fn take_naics(&mut self) -> Vec<NaicsProperty> {
		std::mem::take(&mut self.r#naics)
	}
	fn get_nationality(&self) -> &[NationalityProperty] {
		self.r#nationality.as_slice()
	}
	fn take_nationality(&mut self) -> Vec<NationalityProperty> {
		std::mem::take(&mut self.r#nationality)
	}
	fn get_net_worth(&self) -> &[NetWorthProperty] {
		self.r#net_worth.as_slice()
	}
	fn take_net_worth(&mut self) -> Vec<NetWorthProperty> {
		std::mem::take(&mut self.r#net_worth)
	}
	fn get_owns(&self) -> &[OwnsProperty] {
		self.r#owns.as_slice()
	}
	fn take_owns(&mut self) -> Vec<OwnsProperty> {
		std::mem::take(&mut self.r#owns)
	}
	fn get_parent(&self) -> &[ParentProperty] {
		self.r#parent.as_slice()
	}
	fn take_parent(&mut self) -> Vec<ParentProperty> {
		std::mem::take(&mut self.r#parent)
	}
	fn get_parents(&self) -> &[ParentsProperty] {
		self.r#parents.as_slice()
	}
	fn take_parents(&mut self) -> Vec<ParentsProperty> {
		std::mem::take(&mut self.r#parents)
	}
	fn get_performer_in(&self) -> &[PerformerInProperty] {
		self.r#performer_in.as_slice()
	}
	fn take_performer_in(&mut self) -> Vec<PerformerInProperty> {
		std::mem::take(&mut self.r#performer_in)
	}
	fn get_pronouns(&self) -> &[PronounsProperty] {
		self.r#pronouns.as_slice()
	}
	fn take_pronouns(&mut self) -> Vec<PronounsProperty> {
		std::mem::take(&mut self.r#pronouns)
	}
	fn get_publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn take_publishing_principles(&mut self) -> Vec<PublishingPrinciplesProperty> {
		std::mem::take(&mut self.r#publishing_principles)
	}
	fn get_related_to(&self) -> &[RelatedToProperty] {
		self.r#related_to.as_slice()
	}
	fn take_related_to(&mut self) -> Vec<RelatedToProperty> {
		std::mem::take(&mut self.r#related_to)
	}
	fn get_seeks(&self) -> &[SeeksProperty] {
		self.r#seeks.as_slice()
	}
	fn take_seeks(&mut self) -> Vec<SeeksProperty> {
		std::mem::take(&mut self.r#seeks)
	}
	fn get_sibling(&self) -> &[SiblingProperty] {
		self.r#sibling.as_slice()
	}
	fn take_sibling(&mut self) -> Vec<SiblingProperty> {
		std::mem::take(&mut self.r#sibling)
	}
	fn get_siblings(&self) -> &[SiblingsProperty] {
		self.r#siblings.as_slice()
	}
	fn take_siblings(&mut self) -> Vec<SiblingsProperty> {
		std::mem::take(&mut self.r#siblings)
	}
	fn get_skills(&self) -> &[SkillsProperty] {
		self.r#skills.as_slice()
	}
	fn take_skills(&mut self) -> Vec<SkillsProperty> {
		std::mem::take(&mut self.r#skills)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_spouse(&self) -> &[SpouseProperty] {
		self.r#spouse.as_slice()
	}
	fn take_spouse(&mut self) -> Vec<SpouseProperty> {
		std::mem::take(&mut self.r#spouse)
	}
	fn get_tax_id(&self) -> &[TaxIdProperty] {
		self.r#tax_id.as_slice()
	}
	fn take_tax_id(&mut self) -> Vec<TaxIdProperty> {
		std::mem::take(&mut self.r#tax_id)
	}
	fn get_telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn take_telephone(&mut self) -> Vec<TelephoneProperty> {
		std::mem::take(&mut self.r#telephone)
	}
	fn get_vat_id(&self) -> &[VatIdProperty] {
		self.r#vat_id.as_slice()
	}
	fn take_vat_id(&mut self) -> Vec<VatIdProperty> {
		std::mem::take(&mut self.r#vat_id)
	}
	fn get_weight(&self) -> &[WeightProperty] {
		self.r#weight.as_slice()
	}
	fn take_weight(&mut self) -> Vec<WeightProperty> {
		std::mem::take(&mut self.r#weight)
	}
	fn get_work_location(&self) -> &[WorkLocationProperty] {
		self.r#work_location.as_slice()
	}
	fn take_work_location(&mut self) -> Vec<WorkLocationProperty> {
		std::mem::take(&mut self.r#work_location)
	}
	fn get_works_for(&self) -> &[WorksForProperty] {
		self.r#works_for.as_slice()
	}
	fn take_works_for(&mut self) -> Vec<WorksForProperty> {
		std::mem::take(&mut self.r#works_for)
	}
}
impl ThingTrait for Patient {
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
