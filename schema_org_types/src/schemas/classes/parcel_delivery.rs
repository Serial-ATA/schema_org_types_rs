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
	fn r#carrier(&self) -> &[CarrierProperty];
	/// Get <https://schema.org/deliveryAddress> from [`Self`] as borrowed slice.
	fn r#delivery_address(&self) -> &[DeliveryAddressProperty];
	/// Get <https://schema.org/deliveryStatus> from [`Self`] as borrowed slice.
	fn r#delivery_status(&self) -> &[DeliveryStatusProperty];
	/// Get <https://schema.org/expectedArrivalFrom> from [`Self`] as borrowed slice.
	fn r#expected_arrival_from(&self) -> &[ExpectedArrivalFromProperty];
	/// Get <https://schema.org/expectedArrivalUntil> from [`Self`] as borrowed slice.
	fn r#expected_arrival_until(&self) -> &[ExpectedArrivalUntilProperty];
	/// Get <https://schema.org/hasDeliveryMethod> from [`Self`] as borrowed slice.
	fn r#has_delivery_method(&self) -> &[HasDeliveryMethodProperty];
	/// Get <https://schema.org/itemShipped> from [`Self`] as borrowed slice.
	fn r#item_shipped(&self) -> &[ItemShippedProperty];
	/// Get <https://schema.org/originAddress> from [`Self`] as borrowed slice.
	fn r#origin_address(&self) -> &[OriginAddressProperty];
	/// Get <https://schema.org/partOfOrder> from [`Self`] as borrowed slice.
	fn r#part_of_order(&self) -> &[PartOfOrderProperty];
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn r#provider(&self) -> &[ProviderProperty];
	/// Get <https://schema.org/trackingNumber> from [`Self`] as borrowed slice.
	fn r#tracking_number(&self) -> &[TrackingNumberProperty];
	/// Get <https://schema.org/trackingUrl> from [`Self`] as borrowed slice.
	fn r#tracking_url(&self) -> &[TrackingUrlProperty];
}
impl ParcelDeliveryTrait for ParcelDelivery {
	fn r#carrier(&self) -> &[CarrierProperty] {
		self.r#carrier.as_slice()
	}
	fn r#delivery_address(&self) -> &[DeliveryAddressProperty] {
		self.r#delivery_address.as_slice()
	}
	fn r#delivery_status(&self) -> &[DeliveryStatusProperty] {
		self.r#delivery_status.as_slice()
	}
	fn r#expected_arrival_from(&self) -> &[ExpectedArrivalFromProperty] {
		self.r#expected_arrival_from.as_slice()
	}
	fn r#expected_arrival_until(&self) -> &[ExpectedArrivalUntilProperty] {
		self.r#expected_arrival_until.as_slice()
	}
	fn r#has_delivery_method(&self) -> &[HasDeliveryMethodProperty] {
		self.r#has_delivery_method.as_slice()
	}
	fn r#item_shipped(&self) -> &[ItemShippedProperty] {
		self.r#item_shipped.as_slice()
	}
	fn r#origin_address(&self) -> &[OriginAddressProperty] {
		self.r#origin_address.as_slice()
	}
	fn r#part_of_order(&self) -> &[PartOfOrderProperty] {
		self.r#part_of_order.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#tracking_number(&self) -> &[TrackingNumberProperty] {
		self.r#tracking_number.as_slice()
	}
	fn r#tracking_url(&self) -> &[TrackingUrlProperty] {
		self.r#tracking_url.as_slice()
	}
}
impl ThingTrait for ParcelDelivery {
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
