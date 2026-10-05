use super::*;
/// <https://schema.org/Demand>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Demand {
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
	/// <https://schema.org/advanceBookingRequirement>
	#[cfg_attr(feature = "serde", serde(rename = "advanceBookingRequirement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#advance_booking_requirement: Vec<AdvanceBookingRequirementProperty>,
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
	/// <https://schema.org/asin>
	#[cfg_attr(feature = "serde", serde(rename = "asin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#asin: Vec<AsinProperty>,
	/// <https://schema.org/availability>
	#[cfg_attr(feature = "serde", serde(rename = "availability"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#availability: Vec<AvailabilityProperty>,
	/// <https://schema.org/availabilityEnds>
	#[cfg_attr(feature = "serde", serde(rename = "availabilityEnds"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#availability_ends: Vec<AvailabilityEndsProperty>,
	/// <https://schema.org/availabilityStarts>
	#[cfg_attr(feature = "serde", serde(rename = "availabilityStarts"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#availability_starts: Vec<AvailabilityStartsProperty>,
	/// <https://schema.org/availableAtOrFrom>
	#[cfg_attr(feature = "serde", serde(rename = "availableAtOrFrom"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#available_at_or_from: Vec<AvailableAtOrFromProperty>,
	/// <https://schema.org/availableDeliveryMethod>
	#[cfg_attr(feature = "serde", serde(rename = "availableDeliveryMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#available_delivery_method: Vec<AvailableDeliveryMethodProperty>,
	/// <https://schema.org/businessFunction>
	#[cfg_attr(feature = "serde", serde(rename = "businessFunction"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#business_function: Vec<BusinessFunctionProperty>,
	/// <https://schema.org/deliveryLeadTime>
	#[cfg_attr(feature = "serde", serde(rename = "deliveryLeadTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#delivery_lead_time: Vec<DeliveryLeadTimeProperty>,
	/// <https://schema.org/eligibleCustomerType>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleCustomerType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_customer_type: Vec<EligibleCustomerTypeProperty>,
	/// <https://schema.org/eligibleDuration>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleDuration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_duration: Vec<EligibleDurationProperty>,
	/// <https://schema.org/eligibleQuantity>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleQuantity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_quantity: Vec<EligibleQuantityProperty>,
	/// <https://schema.org/eligibleRegion>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleRegion"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_region: Vec<EligibleRegionProperty>,
	/// <https://schema.org/eligibleTransactionVolume>
	#[cfg_attr(feature = "serde", serde(rename = "eligibleTransactionVolume"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#eligible_transaction_volume: Vec<EligibleTransactionVolumeProperty>,
	/// <https://schema.org/gtin>
	#[cfg_attr(feature = "serde", serde(rename = "gtin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#gtin: Vec<GtinProperty>,
	/// <https://schema.org/gtin12>
	#[cfg_attr(feature = "serde", serde(rename = "gtin12"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#gtin_12: Vec<Gtin12Property>,
	/// <https://schema.org/gtin13>
	#[cfg_attr(feature = "serde", serde(rename = "gtin13"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#gtin_13: Vec<Gtin13Property>,
	/// <https://schema.org/gtin14>
	#[cfg_attr(feature = "serde", serde(rename = "gtin14"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#gtin_14: Vec<Gtin14Property>,
	/// <https://schema.org/gtin8>
	#[cfg_attr(feature = "serde", serde(rename = "gtin8"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#gtin_8: Vec<Gtin8Property>,
	/// <https://schema.org/includesObject>
	#[cfg_attr(feature = "serde", serde(rename = "includesObject"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#includes_object: Vec<IncludesObjectProperty>,
	/// <https://schema.org/ineligibleRegion>
	#[cfg_attr(feature = "serde", serde(rename = "ineligibleRegion"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ineligible_region: Vec<IneligibleRegionProperty>,
	/// <https://schema.org/inventoryLevel>
	#[cfg_attr(feature = "serde", serde(rename = "inventoryLevel"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#inventory_level: Vec<InventoryLevelProperty>,
	/// <https://schema.org/itemCondition>
	#[cfg_attr(feature = "serde", serde(rename = "itemCondition"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_condition: Vec<ItemConditionProperty>,
	/// <https://schema.org/itemOffered>
	#[cfg_attr(feature = "serde", serde(rename = "itemOffered"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_offered: Vec<ItemOfferedProperty>,
	/// <https://schema.org/mpn>
	#[cfg_attr(feature = "serde", serde(rename = "mpn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#mpn: Vec<MpnProperty>,
	/// <https://schema.org/priceSpecification>
	#[cfg_attr(feature = "serde", serde(rename = "priceSpecification"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#price_specification: Vec<PriceSpecificationProperty>,
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
	/// <https://schema.org/serialNumber>
	#[cfg_attr(feature = "serde", serde(rename = "serialNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#serial_number: Vec<SerialNumberProperty>,
	/// <https://schema.org/sku>
	#[cfg_attr(feature = "serde", serde(rename = "sku"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sku: Vec<SkuProperty>,
	/// <https://schema.org/validFrom>
	#[cfg_attr(feature = "serde", serde(rename = "validFrom"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#valid_from: Vec<ValidFromProperty>,
	/// <https://schema.org/validThrough>
	#[cfg_attr(feature = "serde", serde(rename = "validThrough"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#valid_through: Vec<ValidThroughProperty>,
	/// <https://schema.org/warranty>
	#[cfg_attr(feature = "serde", serde(rename = "warranty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#warranty: Vec<WarrantyProperty>,
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
/// This trait is for properties from <https://schema.org/Demand>.
pub trait DemandTrait {
	/// Get <https://schema.org/acceptedPaymentMethod> from [`Self`] as borrowed slice.
	fn r#accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty];
	/// Get <https://schema.org/advanceBookingRequirement> from [`Self`] as borrowed slice.
	fn r#advance_booking_requirement(&self) -> &[AdvanceBookingRequirementProperty];
	/// Get <https://schema.org/areaServed> from [`Self`] as borrowed slice.
	fn r#area_served(&self) -> &[AreaServedProperty];
	/// Get <https://schema.org/asin> from [`Self`] as borrowed slice.
	fn r#asin(&self) -> &[AsinProperty];
	/// Get <https://schema.org/availability> from [`Self`] as borrowed slice.
	fn r#availability(&self) -> &[AvailabilityProperty];
	/// Get <https://schema.org/availabilityEnds> from [`Self`] as borrowed slice.
	fn r#availability_ends(&self) -> &[AvailabilityEndsProperty];
	/// Get <https://schema.org/availabilityStarts> from [`Self`] as borrowed slice.
	fn r#availability_starts(&self) -> &[AvailabilityStartsProperty];
	/// Get <https://schema.org/availableAtOrFrom> from [`Self`] as borrowed slice.
	fn r#available_at_or_from(&self) -> &[AvailableAtOrFromProperty];
	/// Get <https://schema.org/availableDeliveryMethod> from [`Self`] as borrowed slice.
	fn r#available_delivery_method(&self) -> &[AvailableDeliveryMethodProperty];
	/// Get <https://schema.org/businessFunction> from [`Self`] as borrowed slice.
	fn r#business_function(&self) -> &[BusinessFunctionProperty];
	/// Get <https://schema.org/deliveryLeadTime> from [`Self`] as borrowed slice.
	fn r#delivery_lead_time(&self) -> &[DeliveryLeadTimeProperty];
	/// Get <https://schema.org/eligibleCustomerType> from [`Self`] as borrowed slice.
	fn r#eligible_customer_type(&self) -> &[EligibleCustomerTypeProperty];
	/// Get <https://schema.org/eligibleDuration> from [`Self`] as borrowed slice.
	fn r#eligible_duration(&self) -> &[EligibleDurationProperty];
	/// Get <https://schema.org/eligibleQuantity> from [`Self`] as borrowed slice.
	fn r#eligible_quantity(&self) -> &[EligibleQuantityProperty];
	/// Get <https://schema.org/eligibleRegion> from [`Self`] as borrowed slice.
	fn r#eligible_region(&self) -> &[EligibleRegionProperty];
	/// Get <https://schema.org/eligibleTransactionVolume> from [`Self`] as borrowed slice.
	fn r#eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty];
	/// Get <https://schema.org/gtin> from [`Self`] as borrowed slice.
	fn r#gtin(&self) -> &[GtinProperty];
	/// Get <https://schema.org/gtin12> from [`Self`] as borrowed slice.
	fn r#gtin_12(&self) -> &[Gtin12Property];
	/// Get <https://schema.org/gtin13> from [`Self`] as borrowed slice.
	fn r#gtin_13(&self) -> &[Gtin13Property];
	/// Get <https://schema.org/gtin14> from [`Self`] as borrowed slice.
	fn r#gtin_14(&self) -> &[Gtin14Property];
	/// Get <https://schema.org/gtin8> from [`Self`] as borrowed slice.
	fn r#gtin_8(&self) -> &[Gtin8Property];
	/// Get <https://schema.org/includesObject> from [`Self`] as borrowed slice.
	fn r#includes_object(&self) -> &[IncludesObjectProperty];
	/// Get <https://schema.org/ineligibleRegion> from [`Self`] as borrowed slice.
	fn r#ineligible_region(&self) -> &[IneligibleRegionProperty];
	/// Get <https://schema.org/inventoryLevel> from [`Self`] as borrowed slice.
	fn r#inventory_level(&self) -> &[InventoryLevelProperty];
	/// Get <https://schema.org/itemCondition> from [`Self`] as borrowed slice.
	fn r#item_condition(&self) -> &[ItemConditionProperty];
	/// Get <https://schema.org/itemOffered> from [`Self`] as borrowed slice.
	fn r#item_offered(&self) -> &[ItemOfferedProperty];
	/// Get <https://schema.org/mpn> from [`Self`] as borrowed slice.
	fn r#mpn(&self) -> &[MpnProperty];
	/// Get <https://schema.org/priceSpecification> from [`Self`] as borrowed slice.
	fn r#price_specification(&self) -> &[PriceSpecificationProperty];
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn r#seller(&self) -> &[SellerProperty];
	/// Get <https://schema.org/serialNumber> from [`Self`] as borrowed slice.
	fn r#serial_number(&self) -> &[SerialNumberProperty];
	/// Get <https://schema.org/sku> from [`Self`] as borrowed slice.
	fn r#sku(&self) -> &[SkuProperty];
	/// Get <https://schema.org/validFrom> from [`Self`] as borrowed slice.
	fn r#valid_from(&self) -> &[ValidFromProperty];
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn r#valid_through(&self) -> &[ValidThroughProperty];
	/// Get <https://schema.org/warranty> from [`Self`] as borrowed slice.
	fn r#warranty(&self) -> &[WarrantyProperty];
}
impl DemandTrait for Demand {
	fn r#accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty] {
		self.r#accepted_payment_method.as_slice()
	}
	fn r#advance_booking_requirement(&self) -> &[AdvanceBookingRequirementProperty] {
		self.r#advance_booking_requirement.as_slice()
	}
	fn r#area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn r#asin(&self) -> &[AsinProperty] {
		self.r#asin.as_slice()
	}
	fn r#availability(&self) -> &[AvailabilityProperty] {
		self.r#availability.as_slice()
	}
	fn r#availability_ends(&self) -> &[AvailabilityEndsProperty] {
		self.r#availability_ends.as_slice()
	}
	fn r#availability_starts(&self) -> &[AvailabilityStartsProperty] {
		self.r#availability_starts.as_slice()
	}
	fn r#available_at_or_from(&self) -> &[AvailableAtOrFromProperty] {
		self.r#available_at_or_from.as_slice()
	}
	fn r#available_delivery_method(&self) -> &[AvailableDeliveryMethodProperty] {
		self.r#available_delivery_method.as_slice()
	}
	fn r#business_function(&self) -> &[BusinessFunctionProperty] {
		self.r#business_function.as_slice()
	}
	fn r#delivery_lead_time(&self) -> &[DeliveryLeadTimeProperty] {
		self.r#delivery_lead_time.as_slice()
	}
	fn r#eligible_customer_type(&self) -> &[EligibleCustomerTypeProperty] {
		self.r#eligible_customer_type.as_slice()
	}
	fn r#eligible_duration(&self) -> &[EligibleDurationProperty] {
		self.r#eligible_duration.as_slice()
	}
	fn r#eligible_quantity(&self) -> &[EligibleQuantityProperty] {
		self.r#eligible_quantity.as_slice()
	}
	fn r#eligible_region(&self) -> &[EligibleRegionProperty] {
		self.r#eligible_region.as_slice()
	}
	fn r#eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty] {
		self.r#eligible_transaction_volume.as_slice()
	}
	fn r#gtin(&self) -> &[GtinProperty] {
		self.r#gtin.as_slice()
	}
	fn r#gtin_12(&self) -> &[Gtin12Property] {
		self.r#gtin_12.as_slice()
	}
	fn r#gtin_13(&self) -> &[Gtin13Property] {
		self.r#gtin_13.as_slice()
	}
	fn r#gtin_14(&self) -> &[Gtin14Property] {
		self.r#gtin_14.as_slice()
	}
	fn r#gtin_8(&self) -> &[Gtin8Property] {
		self.r#gtin_8.as_slice()
	}
	fn r#includes_object(&self) -> &[IncludesObjectProperty] {
		self.r#includes_object.as_slice()
	}
	fn r#ineligible_region(&self) -> &[IneligibleRegionProperty] {
		self.r#ineligible_region.as_slice()
	}
	fn r#inventory_level(&self) -> &[InventoryLevelProperty] {
		self.r#inventory_level.as_slice()
	}
	fn r#item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn r#item_offered(&self) -> &[ItemOfferedProperty] {
		self.r#item_offered.as_slice()
	}
	fn r#mpn(&self) -> &[MpnProperty] {
		self.r#mpn.as_slice()
	}
	fn r#price_specification(&self) -> &[PriceSpecificationProperty] {
		self.r#price_specification.as_slice()
	}
	fn r#seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn r#serial_number(&self) -> &[SerialNumberProperty] {
		self.r#serial_number.as_slice()
	}
	fn r#sku(&self) -> &[SkuProperty] {
		self.r#sku.as_slice()
	}
	fn r#valid_from(&self) -> &[ValidFromProperty] {
		self.r#valid_from.as_slice()
	}
	fn r#valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn r#warranty(&self) -> &[WarrantyProperty] {
		self.r#warranty.as_slice()
	}
}
impl ThingTrait for Demand {
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
