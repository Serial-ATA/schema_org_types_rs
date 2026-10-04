use super::*;
/// <https://schema.org/Bakery>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Bakery {
	/// <https://schema.org/acceptsReservations>
	#[cfg_attr(feature = "serde", serde(rename = "acceptsReservations"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accepts_reservations: Vec<AcceptsReservationsProperty>,
	/// <https://schema.org/hasMenu>
	#[cfg_attr(feature = "serde", serde(rename = "hasMenu"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_menu: Vec<HasMenuProperty>,
	/// <https://schema.org/menu>
	#[deprecated = "This schema is superseded by <https://schema.org/hasMenu>."]
	#[cfg_attr(feature = "serde", serde(rename = "menu"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#menu: Vec<MenuProperty>,
	/// <https://schema.org/servesCuisine>
	#[cfg_attr(feature = "serde", serde(rename = "servesCuisine"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#serves_cuisine: Vec<ServesCuisineProperty>,
	/// <https://schema.org/starRating>
	#[cfg_attr(feature = "serde", serde(rename = "starRating"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#star_rating: Vec<StarRatingProperty>,
	/// <https://schema.org/branchOf>
	#[deprecated = "This schema is superseded by <https://schema.org/parentOrganization>."]
	#[cfg_attr(feature = "serde", serde(rename = "branchOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#branch_of: Vec<BranchOfProperty>,
	/// <https://schema.org/currenciesAccepted>
	#[cfg_attr(feature = "serde", serde(rename = "currenciesAccepted"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#currencies_accepted: Vec<CurrenciesAcceptedProperty>,
	/// <https://schema.org/floorLevel>
	#[cfg_attr(feature = "serde", serde(rename = "floorLevel"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#floor_level: Vec<FloorLevelProperty>,
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
	/// <https://schema.org/paymentAccepted>
	#[cfg_attr(feature = "serde", serde(rename = "paymentAccepted"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_accepted: Vec<PaymentAcceptedProperty>,
	/// <https://schema.org/priceRange>
	#[cfg_attr(feature = "serde", serde(rename = "priceRange"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price_range: Vec<PriceRangeProperty>,
	/// <https://schema.org/acceptedPaymentMethod>
	#[cfg_attr(feature = "serde", serde(rename = "acceptedPaymentMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accepted_payment_method: Vec<AcceptedPaymentMethodProperty>,
	/// <https://schema.org/actionableFeedbackPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "actionableFeedbackPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#actionable_feedback_policy: Vec<ActionableFeedbackPolicyProperty>,
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
	/// <https://schema.org/alumni>
	#[cfg_attr(feature = "serde", serde(rename = "alumni"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#alumni: Vec<AlumniProperty>,
	/// <https://schema.org/areaServed>
	#[cfg_attr(feature = "serde", serde(rename = "areaServed"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#area_served: Vec<AreaServedProperty>,
	/// <https://schema.org/authorizedRepresentative>
	#[cfg_attr(feature = "serde", serde(rename = "authorizedRepresentative"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#authorized_representative: Vec<AuthorizedRepresentativeProperty>,
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
	/// <https://schema.org/companyRegistration>
	#[cfg_attr(feature = "serde", serde(rename = "companyRegistration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#company_registration: Vec<CompanyRegistrationProperty>,
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
	/// <https://schema.org/correctionsPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "correctionsPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#corrections_policy: Vec<CorrectionsPolicyProperty>,
	/// <https://schema.org/department>
	#[cfg_attr(feature = "serde", serde(rename = "department"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#department: Vec<DepartmentProperty>,
	/// <https://schema.org/dissolutionDate>
	#[cfg_attr(feature = "serde", serde(rename = "dissolutionDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#dissolution_date: Vec<DissolutionDateProperty>,
	/// <https://schema.org/diversityPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "diversityPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#diversity_policy: Vec<DiversityPolicyProperty>,
	/// <https://schema.org/diversityStaffingReport>
	#[cfg_attr(feature = "serde", serde(rename = "diversityStaffingReport"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#diversity_staffing_report: Vec<DiversityStaffingReportProperty>,
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
	/// <https://schema.org/employee>
	#[cfg_attr(feature = "serde", serde(rename = "employee"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#employee: Vec<EmployeeProperty>,
	/// <https://schema.org/employees>
	#[deprecated = "This schema is superseded by <https://schema.org/employee>."]
	#[cfg_attr(feature = "serde", serde(rename = "employees"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#employees: Vec<EmployeesProperty>,
	/// <https://schema.org/ethicsPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "ethicsPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ethics_policy: Vec<EthicsPolicyProperty>,
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
	/// <https://schema.org/founder>
	#[cfg_attr(feature = "serde", serde(rename = "founder"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#founder: Vec<FounderProperty>,
	/// <https://schema.org/founders>
	#[deprecated = "This schema is superseded by <https://schema.org/founder>."]
	#[cfg_attr(feature = "serde", serde(rename = "founders"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#founders: Vec<FoundersProperty>,
	/// <https://schema.org/foundingDate>
	#[cfg_attr(feature = "serde", serde(rename = "foundingDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#founding_date: Vec<FoundingDateProperty>,
	/// <https://schema.org/foundingLocation>
	#[cfg_attr(feature = "serde", serde(rename = "foundingLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#founding_location: Vec<FoundingLocationProperty>,
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
	/// <https://schema.org/hasMemberProgram>
	#[cfg_attr(feature = "serde", serde(rename = "hasMemberProgram"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_member_program: Vec<HasMemberProgramProperty>,
	/// <https://schema.org/hasMerchantReturnPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "hasMerchantReturnPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
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
	/// <https://schema.org/hasProductReturnPolicy>
	#[deprecated = "This schema is superseded by <https://schema.org/hasMerchantReturnPolicy>."]
	#[cfg_attr(feature = "serde", serde(rename = "hasProductReturnPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
	/// <https://schema.org/hasShippingService>
	#[cfg_attr(feature = "serde", serde(rename = "hasShippingService"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_shipping_service: Vec<HasShippingServiceProperty>,
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
	/// <https://schema.org/iso6523Code>
	#[cfg_attr(feature = "serde", serde(rename = "iso6523Code"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#iso_6523_code: Vec<Iso6523CodeProperty>,
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
	/// <https://schema.org/legalAddress>
	#[cfg_attr(feature = "serde", serde(rename = "legalAddress"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#legal_address: Vec<LegalAddressProperty>,
	/// <https://schema.org/legalName>
	#[cfg_attr(feature = "serde", serde(rename = "legalName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#legal_name: Vec<LegalNameProperty>,
	/// <https://schema.org/legalRepresentative>
	#[cfg_attr(feature = "serde", serde(rename = "legalRepresentative"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#legal_representative: Vec<LegalRepresentativeProperty>,
	/// <https://schema.org/leiCode>
	#[cfg_attr(feature = "serde", serde(rename = "leiCode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#lei_code: Vec<LeiCodeProperty>,
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
	/// <https://schema.org/member>
	#[cfg_attr(feature = "serde", serde(rename = "member"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#member: Vec<MemberProperty>,
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
	/// <https://schema.org/members>
	#[deprecated = "This schema is superseded by <https://schema.org/member>."]
	#[cfg_attr(feature = "serde", serde(rename = "members"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#members: Vec<MembersProperty>,
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
	/// <https://schema.org/nonprofitStatus>
	#[cfg_attr(feature = "serde", serde(rename = "nonprofitStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#nonprofit_status: Vec<NonprofitStatusProperty>,
	/// <https://schema.org/numberOfEmployees>
	#[cfg_attr(feature = "serde", serde(rename = "numberOfEmployees"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#number_of_employees: Vec<NumberOfEmployeesProperty>,
	/// <https://schema.org/ownershipFundingInfo>
	#[cfg_attr(feature = "serde", serde(rename = "ownershipFundingInfo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ownership_funding_info: Vec<OwnershipFundingInfoProperty>,
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
	/// <https://schema.org/parentOrganization>
	#[cfg_attr(feature = "serde", serde(rename = "parentOrganization"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#parent_organization: Vec<ParentOrganizationProperty>,
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
	/// <https://schema.org/serviceArea>
	#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
	#[cfg_attr(feature = "serde", serde(rename = "serviceArea"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#service_area: Vec<ServiceAreaProperty>,
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
	/// <https://schema.org/subOrganization>
	#[cfg_attr(feature = "serde", serde(rename = "subOrganization"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sub_organization: Vec<SubOrganizationProperty>,
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
	/// <https://schema.org/unnamedSourcesPolicy>
	#[cfg_attr(feature = "serde", serde(rename = "unnamedSourcesPolicy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#unnamed_sources_policy: Vec<UnnamedSourcesPolicyProperty>,
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
/// This trait is for properties from <https://schema.org/Bakery>.
pub trait BakeryTrait {}
impl BakeryTrait for Bakery {}
impl FoodEstablishmentTrait for Bakery {
	fn get_accepts_reservations(&self) -> &[AcceptsReservationsProperty] {
		self.r#accepts_reservations.as_slice()
	}
	fn take_accepts_reservations(&mut self) -> Vec<AcceptsReservationsProperty> {
		std::mem::take(&mut self.r#accepts_reservations)
	}
	fn get_has_menu(&self) -> &[HasMenuProperty] {
		self.r#has_menu.as_slice()
	}
	fn take_has_menu(&mut self) -> Vec<HasMenuProperty> {
		std::mem::take(&mut self.r#has_menu)
	}
	fn get_menu(&self) -> &[MenuProperty] {
		self.r#menu.as_slice()
	}
	fn take_menu(&mut self) -> Vec<MenuProperty> {
		std::mem::take(&mut self.r#menu)
	}
	fn get_serves_cuisine(&self) -> &[ServesCuisineProperty] {
		self.r#serves_cuisine.as_slice()
	}
	fn take_serves_cuisine(&mut self) -> Vec<ServesCuisineProperty> {
		std::mem::take(&mut self.r#serves_cuisine)
	}
	fn get_star_rating(&self) -> &[StarRatingProperty] {
		self.r#star_rating.as_slice()
	}
	fn take_star_rating(&mut self) -> Vec<StarRatingProperty> {
		std::mem::take(&mut self.r#star_rating)
	}
}
impl LocalBusinessTrait for Bakery {
	fn get_branch_of(&self) -> &[BranchOfProperty] {
		self.r#branch_of.as_slice()
	}
	fn take_branch_of(&mut self) -> Vec<BranchOfProperty> {
		std::mem::take(&mut self.r#branch_of)
	}
	fn get_currencies_accepted(&self) -> &[CurrenciesAcceptedProperty] {
		self.r#currencies_accepted.as_slice()
	}
	fn take_currencies_accepted(&mut self) -> Vec<CurrenciesAcceptedProperty> {
		std::mem::take(&mut self.r#currencies_accepted)
	}
	fn get_floor_level(&self) -> &[FloorLevelProperty] {
		self.r#floor_level.as_slice()
	}
	fn take_floor_level(&mut self) -> Vec<FloorLevelProperty> {
		std::mem::take(&mut self.r#floor_level)
	}
	fn get_opening_hours(&self) -> &[OpeningHoursProperty] {
		self.r#opening_hours.as_slice()
	}
	fn take_opening_hours(&mut self) -> Vec<OpeningHoursProperty> {
		std::mem::take(&mut self.r#opening_hours)
	}
	fn get_payment_accepted(&self) -> &[PaymentAcceptedProperty] {
		self.r#payment_accepted.as_slice()
	}
	fn take_payment_accepted(&mut self) -> Vec<PaymentAcceptedProperty> {
		std::mem::take(&mut self.r#payment_accepted)
	}
	fn get_price_range(&self) -> &[PriceRangeProperty] {
		self.r#price_range.as_slice()
	}
	fn take_price_range(&mut self) -> Vec<PriceRangeProperty> {
		std::mem::take(&mut self.r#price_range)
	}
}
impl OrganizationTrait for Bakery {
	fn get_accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty] {
		self.r#accepted_payment_method.as_slice()
	}
	fn take_accepted_payment_method(&mut self) -> Vec<AcceptedPaymentMethodProperty> {
		std::mem::take(&mut self.r#accepted_payment_method)
	}
	fn get_actionable_feedback_policy(&self) -> &[ActionableFeedbackPolicyProperty] {
		self.r#actionable_feedback_policy.as_slice()
	}
	fn take_actionable_feedback_policy(&mut self) -> Vec<ActionableFeedbackPolicyProperty> {
		std::mem::take(&mut self.r#actionable_feedback_policy)
	}
	fn get_address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn take_address(&mut self) -> Vec<AddressProperty> {
		std::mem::take(&mut self.r#address)
	}
	fn get_agent_interaction_statistic(&self) -> &[AgentInteractionStatisticProperty] {
		self.r#agent_interaction_statistic.as_slice()
	}
	fn take_agent_interaction_statistic(&mut self) -> Vec<AgentInteractionStatisticProperty> {
		std::mem::take(&mut self.r#agent_interaction_statistic)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_alumni(&self) -> &[AlumniProperty] {
		self.r#alumni.as_slice()
	}
	fn take_alumni(&mut self) -> Vec<AlumniProperty> {
		std::mem::take(&mut self.r#alumni)
	}
	fn get_area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn take_area_served(&mut self) -> Vec<AreaServedProperty> {
		std::mem::take(&mut self.r#area_served)
	}
	fn get_authorized_representative(&self) -> &[AuthorizedRepresentativeProperty] {
		self.r#authorized_representative.as_slice()
	}
	fn take_authorized_representative(&mut self) -> Vec<AuthorizedRepresentativeProperty> {
		std::mem::take(&mut self.r#authorized_representative)
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
	fn get_brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn take_brand(&mut self) -> Vec<BrandProperty> {
		std::mem::take(&mut self.r#brand)
	}
	fn get_company_registration(&self) -> &[CompanyRegistrationProperty] {
		self.r#company_registration.as_slice()
	}
	fn take_company_registration(&mut self) -> Vec<CompanyRegistrationProperty> {
		std::mem::take(&mut self.r#company_registration)
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
	fn get_corrections_policy(&self) -> &[CorrectionsPolicyProperty] {
		self.r#corrections_policy.as_slice()
	}
	fn take_corrections_policy(&mut self) -> Vec<CorrectionsPolicyProperty> {
		std::mem::take(&mut self.r#corrections_policy)
	}
	fn get_department(&self) -> &[DepartmentProperty] {
		self.r#department.as_slice()
	}
	fn take_department(&mut self) -> Vec<DepartmentProperty> {
		std::mem::take(&mut self.r#department)
	}
	fn get_dissolution_date(&self) -> &[DissolutionDateProperty] {
		self.r#dissolution_date.as_slice()
	}
	fn take_dissolution_date(&mut self) -> Vec<DissolutionDateProperty> {
		std::mem::take(&mut self.r#dissolution_date)
	}
	fn get_diversity_policy(&self) -> &[DiversityPolicyProperty] {
		self.r#diversity_policy.as_slice()
	}
	fn take_diversity_policy(&mut self) -> Vec<DiversityPolicyProperty> {
		std::mem::take(&mut self.r#diversity_policy)
	}
	fn get_diversity_staffing_report(&self) -> &[DiversityStaffingReportProperty] {
		self.r#diversity_staffing_report.as_slice()
	}
	fn take_diversity_staffing_report(&mut self) -> Vec<DiversityStaffingReportProperty> {
		std::mem::take(&mut self.r#diversity_staffing_report)
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
	fn get_employee(&self) -> &[EmployeeProperty] {
		self.r#employee.as_slice()
	}
	fn take_employee(&mut self) -> Vec<EmployeeProperty> {
		std::mem::take(&mut self.r#employee)
	}
	fn get_employees(&self) -> &[EmployeesProperty] {
		self.r#employees.as_slice()
	}
	fn take_employees(&mut self) -> Vec<EmployeesProperty> {
		std::mem::take(&mut self.r#employees)
	}
	fn get_ethics_policy(&self) -> &[EthicsPolicyProperty] {
		self.r#ethics_policy.as_slice()
	}
	fn take_ethics_policy(&mut self) -> Vec<EthicsPolicyProperty> {
		std::mem::take(&mut self.r#ethics_policy)
	}
	fn get_event(&self) -> &[EventProperty] {
		self.r#event.as_slice()
	}
	fn take_event(&mut self) -> Vec<EventProperty> {
		std::mem::take(&mut self.r#event)
	}
	fn get_events(&self) -> &[EventsProperty] {
		self.r#events.as_slice()
	}
	fn take_events(&mut self) -> Vec<EventsProperty> {
		std::mem::take(&mut self.r#events)
	}
	fn get_fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty> {
		std::mem::take(&mut self.r#fax_number)
	}
	fn get_founder(&self) -> &[FounderProperty] {
		self.r#founder.as_slice()
	}
	fn take_founder(&mut self) -> Vec<FounderProperty> {
		std::mem::take(&mut self.r#founder)
	}
	fn get_founders(&self) -> &[FoundersProperty] {
		self.r#founders.as_slice()
	}
	fn take_founders(&mut self) -> Vec<FoundersProperty> {
		std::mem::take(&mut self.r#founders)
	}
	fn get_founding_date(&self) -> &[FoundingDateProperty] {
		self.r#founding_date.as_slice()
	}
	fn take_founding_date(&mut self) -> Vec<FoundingDateProperty> {
		std::mem::take(&mut self.r#founding_date)
	}
	fn get_founding_location(&self) -> &[FoundingLocationProperty] {
		self.r#founding_location.as_slice()
	}
	fn take_founding_location(&mut self) -> Vec<FoundingLocationProperty> {
		std::mem::take(&mut self.r#founding_location)
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
	fn get_has_gs_1_digital_link(&self) -> &[HasGs1DigitalLinkProperty] {
		self.r#has_gs_1_digital_link.as_slice()
	}
	fn take_has_gs_1_digital_link(&mut self) -> Vec<HasGs1DigitalLinkProperty> {
		std::mem::take(&mut self.r#has_gs_1_digital_link)
	}
	fn get_has_member_program(&self) -> &[HasMemberProgramProperty] {
		self.r#has_member_program.as_slice()
	}
	fn take_has_member_program(&mut self) -> Vec<HasMemberProgramProperty> {
		std::mem::take(&mut self.r#has_member_program)
	}
	fn get_has_merchant_return_policy(&self) -> &[HasMerchantReturnPolicyProperty] {
		self.r#has_merchant_return_policy.as_slice()
	}
	fn take_has_merchant_return_policy(&mut self) -> Vec<HasMerchantReturnPolicyProperty> {
		std::mem::take(&mut self.r#has_merchant_return_policy)
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
	fn get_has_product_return_policy(&self) -> &[HasProductReturnPolicyProperty] {
		self.r#has_product_return_policy.as_slice()
	}
	fn take_has_product_return_policy(&mut self) -> Vec<HasProductReturnPolicyProperty> {
		std::mem::take(&mut self.r#has_product_return_policy)
	}
	fn get_has_shipping_service(&self) -> &[HasShippingServiceProperty] {
		self.r#has_shipping_service.as_slice()
	}
	fn take_has_shipping_service(&mut self) -> Vec<HasShippingServiceProperty> {
		std::mem::take(&mut self.r#has_shipping_service)
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
	fn get_iso_6523_code(&self) -> &[Iso6523CodeProperty] {
		self.r#iso_6523_code.as_slice()
	}
	fn take_iso_6523_code(&mut self) -> Vec<Iso6523CodeProperty> {
		std::mem::take(&mut self.r#iso_6523_code)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
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
	fn get_legal_address(&self) -> &[LegalAddressProperty] {
		self.r#legal_address.as_slice()
	}
	fn take_legal_address(&mut self) -> Vec<LegalAddressProperty> {
		std::mem::take(&mut self.r#legal_address)
	}
	fn get_legal_name(&self) -> &[LegalNameProperty] {
		self.r#legal_name.as_slice()
	}
	fn take_legal_name(&mut self) -> Vec<LegalNameProperty> {
		std::mem::take(&mut self.r#legal_name)
	}
	fn get_legal_representative(&self) -> &[LegalRepresentativeProperty] {
		self.r#legal_representative.as_slice()
	}
	fn take_legal_representative(&mut self) -> Vec<LegalRepresentativeProperty> {
		std::mem::take(&mut self.r#legal_representative)
	}
	fn get_lei_code(&self) -> &[LeiCodeProperty] {
		self.r#lei_code.as_slice()
	}
	fn take_lei_code(&mut self) -> Vec<LeiCodeProperty> {
		std::mem::take(&mut self.r#lei_code)
	}
	fn get_location(&self) -> &[LocationProperty] {
		self.r#location.as_slice()
	}
	fn take_location(&mut self) -> Vec<LocationProperty> {
		std::mem::take(&mut self.r#location)
	}
	fn get_logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn take_logo(&mut self) -> Vec<LogoProperty> {
		std::mem::take(&mut self.r#logo)
	}
	fn get_makes_offer(&self) -> &[MakesOfferProperty] {
		self.r#makes_offer.as_slice()
	}
	fn take_makes_offer(&mut self) -> Vec<MakesOfferProperty> {
		std::mem::take(&mut self.r#makes_offer)
	}
	fn get_member(&self) -> &[MemberProperty] {
		self.r#member.as_slice()
	}
	fn take_member(&mut self) -> Vec<MemberProperty> {
		std::mem::take(&mut self.r#member)
	}
	fn get_member_of(&self) -> &[MemberOfProperty] {
		self.r#member_of.as_slice()
	}
	fn take_member_of(&mut self) -> Vec<MemberOfProperty> {
		std::mem::take(&mut self.r#member_of)
	}
	fn get_members(&self) -> &[MembersProperty] {
		self.r#members.as_slice()
	}
	fn take_members(&mut self) -> Vec<MembersProperty> {
		std::mem::take(&mut self.r#members)
	}
	fn get_naics(&self) -> &[NaicsProperty] {
		self.r#naics.as_slice()
	}
	fn take_naics(&mut self) -> Vec<NaicsProperty> {
		std::mem::take(&mut self.r#naics)
	}
	fn get_nonprofit_status(&self) -> &[NonprofitStatusProperty] {
		self.r#nonprofit_status.as_slice()
	}
	fn take_nonprofit_status(&mut self) -> Vec<NonprofitStatusProperty> {
		std::mem::take(&mut self.r#nonprofit_status)
	}
	fn get_number_of_employees(&self) -> &[NumberOfEmployeesProperty] {
		self.r#number_of_employees.as_slice()
	}
	fn take_number_of_employees(&mut self) -> Vec<NumberOfEmployeesProperty> {
		std::mem::take(&mut self.r#number_of_employees)
	}
	fn get_ownership_funding_info(&self) -> &[OwnershipFundingInfoProperty] {
		self.r#ownership_funding_info.as_slice()
	}
	fn take_ownership_funding_info(&mut self) -> Vec<OwnershipFundingInfoProperty> {
		std::mem::take(&mut self.r#ownership_funding_info)
	}
	fn get_owns(&self) -> &[OwnsProperty] {
		self.r#owns.as_slice()
	}
	fn take_owns(&mut self) -> Vec<OwnsProperty> {
		std::mem::take(&mut self.r#owns)
	}
	fn get_parent_organization(&self) -> &[ParentOrganizationProperty] {
		self.r#parent_organization.as_slice()
	}
	fn take_parent_organization(&mut self) -> Vec<ParentOrganizationProperty> {
		std::mem::take(&mut self.r#parent_organization)
	}
	fn get_publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn take_publishing_principles(&mut self) -> Vec<PublishingPrinciplesProperty> {
		std::mem::take(&mut self.r#publishing_principles)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn take_reviews(&mut self) -> Vec<ReviewsProperty> {
		std::mem::take(&mut self.r#reviews)
	}
	fn get_seeks(&self) -> &[SeeksProperty] {
		self.r#seeks.as_slice()
	}
	fn take_seeks(&mut self) -> Vec<SeeksProperty> {
		std::mem::take(&mut self.r#seeks)
	}
	fn get_service_area(&self) -> &[ServiceAreaProperty] {
		self.r#service_area.as_slice()
	}
	fn take_service_area(&mut self) -> Vec<ServiceAreaProperty> {
		std::mem::take(&mut self.r#service_area)
	}
	fn get_skills(&self) -> &[SkillsProperty] {
		self.r#skills.as_slice()
	}
	fn take_skills(&mut self) -> Vec<SkillsProperty> {
		std::mem::take(&mut self.r#skills)
	}
	fn get_slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn take_slogan(&mut self) -> Vec<SloganProperty> {
		std::mem::take(&mut self.r#slogan)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_sub_organization(&self) -> &[SubOrganizationProperty] {
		self.r#sub_organization.as_slice()
	}
	fn take_sub_organization(&mut self) -> Vec<SubOrganizationProperty> {
		std::mem::take(&mut self.r#sub_organization)
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
	fn get_unnamed_sources_policy(&self) -> &[UnnamedSourcesPolicyProperty] {
		self.r#unnamed_sources_policy.as_slice()
	}
	fn take_unnamed_sources_policy(&mut self) -> Vec<UnnamedSourcesPolicyProperty> {
		std::mem::take(&mut self.r#unnamed_sources_policy)
	}
	fn get_vat_id(&self) -> &[VatIdProperty] {
		self.r#vat_id.as_slice()
	}
	fn take_vat_id(&mut self) -> Vec<VatIdProperty> {
		std::mem::take(&mut self.r#vat_id)
	}
}
impl PlaceTrait for Bakery {
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty> {
		std::mem::take(&mut self.r#additional_property)
	}
	fn get_address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn take_address(&mut self) -> Vec<AddressProperty> {
		std::mem::take(&mut self.r#address)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_amenity_feature(&self) -> &[AmenityFeatureProperty] {
		self.r#amenity_feature.as_slice()
	}
	fn take_amenity_feature(&mut self) -> Vec<AmenityFeatureProperty> {
		std::mem::take(&mut self.r#amenity_feature)
	}
	fn get_branch_code(&self) -> &[BranchCodeProperty] {
		self.r#branch_code.as_slice()
	}
	fn take_branch_code(&mut self) -> Vec<BranchCodeProperty> {
		std::mem::take(&mut self.r#branch_code)
	}
	fn get_contained_in(&self) -> &[ContainedInProperty] {
		self.r#contained_in.as_slice()
	}
	fn take_contained_in(&mut self) -> Vec<ContainedInProperty> {
		std::mem::take(&mut self.r#contained_in)
	}
	fn get_contained_in_place(&self) -> &[ContainedInPlaceProperty] {
		self.r#contained_in_place.as_slice()
	}
	fn take_contained_in_place(&mut self) -> Vec<ContainedInPlaceProperty> {
		std::mem::take(&mut self.r#contained_in_place)
	}
	fn get_contains_place(&self) -> &[ContainsPlaceProperty] {
		self.r#contains_place.as_slice()
	}
	fn take_contains_place(&mut self) -> Vec<ContainsPlaceProperty> {
		std::mem::take(&mut self.r#contains_place)
	}
	fn get_event(&self) -> &[EventProperty] {
		self.r#event.as_slice()
	}
	fn take_event(&mut self) -> Vec<EventProperty> {
		std::mem::take(&mut self.r#event)
	}
	fn get_events(&self) -> &[EventsProperty] {
		self.r#events.as_slice()
	}
	fn take_events(&mut self) -> Vec<EventsProperty> {
		std::mem::take(&mut self.r#events)
	}
	fn get_fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty> {
		std::mem::take(&mut self.r#fax_number)
	}
	fn get_geo(&self) -> &[GeoProperty] {
		self.r#geo.as_slice()
	}
	fn take_geo(&mut self) -> Vec<GeoProperty> {
		std::mem::take(&mut self.r#geo)
	}
	fn get_geo_contains(&self) -> &[GeoContainsProperty] {
		self.r#geo_contains.as_slice()
	}
	fn take_geo_contains(&mut self) -> Vec<GeoContainsProperty> {
		std::mem::take(&mut self.r#geo_contains)
	}
	fn get_geo_covered_by(&self) -> &[GeoCoveredByProperty] {
		self.r#geo_covered_by.as_slice()
	}
	fn take_geo_covered_by(&mut self) -> Vec<GeoCoveredByProperty> {
		std::mem::take(&mut self.r#geo_covered_by)
	}
	fn get_geo_covers(&self) -> &[GeoCoversProperty] {
		self.r#geo_covers.as_slice()
	}
	fn take_geo_covers(&mut self) -> Vec<GeoCoversProperty> {
		std::mem::take(&mut self.r#geo_covers)
	}
	fn get_geo_crosses(&self) -> &[GeoCrossesProperty] {
		self.r#geo_crosses.as_slice()
	}
	fn take_geo_crosses(&mut self) -> Vec<GeoCrossesProperty> {
		std::mem::take(&mut self.r#geo_crosses)
	}
	fn get_geo_disjoint(&self) -> &[GeoDisjointProperty] {
		self.r#geo_disjoint.as_slice()
	}
	fn take_geo_disjoint(&mut self) -> Vec<GeoDisjointProperty> {
		std::mem::take(&mut self.r#geo_disjoint)
	}
	fn get_geo_equals(&self) -> &[GeoEqualsProperty] {
		self.r#geo_equals.as_slice()
	}
	fn take_geo_equals(&mut self) -> Vec<GeoEqualsProperty> {
		std::mem::take(&mut self.r#geo_equals)
	}
	fn get_geo_intersects(&self) -> &[GeoIntersectsProperty] {
		self.r#geo_intersects.as_slice()
	}
	fn take_geo_intersects(&mut self) -> Vec<GeoIntersectsProperty> {
		std::mem::take(&mut self.r#geo_intersects)
	}
	fn get_geo_overlaps(&self) -> &[GeoOverlapsProperty] {
		self.r#geo_overlaps.as_slice()
	}
	fn take_geo_overlaps(&mut self) -> Vec<GeoOverlapsProperty> {
		std::mem::take(&mut self.r#geo_overlaps)
	}
	fn get_geo_touches(&self) -> &[GeoTouchesProperty] {
		self.r#geo_touches.as_slice()
	}
	fn take_geo_touches(&mut self) -> Vec<GeoTouchesProperty> {
		std::mem::take(&mut self.r#geo_touches)
	}
	fn get_geo_within(&self) -> &[GeoWithinProperty] {
		self.r#geo_within.as_slice()
	}
	fn take_geo_within(&mut self) -> Vec<GeoWithinProperty> {
		std::mem::take(&mut self.r#geo_within)
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
	fn get_has_drive_through_service(&self) -> &[HasDriveThroughServiceProperty] {
		self.r#has_drive_through_service.as_slice()
	}
	fn take_has_drive_through_service(&mut self) -> Vec<HasDriveThroughServiceProperty> {
		std::mem::take(&mut self.r#has_drive_through_service)
	}
	fn get_has_gs_1_digital_link(&self) -> &[HasGs1DigitalLinkProperty] {
		self.r#has_gs_1_digital_link.as_slice()
	}
	fn take_has_gs_1_digital_link(&mut self) -> Vec<HasGs1DigitalLinkProperty> {
		std::mem::take(&mut self.r#has_gs_1_digital_link)
	}
	fn get_has_map(&self) -> &[HasMapProperty] {
		self.r#has_map.as_slice()
	}
	fn take_has_map(&mut self) -> Vec<HasMapProperty> {
		std::mem::take(&mut self.r#has_map)
	}
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty> {
		std::mem::take(&mut self.r#is_accessible_for_free)
	}
	fn get_isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn take_isic_v_4(&mut self) -> Vec<IsicV4Property> {
		std::mem::take(&mut self.r#isic_v_4)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_latitude(&self) -> &[LatitudeProperty] {
		self.r#latitude.as_slice()
	}
	fn take_latitude(&mut self) -> Vec<LatitudeProperty> {
		std::mem::take(&mut self.r#latitude)
	}
	fn get_logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn take_logo(&mut self) -> Vec<LogoProperty> {
		std::mem::take(&mut self.r#logo)
	}
	fn get_longitude(&self) -> &[LongitudeProperty] {
		self.r#longitude.as_slice()
	}
	fn take_longitude(&mut self) -> Vec<LongitudeProperty> {
		std::mem::take(&mut self.r#longitude)
	}
	fn get_map(&self) -> &[MapProperty] {
		self.r#map.as_slice()
	}
	fn take_map(&mut self) -> Vec<MapProperty> {
		std::mem::take(&mut self.r#map)
	}
	fn get_maps(&self) -> &[MapsProperty] {
		self.r#maps.as_slice()
	}
	fn take_maps(&mut self) -> Vec<MapsProperty> {
		std::mem::take(&mut self.r#maps)
	}
	fn get_maximum_attendee_capacity(&self) -> &[MaximumAttendeeCapacityProperty] {
		self.r#maximum_attendee_capacity.as_slice()
	}
	fn take_maximum_attendee_capacity(&mut self) -> Vec<MaximumAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_attendee_capacity)
	}
	fn get_opening_hours_specification(&self) -> &[OpeningHoursSpecificationProperty] {
		self.r#opening_hours_specification.as_slice()
	}
	fn take_opening_hours_specification(&mut self) -> Vec<OpeningHoursSpecificationProperty> {
		std::mem::take(&mut self.r#opening_hours_specification)
	}
	fn get_photo(&self) -> &[PhotoProperty] {
		self.r#photo.as_slice()
	}
	fn take_photo(&mut self) -> Vec<PhotoProperty> {
		std::mem::take(&mut self.r#photo)
	}
	fn get_photos(&self) -> &[PhotosProperty] {
		self.r#photos.as_slice()
	}
	fn take_photos(&mut self) -> Vec<PhotosProperty> {
		std::mem::take(&mut self.r#photos)
	}
	fn get_public_access(&self) -> &[PublicAccessProperty] {
		self.r#public_access.as_slice()
	}
	fn take_public_access(&mut self) -> Vec<PublicAccessProperty> {
		std::mem::take(&mut self.r#public_access)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn take_reviews(&mut self) -> Vec<ReviewsProperty> {
		std::mem::take(&mut self.r#reviews)
	}
	fn get_slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn take_slogan(&mut self) -> Vec<SloganProperty> {
		std::mem::take(&mut self.r#slogan)
	}
	fn get_smoking_allowed(&self) -> &[SmokingAllowedProperty] {
		self.r#smoking_allowed.as_slice()
	}
	fn take_smoking_allowed(&mut self) -> Vec<SmokingAllowedProperty> {
		std::mem::take(&mut self.r#smoking_allowed)
	}
	fn get_special_opening_hours_specification(
		&self,
	) -> &[SpecialOpeningHoursSpecificationProperty] {
		self.r#special_opening_hours_specification.as_slice()
	}
	fn take_special_opening_hours_specification(
		&mut self,
	) -> Vec<SpecialOpeningHoursSpecificationProperty> {
		std::mem::take(&mut self.r#special_opening_hours_specification)
	}
	fn get_telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn take_telephone(&mut self) -> Vec<TelephoneProperty> {
		std::mem::take(&mut self.r#telephone)
	}
	fn get_tour_booking_page(&self) -> &[TourBookingPageProperty] {
		self.r#tour_booking_page.as_slice()
	}
	fn take_tour_booking_page(&mut self) -> Vec<TourBookingPageProperty> {
		std::mem::take(&mut self.r#tour_booking_page)
	}
}
impl ThingTrait for Bakery {
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
