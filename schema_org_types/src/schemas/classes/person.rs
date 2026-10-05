use super::*;
/// <https://schema.org/Person>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Person {
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
/// This trait is for properties from <https://schema.org/Person>.
pub trait PersonTrait {
	/// Get <https://schema.org/additionalName> from [`Self`] as borrowed slice.
	fn r#additional_name(&self) -> &[AdditionalNameProperty];
	/// Get <https://schema.org/address> from [`Self`] as borrowed slice.
	fn r#address(&self) -> &[AddressProperty];
	/// Get <https://schema.org/affiliation> from [`Self`] as borrowed slice.
	fn r#affiliation(&self) -> &[AffiliationProperty];
	/// Get <https://schema.org/agentInteractionStatistic> from [`Self`] as borrowed slice.
	fn r#agent_interaction_statistic(&self) -> &[AgentInteractionStatisticProperty];
	/// Get <https://schema.org/alumniOf> from [`Self`] as borrowed slice.
	fn r#alumni_of(&self) -> &[AlumniOfProperty];
	/// Get <https://schema.org/award> from [`Self`] as borrowed slice.
	fn r#award(&self) -> &[AwardProperty];
	/// Get <https://schema.org/awards> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/award>."]
	fn r#awards(&self) -> &[AwardsProperty];
	/// Get <https://schema.org/birthDate> from [`Self`] as borrowed slice.
	fn r#birth_date(&self) -> &[BirthDateProperty];
	/// Get <https://schema.org/birthPlace> from [`Self`] as borrowed slice.
	fn r#birth_place(&self) -> &[BirthPlaceProperty];
	/// Get <https://schema.org/brand> from [`Self`] as borrowed slice.
	fn r#brand(&self) -> &[BrandProperty];
	/// Get <https://schema.org/callSign> from [`Self`] as borrowed slice.
	fn r#call_sign(&self) -> &[CallSignProperty];
	/// Get <https://schema.org/children> from [`Self`] as borrowed slice.
	fn r#children(&self) -> &[ChildrenProperty];
	/// Get <https://schema.org/colleague> from [`Self`] as borrowed slice.
	fn r#colleague(&self) -> &[ColleagueProperty];
	/// Get <https://schema.org/colleagues> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
	fn r#colleagues(&self) -> &[ColleaguesProperty];
	/// Get <https://schema.org/contactPoint> from [`Self`] as borrowed slice.
	fn r#contact_point(&self) -> &[ContactPointProperty];
	/// Get <https://schema.org/contactPoints> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/contactPoint>."]
	fn r#contact_points(&self) -> &[ContactPointsProperty];
	/// Get <https://schema.org/deathDate> from [`Self`] as borrowed slice.
	fn r#death_date(&self) -> &[DeathDateProperty];
	/// Get <https://schema.org/deathPlace> from [`Self`] as borrowed slice.
	fn r#death_place(&self) -> &[DeathPlaceProperty];
	/// Get <https://schema.org/duns> from [`Self`] as borrowed slice.
	fn r#duns(&self) -> &[DunsProperty];
	/// Get <https://schema.org/email> from [`Self`] as borrowed slice.
	fn r#email(&self) -> &[EmailProperty];
	/// Get <https://schema.org/familyName> from [`Self`] as borrowed slice.
	fn r#family_name(&self) -> &[FamilyNameProperty];
	/// Get <https://schema.org/faxNumber> from [`Self`] as borrowed slice.
	fn r#fax_number(&self) -> &[FaxNumberProperty];
	/// Get <https://schema.org/follows> from [`Self`] as borrowed slice.
	fn r#follows(&self) -> &[FollowsProperty];
	/// Get <https://schema.org/funder> from [`Self`] as borrowed slice.
	fn r#funder(&self) -> &[FunderProperty];
	/// Get <https://schema.org/funding> from [`Self`] as borrowed slice.
	fn r#funding(&self) -> &[FundingProperty];
	/// Get <https://schema.org/gender> from [`Self`] as borrowed slice.
	fn r#gender(&self) -> &[GenderProperty];
	/// Get <https://schema.org/givenName> from [`Self`] as borrowed slice.
	fn r#given_name(&self) -> &[GivenNameProperty];
	/// Get <https://schema.org/globalLocationNumber> from [`Self`] as borrowed slice.
	fn r#global_location_number(&self) -> &[GlobalLocationNumberProperty];
	/// Get <https://schema.org/hasCertification> from [`Self`] as borrowed slice.
	fn r#has_certification(&self) -> &[HasCertificationProperty];
	/// Get <https://schema.org/hasCredential> from [`Self`] as borrowed slice.
	fn r#has_credential(&self) -> &[HasCredentialProperty];
	/// Get <https://schema.org/hasOccupation> from [`Self`] as borrowed slice.
	fn r#has_occupation(&self) -> &[HasOccupationProperty];
	/// Get <https://schema.org/hasOfferCatalog> from [`Self`] as borrowed slice.
	fn r#has_offer_catalog(&self) -> &[HasOfferCatalogProperty];
	/// Get <https://schema.org/hasPOS> from [`Self`] as borrowed slice.
	fn r#has_pos(&self) -> &[HasPosProperty];
	/// Get <https://schema.org/height> from [`Self`] as borrowed slice.
	fn r#height(&self) -> &[HeightProperty];
	/// Get <https://schema.org/homeLocation> from [`Self`] as borrowed slice.
	fn r#home_location(&self) -> &[HomeLocationProperty];
	/// Get <https://schema.org/honorificPrefix> from [`Self`] as borrowed slice.
	fn r#honorific_prefix(&self) -> &[HonorificPrefixProperty];
	/// Get <https://schema.org/honorificSuffix> from [`Self`] as borrowed slice.
	fn r#honorific_suffix(&self) -> &[HonorificSuffixProperty];
	/// Get <https://schema.org/interactionStatistic> from [`Self`] as borrowed slice.
	fn r#interaction_statistic(&self) -> &[InteractionStatisticProperty];
	/// Get <https://schema.org/isicV4> from [`Self`] as borrowed slice.
	fn r#isic_v_4(&self) -> &[IsicV4Property];
	/// Get <https://schema.org/jobTitle> from [`Self`] as borrowed slice.
	fn r#job_title(&self) -> &[JobTitleProperty];
	/// Get <https://schema.org/knows> from [`Self`] as borrowed slice.
	fn r#knows(&self) -> &[KnowsProperty];
	/// Get <https://schema.org/knowsAbout> from [`Self`] as borrowed slice.
	fn r#knows_about(&self) -> &[KnowsAboutProperty];
	/// Get <https://schema.org/knowsLanguage> from [`Self`] as borrowed slice.
	fn r#knows_language(&self) -> &[KnowsLanguageProperty];
	/// Get <https://schema.org/lifeEvent> from [`Self`] as borrowed slice.
	fn r#life_event(&self) -> &[LifeEventProperty];
	/// Get <https://schema.org/makesOffer> from [`Self`] as borrowed slice.
	fn r#makes_offer(&self) -> &[MakesOfferProperty];
	/// Get <https://schema.org/memberOf> from [`Self`] as borrowed slice.
	fn r#member_of(&self) -> &[MemberOfProperty];
	/// Get <https://schema.org/naics> from [`Self`] as borrowed slice.
	fn r#naics(&self) -> &[NaicsProperty];
	/// Get <https://schema.org/nationality> from [`Self`] as borrowed slice.
	fn r#nationality(&self) -> &[NationalityProperty];
	/// Get <https://schema.org/netWorth> from [`Self`] as borrowed slice.
	fn r#net_worth(&self) -> &[NetWorthProperty];
	/// Get <https://schema.org/owns> from [`Self`] as borrowed slice.
	fn r#owns(&self) -> &[OwnsProperty];
	/// Get <https://schema.org/parent> from [`Self`] as borrowed slice.
	fn r#parent(&self) -> &[ParentProperty];
	/// Get <https://schema.org/parents> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/parent>."]
	fn r#parents(&self) -> &[ParentsProperty];
	/// Get <https://schema.org/performerIn> from [`Self`] as borrowed slice.
	fn r#performer_in(&self) -> &[PerformerInProperty];
	/// Get <https://schema.org/pronouns> from [`Self`] as borrowed slice.
	fn r#pronouns(&self) -> &[PronounsProperty];
	/// Get <https://schema.org/publishingPrinciples> from [`Self`] as borrowed slice.
	fn r#publishing_principles(&self) -> &[PublishingPrinciplesProperty];
	/// Get <https://schema.org/relatedTo> from [`Self`] as borrowed slice.
	fn r#related_to(&self) -> &[RelatedToProperty];
	/// Get <https://schema.org/seeks> from [`Self`] as borrowed slice.
	fn r#seeks(&self) -> &[SeeksProperty];
	/// Get <https://schema.org/sibling> from [`Self`] as borrowed slice.
	fn r#sibling(&self) -> &[SiblingProperty];
	/// Get <https://schema.org/siblings> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/sibling>."]
	fn r#siblings(&self) -> &[SiblingsProperty];
	/// Get <https://schema.org/skills> from [`Self`] as borrowed slice.
	fn r#skills(&self) -> &[SkillsProperty];
	/// Get <https://schema.org/sponsor> from [`Self`] as borrowed slice.
	fn r#sponsor(&self) -> &[SponsorProperty];
	/// Get <https://schema.org/spouse> from [`Self`] as borrowed slice.
	fn r#spouse(&self) -> &[SpouseProperty];
	/// Get <https://schema.org/taxID> from [`Self`] as borrowed slice.
	fn r#tax_id(&self) -> &[TaxIdProperty];
	/// Get <https://schema.org/telephone> from [`Self`] as borrowed slice.
	fn r#telephone(&self) -> &[TelephoneProperty];
	/// Get <https://schema.org/vatID> from [`Self`] as borrowed slice.
	fn r#vat_id(&self) -> &[VatIdProperty];
	/// Get <https://schema.org/weight> from [`Self`] as borrowed slice.
	fn r#weight(&self) -> &[WeightProperty];
	/// Get <https://schema.org/workLocation> from [`Self`] as borrowed slice.
	fn r#work_location(&self) -> &[WorkLocationProperty];
	/// Get <https://schema.org/worksFor> from [`Self`] as borrowed slice.
	fn r#works_for(&self) -> &[WorksForProperty];
}
impl PersonTrait for Person {
	fn r#additional_name(&self) -> &[AdditionalNameProperty] {
		self.r#additional_name.as_slice()
	}
	fn r#address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn r#affiliation(&self) -> &[AffiliationProperty] {
		self.r#affiliation.as_slice()
	}
	fn r#agent_interaction_statistic(&self) -> &[AgentInteractionStatisticProperty] {
		self.r#agent_interaction_statistic.as_slice()
	}
	fn r#alumni_of(&self) -> &[AlumniOfProperty] {
		self.r#alumni_of.as_slice()
	}
	fn r#award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn r#awards(&self) -> &[AwardsProperty] {
		self.r#awards.as_slice()
	}
	fn r#birth_date(&self) -> &[BirthDateProperty] {
		self.r#birth_date.as_slice()
	}
	fn r#birth_place(&self) -> &[BirthPlaceProperty] {
		self.r#birth_place.as_slice()
	}
	fn r#brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn r#call_sign(&self) -> &[CallSignProperty] {
		self.r#call_sign.as_slice()
	}
	fn r#children(&self) -> &[ChildrenProperty] {
		self.r#children.as_slice()
	}
	fn r#colleague(&self) -> &[ColleagueProperty] {
		self.r#colleague.as_slice()
	}
	fn r#colleagues(&self) -> &[ColleaguesProperty] {
		self.r#colleagues.as_slice()
	}
	fn r#contact_point(&self) -> &[ContactPointProperty] {
		self.r#contact_point.as_slice()
	}
	fn r#contact_points(&self) -> &[ContactPointsProperty] {
		self.r#contact_points.as_slice()
	}
	fn r#death_date(&self) -> &[DeathDateProperty] {
		self.r#death_date.as_slice()
	}
	fn r#death_place(&self) -> &[DeathPlaceProperty] {
		self.r#death_place.as_slice()
	}
	fn r#duns(&self) -> &[DunsProperty] {
		self.r#duns.as_slice()
	}
	fn r#email(&self) -> &[EmailProperty] {
		self.r#email.as_slice()
	}
	fn r#family_name(&self) -> &[FamilyNameProperty] {
		self.r#family_name.as_slice()
	}
	fn r#fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn r#follows(&self) -> &[FollowsProperty] {
		self.r#follows.as_slice()
	}
	fn r#funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn r#funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn r#gender(&self) -> &[GenderProperty] {
		self.r#gender.as_slice()
	}
	fn r#given_name(&self) -> &[GivenNameProperty] {
		self.r#given_name.as_slice()
	}
	fn r#global_location_number(&self) -> &[GlobalLocationNumberProperty] {
		self.r#global_location_number.as_slice()
	}
	fn r#has_certification(&self) -> &[HasCertificationProperty] {
		self.r#has_certification.as_slice()
	}
	fn r#has_credential(&self) -> &[HasCredentialProperty] {
		self.r#has_credential.as_slice()
	}
	fn r#has_occupation(&self) -> &[HasOccupationProperty] {
		self.r#has_occupation.as_slice()
	}
	fn r#has_offer_catalog(&self) -> &[HasOfferCatalogProperty] {
		self.r#has_offer_catalog.as_slice()
	}
	fn r#has_pos(&self) -> &[HasPosProperty] {
		self.r#has_pos.as_slice()
	}
	fn r#height(&self) -> &[HeightProperty] {
		self.r#height.as_slice()
	}
	fn r#home_location(&self) -> &[HomeLocationProperty] {
		self.r#home_location.as_slice()
	}
	fn r#honorific_prefix(&self) -> &[HonorificPrefixProperty] {
		self.r#honorific_prefix.as_slice()
	}
	fn r#honorific_suffix(&self) -> &[HonorificSuffixProperty] {
		self.r#honorific_suffix.as_slice()
	}
	fn r#interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn r#isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn r#job_title(&self) -> &[JobTitleProperty] {
		self.r#job_title.as_slice()
	}
	fn r#knows(&self) -> &[KnowsProperty] {
		self.r#knows.as_slice()
	}
	fn r#knows_about(&self) -> &[KnowsAboutProperty] {
		self.r#knows_about.as_slice()
	}
	fn r#knows_language(&self) -> &[KnowsLanguageProperty] {
		self.r#knows_language.as_slice()
	}
	fn r#life_event(&self) -> &[LifeEventProperty] {
		self.r#life_event.as_slice()
	}
	fn r#makes_offer(&self) -> &[MakesOfferProperty] {
		self.r#makes_offer.as_slice()
	}
	fn r#member_of(&self) -> &[MemberOfProperty] {
		self.r#member_of.as_slice()
	}
	fn r#naics(&self) -> &[NaicsProperty] {
		self.r#naics.as_slice()
	}
	fn r#nationality(&self) -> &[NationalityProperty] {
		self.r#nationality.as_slice()
	}
	fn r#net_worth(&self) -> &[NetWorthProperty] {
		self.r#net_worth.as_slice()
	}
	fn r#owns(&self) -> &[OwnsProperty] {
		self.r#owns.as_slice()
	}
	fn r#parent(&self) -> &[ParentProperty] {
		self.r#parent.as_slice()
	}
	fn r#parents(&self) -> &[ParentsProperty] {
		self.r#parents.as_slice()
	}
	fn r#performer_in(&self) -> &[PerformerInProperty] {
		self.r#performer_in.as_slice()
	}
	fn r#pronouns(&self) -> &[PronounsProperty] {
		self.r#pronouns.as_slice()
	}
	fn r#publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn r#related_to(&self) -> &[RelatedToProperty] {
		self.r#related_to.as_slice()
	}
	fn r#seeks(&self) -> &[SeeksProperty] {
		self.r#seeks.as_slice()
	}
	fn r#sibling(&self) -> &[SiblingProperty] {
		self.r#sibling.as_slice()
	}
	fn r#siblings(&self) -> &[SiblingsProperty] {
		self.r#siblings.as_slice()
	}
	fn r#skills(&self) -> &[SkillsProperty] {
		self.r#skills.as_slice()
	}
	fn r#sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn r#spouse(&self) -> &[SpouseProperty] {
		self.r#spouse.as_slice()
	}
	fn r#tax_id(&self) -> &[TaxIdProperty] {
		self.r#tax_id.as_slice()
	}
	fn r#telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn r#vat_id(&self) -> &[VatIdProperty] {
		self.r#vat_id.as_slice()
	}
	fn r#weight(&self) -> &[WeightProperty] {
		self.r#weight.as_slice()
	}
	fn r#work_location(&self) -> &[WorkLocationProperty] {
		self.r#work_location.as_slice()
	}
	fn r#works_for(&self) -> &[WorksForProperty] {
		self.r#works_for.as_slice()
	}
}
impl ThingTrait for Person {
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
