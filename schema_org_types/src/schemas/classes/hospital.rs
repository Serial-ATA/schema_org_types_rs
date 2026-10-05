use super::*;
/// <https://schema.org/Hospital>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Hospital {
	/// <https://schema.org/availableService>
	#[cfg_attr(feature = "serde", serde(rename = "availableService"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#available_service: Vec<AvailableServiceProperty>,
	/// <https://schema.org/healthcareReportingData>
	#[cfg_attr(feature = "serde", serde(rename = "healthcareReportingData"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#healthcare_reporting_data: Vec<HealthcareReportingDataProperty>,
	/// <https://schema.org/medicalSpecialty>
	#[cfg_attr(feature = "serde", serde(rename = "medicalSpecialty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#medical_specialty: Vec<MedicalSpecialtyProperty>,
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
	/// <https://schema.org/healthPlanNetworkId>
	#[cfg_attr(feature = "serde", serde(rename = "healthPlanNetworkId"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#health_plan_network_id: Vec<HealthPlanNetworkIdProperty>,
	/// <https://schema.org/isAcceptingNewPatients>
	#[cfg_attr(feature = "serde", serde(rename = "isAcceptingNewPatients"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_accepting_new_patients: Vec<IsAcceptingNewPatientsProperty>,
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
/// This trait is for properties from <https://schema.org/Hospital>.
pub trait HospitalTrait {
	/// Get <https://schema.org/availableService> from [`Self`] as borrowed slice.
	fn r#available_service(&self) -> &[AvailableServiceProperty];
	/// Get <https://schema.org/healthcareReportingData> from [`Self`] as borrowed slice.
	fn r#healthcare_reporting_data(&self) -> &[HealthcareReportingDataProperty];
	/// Get <https://schema.org/medicalSpecialty> from [`Self`] as borrowed slice.
	fn r#medical_specialty(&self) -> &[MedicalSpecialtyProperty];
}
impl HospitalTrait for Hospital {
	fn r#available_service(&self) -> &[AvailableServiceProperty] {
		self.r#available_service.as_slice()
	}
	fn r#healthcare_reporting_data(&self) -> &[HealthcareReportingDataProperty] {
		self.r#healthcare_reporting_data.as_slice()
	}
	fn r#medical_specialty(&self) -> &[MedicalSpecialtyProperty] {
		self.r#medical_specialty.as_slice()
	}
}
impl CivicStructureTrait for Hospital {
	fn r#opening_hours(&self) -> &[OpeningHoursProperty] {
		self.r#opening_hours.as_slice()
	}
}
impl EmergencyServiceTrait for Hospital {}
impl LocalBusinessTrait for Hospital {
	fn r#branch_of(&self) -> &[BranchOfProperty] {
		self.r#branch_of.as_slice()
	}
	fn r#currencies_accepted(&self) -> &[CurrenciesAcceptedProperty] {
		self.r#currencies_accepted.as_slice()
	}
	fn r#floor_level(&self) -> &[FloorLevelProperty] {
		self.r#floor_level.as_slice()
	}
	fn r#opening_hours(&self) -> &[OpeningHoursProperty] {
		self.r#opening_hours.as_slice()
	}
	fn r#payment_accepted(&self) -> &[PaymentAcceptedProperty] {
		self.r#payment_accepted.as_slice()
	}
	fn r#price_range(&self) -> &[PriceRangeProperty] {
		self.r#price_range.as_slice()
	}
}
impl MedicalOrganizationTrait for Hospital {
	fn r#health_plan_network_id(&self) -> &[HealthPlanNetworkIdProperty] {
		self.r#health_plan_network_id.as_slice()
	}
	fn r#is_accepting_new_patients(&self) -> &[IsAcceptingNewPatientsProperty] {
		self.r#is_accepting_new_patients.as_slice()
	}
	fn r#medical_specialty(&self) -> &[MedicalSpecialtyProperty] {
		self.r#medical_specialty.as_slice()
	}
}
impl OrganizationTrait for Hospital {
	fn r#accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty] {
		self.r#accepted_payment_method.as_slice()
	}
	fn r#actionable_feedback_policy(&self) -> &[ActionableFeedbackPolicyProperty] {
		self.r#actionable_feedback_policy.as_slice()
	}
	fn r#address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn r#agent_interaction_statistic(&self) -> &[AgentInteractionStatisticProperty] {
		self.r#agent_interaction_statistic.as_slice()
	}
	fn r#aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn r#alumni(&self) -> &[AlumniProperty] {
		self.r#alumni.as_slice()
	}
	fn r#area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn r#authorized_representative(&self) -> &[AuthorizedRepresentativeProperty] {
		self.r#authorized_representative.as_slice()
	}
	fn r#award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn r#awards(&self) -> &[AwardsProperty] {
		self.r#awards.as_slice()
	}
	fn r#brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn r#company_registration(&self) -> &[CompanyRegistrationProperty] {
		self.r#company_registration.as_slice()
	}
	fn r#contact_point(&self) -> &[ContactPointProperty] {
		self.r#contact_point.as_slice()
	}
	fn r#contact_points(&self) -> &[ContactPointsProperty] {
		self.r#contact_points.as_slice()
	}
	fn r#corrections_policy(&self) -> &[CorrectionsPolicyProperty] {
		self.r#corrections_policy.as_slice()
	}
	fn r#department(&self) -> &[DepartmentProperty] {
		self.r#department.as_slice()
	}
	fn r#dissolution_date(&self) -> &[DissolutionDateProperty] {
		self.r#dissolution_date.as_slice()
	}
	fn r#diversity_policy(&self) -> &[DiversityPolicyProperty] {
		self.r#diversity_policy.as_slice()
	}
	fn r#diversity_staffing_report(&self) -> &[DiversityStaffingReportProperty] {
		self.r#diversity_staffing_report.as_slice()
	}
	fn r#duns(&self) -> &[DunsProperty] {
		self.r#duns.as_slice()
	}
	fn r#email(&self) -> &[EmailProperty] {
		self.r#email.as_slice()
	}
	fn r#employee(&self) -> &[EmployeeProperty] {
		self.r#employee.as_slice()
	}
	fn r#employees(&self) -> &[EmployeesProperty] {
		self.r#employees.as_slice()
	}
	fn r#ethics_policy(&self) -> &[EthicsPolicyProperty] {
		self.r#ethics_policy.as_slice()
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
	fn r#founder(&self) -> &[FounderProperty] {
		self.r#founder.as_slice()
	}
	fn r#founders(&self) -> &[FoundersProperty] {
		self.r#founders.as_slice()
	}
	fn r#founding_date(&self) -> &[FoundingDateProperty] {
		self.r#founding_date.as_slice()
	}
	fn r#founding_location(&self) -> &[FoundingLocationProperty] {
		self.r#founding_location.as_slice()
	}
	fn r#funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn r#funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
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
	fn r#has_gs_1_digital_link(&self) -> &[HasGs1DigitalLinkProperty] {
		self.r#has_gs_1_digital_link.as_slice()
	}
	fn r#has_member_program(&self) -> &[HasMemberProgramProperty] {
		self.r#has_member_program.as_slice()
	}
	fn r#has_merchant_return_policy(&self) -> &[HasMerchantReturnPolicyProperty] {
		self.r#has_merchant_return_policy.as_slice()
	}
	fn r#has_offer_catalog(&self) -> &[HasOfferCatalogProperty] {
		self.r#has_offer_catalog.as_slice()
	}
	fn r#has_pos(&self) -> &[HasPosProperty] {
		self.r#has_pos.as_slice()
	}
	fn r#has_product_return_policy(&self) -> &[HasProductReturnPolicyProperty] {
		self.r#has_product_return_policy.as_slice()
	}
	fn r#has_shipping_service(&self) -> &[HasShippingServiceProperty] {
		self.r#has_shipping_service.as_slice()
	}
	fn r#interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn r#isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn r#iso_6523_code(&self) -> &[Iso6523CodeProperty] {
		self.r#iso_6523_code.as_slice()
	}
	fn r#keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn r#knows_about(&self) -> &[KnowsAboutProperty] {
		self.r#knows_about.as_slice()
	}
	fn r#knows_language(&self) -> &[KnowsLanguageProperty] {
		self.r#knows_language.as_slice()
	}
	fn r#legal_address(&self) -> &[LegalAddressProperty] {
		self.r#legal_address.as_slice()
	}
	fn r#legal_name(&self) -> &[LegalNameProperty] {
		self.r#legal_name.as_slice()
	}
	fn r#legal_representative(&self) -> &[LegalRepresentativeProperty] {
		self.r#legal_representative.as_slice()
	}
	fn r#lei_code(&self) -> &[LeiCodeProperty] {
		self.r#lei_code.as_slice()
	}
	fn r#location(&self) -> &[LocationProperty] {
		self.r#location.as_slice()
	}
	fn r#logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn r#makes_offer(&self) -> &[MakesOfferProperty] {
		self.r#makes_offer.as_slice()
	}
	fn r#member(&self) -> &[MemberProperty] {
		self.r#member.as_slice()
	}
	fn r#member_of(&self) -> &[MemberOfProperty] {
		self.r#member_of.as_slice()
	}
	fn r#members(&self) -> &[MembersProperty] {
		self.r#members.as_slice()
	}
	fn r#naics(&self) -> &[NaicsProperty] {
		self.r#naics.as_slice()
	}
	fn r#nonprofit_status(&self) -> &[NonprofitStatusProperty] {
		self.r#nonprofit_status.as_slice()
	}
	fn r#number_of_employees(&self) -> &[NumberOfEmployeesProperty] {
		self.r#number_of_employees.as_slice()
	}
	fn r#ownership_funding_info(&self) -> &[OwnershipFundingInfoProperty] {
		self.r#ownership_funding_info.as_slice()
	}
	fn r#owns(&self) -> &[OwnsProperty] {
		self.r#owns.as_slice()
	}
	fn r#parent_organization(&self) -> &[ParentOrganizationProperty] {
		self.r#parent_organization.as_slice()
	}
	fn r#publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn r#review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn r#reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn r#seeks(&self) -> &[SeeksProperty] {
		self.r#seeks.as_slice()
	}
	fn r#service_area(&self) -> &[ServiceAreaProperty] {
		self.r#service_area.as_slice()
	}
	fn r#skills(&self) -> &[SkillsProperty] {
		self.r#skills.as_slice()
	}
	fn r#slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn r#sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn r#sub_organization(&self) -> &[SubOrganizationProperty] {
		self.r#sub_organization.as_slice()
	}
	fn r#tax_id(&self) -> &[TaxIdProperty] {
		self.r#tax_id.as_slice()
	}
	fn r#telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn r#unnamed_sources_policy(&self) -> &[UnnamedSourcesPolicyProperty] {
		self.r#unnamed_sources_policy.as_slice()
	}
	fn r#vat_id(&self) -> &[VatIdProperty] {
		self.r#vat_id.as_slice()
	}
}
impl PlaceTrait for Hospital {
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
impl ThingTrait for Hospital {
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
