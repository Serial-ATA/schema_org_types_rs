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
	fn get_accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty];
	/// Take <https://schema.org/acceptedPaymentMethod> from [`Self`] as owned vector.
	fn take_accepted_payment_method(&mut self) -> Vec<AcceptedPaymentMethodProperty>;
	/// Get <https://schema.org/advanceBookingRequirement> from [`Self`] as borrowed slice.
	fn get_advance_booking_requirement(&self) -> &[AdvanceBookingRequirementProperty];
	/// Take <https://schema.org/advanceBookingRequirement> from [`Self`] as owned vector.
	fn take_advance_booking_requirement(&mut self) -> Vec<AdvanceBookingRequirementProperty>;
	/// Get <https://schema.org/areaServed> from [`Self`] as borrowed slice.
	fn get_area_served(&self) -> &[AreaServedProperty];
	/// Take <https://schema.org/areaServed> from [`Self`] as owned vector.
	fn take_area_served(&mut self) -> Vec<AreaServedProperty>;
	/// Get <https://schema.org/asin> from [`Self`] as borrowed slice.
	fn get_asin(&self) -> &[AsinProperty];
	/// Take <https://schema.org/asin> from [`Self`] as owned vector.
	fn take_asin(&mut self) -> Vec<AsinProperty>;
	/// Get <https://schema.org/availability> from [`Self`] as borrowed slice.
	fn get_availability(&self) -> &[AvailabilityProperty];
	/// Take <https://schema.org/availability> from [`Self`] as owned vector.
	fn take_availability(&mut self) -> Vec<AvailabilityProperty>;
	/// Get <https://schema.org/availabilityEnds> from [`Self`] as borrowed slice.
	fn get_availability_ends(&self) -> &[AvailabilityEndsProperty];
	/// Take <https://schema.org/availabilityEnds> from [`Self`] as owned vector.
	fn take_availability_ends(&mut self) -> Vec<AvailabilityEndsProperty>;
	/// Get <https://schema.org/availabilityStarts> from [`Self`] as borrowed slice.
	fn get_availability_starts(&self) -> &[AvailabilityStartsProperty];
	/// Take <https://schema.org/availabilityStarts> from [`Self`] as owned vector.
	fn take_availability_starts(&mut self) -> Vec<AvailabilityStartsProperty>;
	/// Get <https://schema.org/availableAtOrFrom> from [`Self`] as borrowed slice.
	fn get_available_at_or_from(&self) -> &[AvailableAtOrFromProperty];
	/// Take <https://schema.org/availableAtOrFrom> from [`Self`] as owned vector.
	fn take_available_at_or_from(&mut self) -> Vec<AvailableAtOrFromProperty>;
	/// Get <https://schema.org/availableDeliveryMethod> from [`Self`] as borrowed slice.
	fn get_available_delivery_method(&self) -> &[AvailableDeliveryMethodProperty];
	/// Take <https://schema.org/availableDeliveryMethod> from [`Self`] as owned vector.
	fn take_available_delivery_method(&mut self) -> Vec<AvailableDeliveryMethodProperty>;
	/// Get <https://schema.org/businessFunction> from [`Self`] as borrowed slice.
	fn get_business_function(&self) -> &[BusinessFunctionProperty];
	/// Take <https://schema.org/businessFunction> from [`Self`] as owned vector.
	fn take_business_function(&mut self) -> Vec<BusinessFunctionProperty>;
	/// Get <https://schema.org/deliveryLeadTime> from [`Self`] as borrowed slice.
	fn get_delivery_lead_time(&self) -> &[DeliveryLeadTimeProperty];
	/// Take <https://schema.org/deliveryLeadTime> from [`Self`] as owned vector.
	fn take_delivery_lead_time(&mut self) -> Vec<DeliveryLeadTimeProperty>;
	/// Get <https://schema.org/eligibleCustomerType> from [`Self`] as borrowed slice.
	fn get_eligible_customer_type(&self) -> &[EligibleCustomerTypeProperty];
	/// Take <https://schema.org/eligibleCustomerType> from [`Self`] as owned vector.
	fn take_eligible_customer_type(&mut self) -> Vec<EligibleCustomerTypeProperty>;
	/// Get <https://schema.org/eligibleDuration> from [`Self`] as borrowed slice.
	fn get_eligible_duration(&self) -> &[EligibleDurationProperty];
	/// Take <https://schema.org/eligibleDuration> from [`Self`] as owned vector.
	fn take_eligible_duration(&mut self) -> Vec<EligibleDurationProperty>;
	/// Get <https://schema.org/eligibleQuantity> from [`Self`] as borrowed slice.
	fn get_eligible_quantity(&self) -> &[EligibleQuantityProperty];
	/// Take <https://schema.org/eligibleQuantity> from [`Self`] as owned vector.
	fn take_eligible_quantity(&mut self) -> Vec<EligibleQuantityProperty>;
	/// Get <https://schema.org/eligibleRegion> from [`Self`] as borrowed slice.
	fn get_eligible_region(&self) -> &[EligibleRegionProperty];
	/// Take <https://schema.org/eligibleRegion> from [`Self`] as owned vector.
	fn take_eligible_region(&mut self) -> Vec<EligibleRegionProperty>;
	/// Get <https://schema.org/eligibleTransactionVolume> from [`Self`] as borrowed slice.
	fn get_eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty];
	/// Take <https://schema.org/eligibleTransactionVolume> from [`Self`] as owned vector.
	fn take_eligible_transaction_volume(&mut self) -> Vec<EligibleTransactionVolumeProperty>;
	/// Get <https://schema.org/gtin> from [`Self`] as borrowed slice.
	fn get_gtin(&self) -> &[GtinProperty];
	/// Take <https://schema.org/gtin> from [`Self`] as owned vector.
	fn take_gtin(&mut self) -> Vec<GtinProperty>;
	/// Get <https://schema.org/gtin12> from [`Self`] as borrowed slice.
	fn get_gtin_12(&self) -> &[Gtin12Property];
	/// Take <https://schema.org/gtin12> from [`Self`] as owned vector.
	fn take_gtin_12(&mut self) -> Vec<Gtin12Property>;
	/// Get <https://schema.org/gtin13> from [`Self`] as borrowed slice.
	fn get_gtin_13(&self) -> &[Gtin13Property];
	/// Take <https://schema.org/gtin13> from [`Self`] as owned vector.
	fn take_gtin_13(&mut self) -> Vec<Gtin13Property>;
	/// Get <https://schema.org/gtin14> from [`Self`] as borrowed slice.
	fn get_gtin_14(&self) -> &[Gtin14Property];
	/// Take <https://schema.org/gtin14> from [`Self`] as owned vector.
	fn take_gtin_14(&mut self) -> Vec<Gtin14Property>;
	/// Get <https://schema.org/gtin8> from [`Self`] as borrowed slice.
	fn get_gtin_8(&self) -> &[Gtin8Property];
	/// Take <https://schema.org/gtin8> from [`Self`] as owned vector.
	fn take_gtin_8(&mut self) -> Vec<Gtin8Property>;
	/// Get <https://schema.org/includesObject> from [`Self`] as borrowed slice.
	fn get_includes_object(&self) -> &[IncludesObjectProperty];
	/// Take <https://schema.org/includesObject> from [`Self`] as owned vector.
	fn take_includes_object(&mut self) -> Vec<IncludesObjectProperty>;
	/// Get <https://schema.org/ineligibleRegion> from [`Self`] as borrowed slice.
	fn get_ineligible_region(&self) -> &[IneligibleRegionProperty];
	/// Take <https://schema.org/ineligibleRegion> from [`Self`] as owned vector.
	fn take_ineligible_region(&mut self) -> Vec<IneligibleRegionProperty>;
	/// Get <https://schema.org/inventoryLevel> from [`Self`] as borrowed slice.
	fn get_inventory_level(&self) -> &[InventoryLevelProperty];
	/// Take <https://schema.org/inventoryLevel> from [`Self`] as owned vector.
	fn take_inventory_level(&mut self) -> Vec<InventoryLevelProperty>;
	/// Get <https://schema.org/itemCondition> from [`Self`] as borrowed slice.
	fn get_item_condition(&self) -> &[ItemConditionProperty];
	/// Take <https://schema.org/itemCondition> from [`Self`] as owned vector.
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty>;
	/// Get <https://schema.org/itemOffered> from [`Self`] as borrowed slice.
	fn get_item_offered(&self) -> &[ItemOfferedProperty];
	/// Take <https://schema.org/itemOffered> from [`Self`] as owned vector.
	fn take_item_offered(&mut self) -> Vec<ItemOfferedProperty>;
	/// Get <https://schema.org/mpn> from [`Self`] as borrowed slice.
	fn get_mpn(&self) -> &[MpnProperty];
	/// Take <https://schema.org/mpn> from [`Self`] as owned vector.
	fn take_mpn(&mut self) -> Vec<MpnProperty>;
	/// Get <https://schema.org/priceSpecification> from [`Self`] as borrowed slice.
	fn get_price_specification(&self) -> &[PriceSpecificationProperty];
	/// Take <https://schema.org/priceSpecification> from [`Self`] as owned vector.
	fn take_price_specification(&mut self) -> Vec<PriceSpecificationProperty>;
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn get_seller(&self) -> &[SellerProperty];
	/// Take <https://schema.org/seller> from [`Self`] as owned vector.
	fn take_seller(&mut self) -> Vec<SellerProperty>;
	/// Get <https://schema.org/serialNumber> from [`Self`] as borrowed slice.
	fn get_serial_number(&self) -> &[SerialNumberProperty];
	/// Take <https://schema.org/serialNumber> from [`Self`] as owned vector.
	fn take_serial_number(&mut self) -> Vec<SerialNumberProperty>;
	/// Get <https://schema.org/sku> from [`Self`] as borrowed slice.
	fn get_sku(&self) -> &[SkuProperty];
	/// Take <https://schema.org/sku> from [`Self`] as owned vector.
	fn take_sku(&mut self) -> Vec<SkuProperty>;
	/// Get <https://schema.org/validFrom> from [`Self`] as borrowed slice.
	fn get_valid_from(&self) -> &[ValidFromProperty];
	/// Take <https://schema.org/validFrom> from [`Self`] as owned vector.
	fn take_valid_from(&mut self) -> Vec<ValidFromProperty>;
	/// Get <https://schema.org/validThrough> from [`Self`] as borrowed slice.
	fn get_valid_through(&self) -> &[ValidThroughProperty];
	/// Take <https://schema.org/validThrough> from [`Self`] as owned vector.
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty>;
	/// Get <https://schema.org/warranty> from [`Self`] as borrowed slice.
	fn get_warranty(&self) -> &[WarrantyProperty];
	/// Take <https://schema.org/warranty> from [`Self`] as owned vector.
	fn take_warranty(&mut self) -> Vec<WarrantyProperty>;
}
impl DemandTrait for Demand {
	fn get_accepted_payment_method(&self) -> &[AcceptedPaymentMethodProperty] {
		self.r#accepted_payment_method.as_slice()
	}
	fn take_accepted_payment_method(&mut self) -> Vec<AcceptedPaymentMethodProperty> {
		std::mem::take(&mut self.r#accepted_payment_method)
	}
	fn get_advance_booking_requirement(&self) -> &[AdvanceBookingRequirementProperty] {
		self.r#advance_booking_requirement.as_slice()
	}
	fn take_advance_booking_requirement(&mut self) -> Vec<AdvanceBookingRequirementProperty> {
		std::mem::take(&mut self.r#advance_booking_requirement)
	}
	fn get_area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn take_area_served(&mut self) -> Vec<AreaServedProperty> {
		std::mem::take(&mut self.r#area_served)
	}
	fn get_asin(&self) -> &[AsinProperty] {
		self.r#asin.as_slice()
	}
	fn take_asin(&mut self) -> Vec<AsinProperty> {
		std::mem::take(&mut self.r#asin)
	}
	fn get_availability(&self) -> &[AvailabilityProperty] {
		self.r#availability.as_slice()
	}
	fn take_availability(&mut self) -> Vec<AvailabilityProperty> {
		std::mem::take(&mut self.r#availability)
	}
	fn get_availability_ends(&self) -> &[AvailabilityEndsProperty] {
		self.r#availability_ends.as_slice()
	}
	fn take_availability_ends(&mut self) -> Vec<AvailabilityEndsProperty> {
		std::mem::take(&mut self.r#availability_ends)
	}
	fn get_availability_starts(&self) -> &[AvailabilityStartsProperty] {
		self.r#availability_starts.as_slice()
	}
	fn take_availability_starts(&mut self) -> Vec<AvailabilityStartsProperty> {
		std::mem::take(&mut self.r#availability_starts)
	}
	fn get_available_at_or_from(&self) -> &[AvailableAtOrFromProperty] {
		self.r#available_at_or_from.as_slice()
	}
	fn take_available_at_or_from(&mut self) -> Vec<AvailableAtOrFromProperty> {
		std::mem::take(&mut self.r#available_at_or_from)
	}
	fn get_available_delivery_method(&self) -> &[AvailableDeliveryMethodProperty] {
		self.r#available_delivery_method.as_slice()
	}
	fn take_available_delivery_method(&mut self) -> Vec<AvailableDeliveryMethodProperty> {
		std::mem::take(&mut self.r#available_delivery_method)
	}
	fn get_business_function(&self) -> &[BusinessFunctionProperty] {
		self.r#business_function.as_slice()
	}
	fn take_business_function(&mut self) -> Vec<BusinessFunctionProperty> {
		std::mem::take(&mut self.r#business_function)
	}
	fn get_delivery_lead_time(&self) -> &[DeliveryLeadTimeProperty] {
		self.r#delivery_lead_time.as_slice()
	}
	fn take_delivery_lead_time(&mut self) -> Vec<DeliveryLeadTimeProperty> {
		std::mem::take(&mut self.r#delivery_lead_time)
	}
	fn get_eligible_customer_type(&self) -> &[EligibleCustomerTypeProperty] {
		self.r#eligible_customer_type.as_slice()
	}
	fn take_eligible_customer_type(&mut self) -> Vec<EligibleCustomerTypeProperty> {
		std::mem::take(&mut self.r#eligible_customer_type)
	}
	fn get_eligible_duration(&self) -> &[EligibleDurationProperty] {
		self.r#eligible_duration.as_slice()
	}
	fn take_eligible_duration(&mut self) -> Vec<EligibleDurationProperty> {
		std::mem::take(&mut self.r#eligible_duration)
	}
	fn get_eligible_quantity(&self) -> &[EligibleQuantityProperty] {
		self.r#eligible_quantity.as_slice()
	}
	fn take_eligible_quantity(&mut self) -> Vec<EligibleQuantityProperty> {
		std::mem::take(&mut self.r#eligible_quantity)
	}
	fn get_eligible_region(&self) -> &[EligibleRegionProperty] {
		self.r#eligible_region.as_slice()
	}
	fn take_eligible_region(&mut self) -> Vec<EligibleRegionProperty> {
		std::mem::take(&mut self.r#eligible_region)
	}
	fn get_eligible_transaction_volume(&self) -> &[EligibleTransactionVolumeProperty] {
		self.r#eligible_transaction_volume.as_slice()
	}
	fn take_eligible_transaction_volume(&mut self) -> Vec<EligibleTransactionVolumeProperty> {
		std::mem::take(&mut self.r#eligible_transaction_volume)
	}
	fn get_gtin(&self) -> &[GtinProperty] {
		self.r#gtin.as_slice()
	}
	fn take_gtin(&mut self) -> Vec<GtinProperty> {
		std::mem::take(&mut self.r#gtin)
	}
	fn get_gtin_12(&self) -> &[Gtin12Property] {
		self.r#gtin_12.as_slice()
	}
	fn take_gtin_12(&mut self) -> Vec<Gtin12Property> {
		std::mem::take(&mut self.r#gtin_12)
	}
	fn get_gtin_13(&self) -> &[Gtin13Property] {
		self.r#gtin_13.as_slice()
	}
	fn take_gtin_13(&mut self) -> Vec<Gtin13Property> {
		std::mem::take(&mut self.r#gtin_13)
	}
	fn get_gtin_14(&self) -> &[Gtin14Property] {
		self.r#gtin_14.as_slice()
	}
	fn take_gtin_14(&mut self) -> Vec<Gtin14Property> {
		std::mem::take(&mut self.r#gtin_14)
	}
	fn get_gtin_8(&self) -> &[Gtin8Property] {
		self.r#gtin_8.as_slice()
	}
	fn take_gtin_8(&mut self) -> Vec<Gtin8Property> {
		std::mem::take(&mut self.r#gtin_8)
	}
	fn get_includes_object(&self) -> &[IncludesObjectProperty] {
		self.r#includes_object.as_slice()
	}
	fn take_includes_object(&mut self) -> Vec<IncludesObjectProperty> {
		std::mem::take(&mut self.r#includes_object)
	}
	fn get_ineligible_region(&self) -> &[IneligibleRegionProperty] {
		self.r#ineligible_region.as_slice()
	}
	fn take_ineligible_region(&mut self) -> Vec<IneligibleRegionProperty> {
		std::mem::take(&mut self.r#ineligible_region)
	}
	fn get_inventory_level(&self) -> &[InventoryLevelProperty] {
		self.r#inventory_level.as_slice()
	}
	fn take_inventory_level(&mut self) -> Vec<InventoryLevelProperty> {
		std::mem::take(&mut self.r#inventory_level)
	}
	fn get_item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty> {
		std::mem::take(&mut self.r#item_condition)
	}
	fn get_item_offered(&self) -> &[ItemOfferedProperty] {
		self.r#item_offered.as_slice()
	}
	fn take_item_offered(&mut self) -> Vec<ItemOfferedProperty> {
		std::mem::take(&mut self.r#item_offered)
	}
	fn get_mpn(&self) -> &[MpnProperty] {
		self.r#mpn.as_slice()
	}
	fn take_mpn(&mut self) -> Vec<MpnProperty> {
		std::mem::take(&mut self.r#mpn)
	}
	fn get_price_specification(&self) -> &[PriceSpecificationProperty] {
		self.r#price_specification.as_slice()
	}
	fn take_price_specification(&mut self) -> Vec<PriceSpecificationProperty> {
		std::mem::take(&mut self.r#price_specification)
	}
	fn get_seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn take_seller(&mut self) -> Vec<SellerProperty> {
		std::mem::take(&mut self.r#seller)
	}
	fn get_serial_number(&self) -> &[SerialNumberProperty] {
		self.r#serial_number.as_slice()
	}
	fn take_serial_number(&mut self) -> Vec<SerialNumberProperty> {
		std::mem::take(&mut self.r#serial_number)
	}
	fn get_sku(&self) -> &[SkuProperty] {
		self.r#sku.as_slice()
	}
	fn take_sku(&mut self) -> Vec<SkuProperty> {
		std::mem::take(&mut self.r#sku)
	}
	fn get_valid_from(&self) -> &[ValidFromProperty] {
		self.r#valid_from.as_slice()
	}
	fn take_valid_from(&mut self) -> Vec<ValidFromProperty> {
		std::mem::take(&mut self.r#valid_from)
	}
	fn get_valid_through(&self) -> &[ValidThroughProperty] {
		self.r#valid_through.as_slice()
	}
	fn take_valid_through(&mut self) -> Vec<ValidThroughProperty> {
		std::mem::take(&mut self.r#valid_through)
	}
	fn get_warranty(&self) -> &[WarrantyProperty] {
		self.r#warranty.as_slice()
	}
	fn take_warranty(&mut self) -> Vec<WarrantyProperty> {
		std::mem::take(&mut self.r#warranty)
	}
}
impl ThingTrait for Demand {
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
