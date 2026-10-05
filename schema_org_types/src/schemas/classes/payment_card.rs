use super::*;
/// <https://schema.org/PaymentCard>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PaymentCard {
	/// <https://schema.org/cashBack>
	#[cfg_attr(feature = "serde", serde(rename = "cashBack"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cash_back: Vec<CashBackProperty>,
	/// <https://schema.org/contactlessPayment>
	#[cfg_attr(feature = "serde", serde(rename = "contactlessPayment"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contactless_payment: Vec<ContactlessPaymentProperty>,
	/// <https://schema.org/floorLimit>
	#[cfg_attr(feature = "serde", serde(rename = "floorLimit"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#floor_limit: Vec<FloorLimitProperty>,
	/// <https://schema.org/monthlyMinimumRepaymentAmount>
	#[cfg_attr(feature = "serde", serde(rename = "monthlyMinimumRepaymentAmount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#monthly_minimum_repayment_amount: Vec<MonthlyMinimumRepaymentAmountProperty>,
	/// <https://schema.org/annualPercentageRate>
	#[cfg_attr(feature = "serde", serde(rename = "annualPercentageRate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#annual_percentage_rate: Vec<AnnualPercentageRateProperty>,
	/// <https://schema.org/feesAndCommissionsSpecification>
	#[cfg_attr(feature = "serde", serde(rename = "feesAndCommissionsSpecification"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#fees_and_commissions_specification: Vec<FeesAndCommissionsSpecificationProperty>,
	/// <https://schema.org/interestRate>
	#[cfg_attr(feature = "serde", serde(rename = "interestRate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#interest_rate: Vec<InterestRateProperty>,
	/// <https://schema.org/paymentMethodType>
	#[cfg_attr(feature = "serde", serde(rename = "paymentMethodType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_method_type: Vec<PaymentMethodTypeProperty>,
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
	/// <https://schema.org/availableChannel>
	#[cfg_attr(feature = "serde", serde(rename = "availableChannel"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#available_channel: Vec<AvailableChannelProperty>,
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
	/// <https://schema.org/broker>
	#[cfg_attr(feature = "serde", serde(rename = "broker"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#broker: Vec<BrokerProperty>,
	/// <https://schema.org/category>
	#[cfg_attr(feature = "serde", serde(rename = "category"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#category: Vec<CategoryProperty>,
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
	/// <https://schema.org/hoursAvailable>
	#[cfg_attr(feature = "serde", serde(rename = "hoursAvailable"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#hours_available: Vec<HoursAvailableProperty>,
	/// <https://schema.org/isRelatedTo>
	#[cfg_attr(feature = "serde", serde(rename = "isRelatedTo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_related_to: Vec<IsRelatedToProperty>,
	/// <https://schema.org/isSimilarTo>
	#[cfg_attr(feature = "serde", serde(rename = "isSimilarTo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_similar_to: Vec<IsSimilarToProperty>,
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
	/// <https://schema.org/produces>
	#[deprecated = "This schema is superseded by <https://schema.org/serviceOutput>."]
	#[cfg_attr(feature = "serde", serde(rename = "produces"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#produces: Vec<ProducesProperty>,
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
	/// <https://schema.org/providerMobility>
	#[cfg_attr(feature = "serde", serde(rename = "providerMobility"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#provider_mobility: Vec<ProviderMobilityProperty>,
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
	/// <https://schema.org/serviceAudience>
	#[deprecated = "This schema is superseded by <https://schema.org/audience>."]
	#[cfg_attr(feature = "serde", serde(rename = "serviceAudience"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#service_audience: Vec<ServiceAudienceProperty>,
	/// <https://schema.org/serviceOutput>
	#[cfg_attr(feature = "serde", serde(rename = "serviceOutput"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#service_output: Vec<ServiceOutputProperty>,
	/// <https://schema.org/serviceType>
	#[cfg_attr(feature = "serde", serde(rename = "serviceType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#service_type: Vec<ServiceTypeProperty>,
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
	/// <https://schema.org/termsOfService>
	#[cfg_attr(feature = "serde", serde(rename = "termsOfService"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#terms_of_service: Vec<TermsOfServiceProperty>,
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
/// This trait is for properties from <https://schema.org/PaymentCard>.
pub trait PaymentCardTrait {
	/// Get <https://schema.org/cashBack> from [`Self`] as borrowed slice.
	fn r#cash_back(&self) -> &[CashBackProperty];
	/// Get <https://schema.org/contactlessPayment> from [`Self`] as borrowed slice.
	fn r#contactless_payment(&self) -> &[ContactlessPaymentProperty];
	/// Get <https://schema.org/floorLimit> from [`Self`] as borrowed slice.
	fn r#floor_limit(&self) -> &[FloorLimitProperty];
	/// Get <https://schema.org/monthlyMinimumRepaymentAmount> from [`Self`] as borrowed slice.
	fn r#monthly_minimum_repayment_amount(&self) -> &[MonthlyMinimumRepaymentAmountProperty];
}
impl PaymentCardTrait for PaymentCard {
	fn r#cash_back(&self) -> &[CashBackProperty] {
		self.r#cash_back.as_slice()
	}
	fn r#contactless_payment(&self) -> &[ContactlessPaymentProperty] {
		self.r#contactless_payment.as_slice()
	}
	fn r#floor_limit(&self) -> &[FloorLimitProperty] {
		self.r#floor_limit.as_slice()
	}
	fn r#monthly_minimum_repayment_amount(&self) -> &[MonthlyMinimumRepaymentAmountProperty] {
		self.r#monthly_minimum_repayment_amount.as_slice()
	}
}
impl FinancialProductTrait for PaymentCard {
	fn r#annual_percentage_rate(&self) -> &[AnnualPercentageRateProperty] {
		self.r#annual_percentage_rate.as_slice()
	}
	fn r#fees_and_commissions_specification(&self) -> &[FeesAndCommissionsSpecificationProperty] {
		self.r#fees_and_commissions_specification.as_slice()
	}
	fn r#interest_rate(&self) -> &[InterestRateProperty] {
		self.r#interest_rate.as_slice()
	}
}
impl PaymentMethodTrait for PaymentCard {
	fn r#payment_method_type(&self) -> &[PaymentMethodTypeProperty] {
		self.r#payment_method_type.as_slice()
	}
}
impl ServiceTrait for PaymentCard {
	fn r#aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn r#area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn r#audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn r#available_channel(&self) -> &[AvailableChannelProperty] {
		self.r#available_channel.as_slice()
	}
	fn r#award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn r#brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn r#broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn r#category(&self) -> &[CategoryProperty] {
		self.r#category.as_slice()
	}
	fn r#has_certification(&self) -> &[HasCertificationProperty] {
		self.r#has_certification.as_slice()
	}
	fn r#has_offer_catalog(&self) -> &[HasOfferCatalogProperty] {
		self.r#has_offer_catalog.as_slice()
	}
	fn r#hours_available(&self) -> &[HoursAvailableProperty] {
		self.r#hours_available.as_slice()
	}
	fn r#is_related_to(&self) -> &[IsRelatedToProperty] {
		self.r#is_related_to.as_slice()
	}
	fn r#is_similar_to(&self) -> &[IsSimilarToProperty] {
		self.r#is_similar_to.as_slice()
	}
	fn r#logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn r#offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn r#produces(&self) -> &[ProducesProperty] {
		self.r#produces.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#provider_mobility(&self) -> &[ProviderMobilityProperty] {
		self.r#provider_mobility.as_slice()
	}
	fn r#review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn r#service_area(&self) -> &[ServiceAreaProperty] {
		self.r#service_area.as_slice()
	}
	fn r#service_audience(&self) -> &[ServiceAudienceProperty] {
		self.r#service_audience.as_slice()
	}
	fn r#service_output(&self) -> &[ServiceOutputProperty] {
		self.r#service_output.as_slice()
	}
	fn r#service_type(&self) -> &[ServiceTypeProperty] {
		self.r#service_type.as_slice()
	}
	fn r#slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn r#terms_of_service(&self) -> &[TermsOfServiceProperty] {
		self.r#terms_of_service.as_slice()
	}
}
impl ThingTrait for PaymentCard {
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
