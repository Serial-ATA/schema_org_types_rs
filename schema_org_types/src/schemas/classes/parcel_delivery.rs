use super::*;
/// <https://schema.org/ParcelDelivery>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ParcelDelivery {
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
	/// <https://schema.org/deliveryAddress>
	#[cfg_attr(feature = "serde", serde(rename = "deliveryAddress"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#delivery_address: Vec<DeliveryAddressProperty>,
	/// <https://schema.org/deliveryStatus>
	#[cfg_attr(feature = "serde", serde(rename = "deliveryStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#delivery_status: Vec<DeliveryStatusProperty>,
	/// <https://schema.org/expectedArrivalFrom>
	#[cfg_attr(feature = "serde", serde(rename = "expectedArrivalFrom"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#expected_arrival_from: Vec<ExpectedArrivalFromProperty>,
	/// <https://schema.org/expectedArrivalUntil>
	#[cfg_attr(feature = "serde", serde(rename = "expectedArrivalUntil"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#expected_arrival_until: Vec<ExpectedArrivalUntilProperty>,
	/// <https://schema.org/hasDeliveryMethod>
	#[cfg_attr(feature = "serde", serde(rename = "hasDeliveryMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_delivery_method: Vec<HasDeliveryMethodProperty>,
	/// <https://schema.org/itemShipped>
	#[cfg_attr(feature = "serde", serde(rename = "itemShipped"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#item_shipped: Vec<ItemShippedProperty>,
	/// <https://schema.org/originAddress>
	#[cfg_attr(feature = "serde", serde(rename = "originAddress"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#origin_address: Vec<OriginAddressProperty>,
	/// <https://schema.org/partOfOrder>
	#[cfg_attr(feature = "serde", serde(rename = "partOfOrder"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#part_of_order: Vec<PartOfOrderProperty>,
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
	/// <https://schema.org/trackingNumber>
	#[cfg_attr(feature = "serde", serde(rename = "trackingNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#tracking_number: Vec<TrackingNumberProperty>,
	/// <https://schema.org/trackingUrl>
	#[cfg_attr(feature = "serde", serde(rename = "trackingUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#tracking_url: Vec<TrackingUrlProperty>,
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
/// This trait is for properties from <https://schema.org/ParcelDelivery>.
pub trait ParcelDeliveryTrait {
	/// Get <https://schema.org/carrier> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	fn get_carrier(&self) -> &[CarrierProperty];
	/// Take <https://schema.org/carrier> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	fn take_carrier(&mut self) -> Vec<CarrierProperty>;
	/// Get <https://schema.org/deliveryAddress> from [`Self`] as borrowed slice.
	fn get_delivery_address(&self) -> &[DeliveryAddressProperty];
	/// Take <https://schema.org/deliveryAddress> from [`Self`] as owned vector.
	fn take_delivery_address(&mut self) -> Vec<DeliveryAddressProperty>;
	/// Get <https://schema.org/deliveryStatus> from [`Self`] as borrowed slice.
	fn get_delivery_status(&self) -> &[DeliveryStatusProperty];
	/// Take <https://schema.org/deliveryStatus> from [`Self`] as owned vector.
	fn take_delivery_status(&mut self) -> Vec<DeliveryStatusProperty>;
	/// Get <https://schema.org/expectedArrivalFrom> from [`Self`] as borrowed slice.
	fn get_expected_arrival_from(&self) -> &[ExpectedArrivalFromProperty];
	/// Take <https://schema.org/expectedArrivalFrom> from [`Self`] as owned vector.
	fn take_expected_arrival_from(&mut self) -> Vec<ExpectedArrivalFromProperty>;
	/// Get <https://schema.org/expectedArrivalUntil> from [`Self`] as borrowed slice.
	fn get_expected_arrival_until(&self) -> &[ExpectedArrivalUntilProperty];
	/// Take <https://schema.org/expectedArrivalUntil> from [`Self`] as owned vector.
	fn take_expected_arrival_until(&mut self) -> Vec<ExpectedArrivalUntilProperty>;
	/// Get <https://schema.org/hasDeliveryMethod> from [`Self`] as borrowed slice.
	fn get_has_delivery_method(&self) -> &[HasDeliveryMethodProperty];
	/// Take <https://schema.org/hasDeliveryMethod> from [`Self`] as owned vector.
	fn take_has_delivery_method(&mut self) -> Vec<HasDeliveryMethodProperty>;
	/// Get <https://schema.org/itemShipped> from [`Self`] as borrowed slice.
	fn get_item_shipped(&self) -> &[ItemShippedProperty];
	/// Take <https://schema.org/itemShipped> from [`Self`] as owned vector.
	fn take_item_shipped(&mut self) -> Vec<ItemShippedProperty>;
	/// Get <https://schema.org/originAddress> from [`Self`] as borrowed slice.
	fn get_origin_address(&self) -> &[OriginAddressProperty];
	/// Take <https://schema.org/originAddress> from [`Self`] as owned vector.
	fn take_origin_address(&mut self) -> Vec<OriginAddressProperty>;
	/// Get <https://schema.org/partOfOrder> from [`Self`] as borrowed slice.
	fn get_part_of_order(&self) -> &[PartOfOrderProperty];
	/// Take <https://schema.org/partOfOrder> from [`Self`] as owned vector.
	fn take_part_of_order(&mut self) -> Vec<PartOfOrderProperty>;
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn get_provider(&self) -> &[ProviderProperty];
	/// Take <https://schema.org/provider> from [`Self`] as owned vector.
	fn take_provider(&mut self) -> Vec<ProviderProperty>;
	/// Get <https://schema.org/trackingNumber> from [`Self`] as borrowed slice.
	fn get_tracking_number(&self) -> &[TrackingNumberProperty];
	/// Take <https://schema.org/trackingNumber> from [`Self`] as owned vector.
	fn take_tracking_number(&mut self) -> Vec<TrackingNumberProperty>;
	/// Get <https://schema.org/trackingUrl> from [`Self`] as borrowed slice.
	fn get_tracking_url(&self) -> &[TrackingUrlProperty];
	/// Take <https://schema.org/trackingUrl> from [`Self`] as owned vector.
	fn take_tracking_url(&mut self) -> Vec<TrackingUrlProperty>;
}
impl ParcelDeliveryTrait for ParcelDelivery {
	fn get_carrier(&self) -> &[CarrierProperty] {
		self.r#carrier.as_slice()
	}
	fn take_carrier(&mut self) -> Vec<CarrierProperty> {
		std::mem::take(&mut self.r#carrier)
	}
	fn get_delivery_address(&self) -> &[DeliveryAddressProperty] {
		self.r#delivery_address.as_slice()
	}
	fn take_delivery_address(&mut self) -> Vec<DeliveryAddressProperty> {
		std::mem::take(&mut self.r#delivery_address)
	}
	fn get_delivery_status(&self) -> &[DeliveryStatusProperty] {
		self.r#delivery_status.as_slice()
	}
	fn take_delivery_status(&mut self) -> Vec<DeliveryStatusProperty> {
		std::mem::take(&mut self.r#delivery_status)
	}
	fn get_expected_arrival_from(&self) -> &[ExpectedArrivalFromProperty] {
		self.r#expected_arrival_from.as_slice()
	}
	fn take_expected_arrival_from(&mut self) -> Vec<ExpectedArrivalFromProperty> {
		std::mem::take(&mut self.r#expected_arrival_from)
	}
	fn get_expected_arrival_until(&self) -> &[ExpectedArrivalUntilProperty] {
		self.r#expected_arrival_until.as_slice()
	}
	fn take_expected_arrival_until(&mut self) -> Vec<ExpectedArrivalUntilProperty> {
		std::mem::take(&mut self.r#expected_arrival_until)
	}
	fn get_has_delivery_method(&self) -> &[HasDeliveryMethodProperty] {
		self.r#has_delivery_method.as_slice()
	}
	fn take_has_delivery_method(&mut self) -> Vec<HasDeliveryMethodProperty> {
		std::mem::take(&mut self.r#has_delivery_method)
	}
	fn get_item_shipped(&self) -> &[ItemShippedProperty] {
		self.r#item_shipped.as_slice()
	}
	fn take_item_shipped(&mut self) -> Vec<ItemShippedProperty> {
		std::mem::take(&mut self.r#item_shipped)
	}
	fn get_origin_address(&self) -> &[OriginAddressProperty] {
		self.r#origin_address.as_slice()
	}
	fn take_origin_address(&mut self) -> Vec<OriginAddressProperty> {
		std::mem::take(&mut self.r#origin_address)
	}
	fn get_part_of_order(&self) -> &[PartOfOrderProperty] {
		self.r#part_of_order.as_slice()
	}
	fn take_part_of_order(&mut self) -> Vec<PartOfOrderProperty> {
		std::mem::take(&mut self.r#part_of_order)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_tracking_number(&self) -> &[TrackingNumberProperty] {
		self.r#tracking_number.as_slice()
	}
	fn take_tracking_number(&mut self) -> Vec<TrackingNumberProperty> {
		std::mem::take(&mut self.r#tracking_number)
	}
	fn get_tracking_url(&self) -> &[TrackingUrlProperty] {
		self.r#tracking_url.as_slice()
	}
	fn take_tracking_url(&mut self) -> Vec<TrackingUrlProperty> {
		std::mem::take(&mut self.r#tracking_url)
	}
}
impl ThingTrait for ParcelDelivery {
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
