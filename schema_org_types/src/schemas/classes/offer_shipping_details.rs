use super::*;
/// <https://schema.org/OfferShippingDetails>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct OfferShippingDetails {
	/// <https://schema.org/deliveryTime>
	#[cfg_attr(feature = "serde", serde(rename = "deliveryTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#delivery_time: Vec<DeliveryTimeProperty>,
	/// <https://schema.org/depth>
	#[cfg_attr(feature = "serde", serde(rename = "depth"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#depth: Vec<DepthProperty>,
	/// <https://schema.org/doesNotShip>
	#[cfg_attr(feature = "serde", serde(rename = "doesNotShip"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#does_not_ship: Vec<DoesNotShipProperty>,
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
	/// <https://schema.org/shippingDestination>
	#[cfg_attr(feature = "serde", serde(rename = "shippingDestination"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#shipping_destination: Vec<ShippingDestinationProperty>,
	/// <https://schema.org/shippingLabel>
	#[cfg_attr(feature = "serde", serde(rename = "shippingLabel"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#shipping_label: Vec<ShippingLabelProperty>,
	/// <https://schema.org/shippingOrigin>
	#[cfg_attr(feature = "serde", serde(rename = "shippingOrigin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#shipping_origin: Vec<ShippingOriginProperty>,
	/// <https://schema.org/shippingRate>
	#[cfg_attr(feature = "serde", serde(rename = "shippingRate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#shipping_rate: Vec<ShippingRateProperty>,
	/// <https://schema.org/shippingSettingsLink>
	#[cfg_attr(feature = "serde", serde(rename = "shippingSettingsLink"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#shipping_settings_link: Vec<ShippingSettingsLinkProperty>,
	/// <https://schema.org/transitTimeLabel>
	#[cfg_attr(feature = "serde", serde(rename = "transitTimeLabel"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#transit_time_label: Vec<TransitTimeLabelProperty>,
	/// <https://schema.org/validForMemberTier>
	#[cfg_attr(feature = "serde", serde(rename = "validForMemberTier"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#valid_for_member_tier: Vec<ValidForMemberTierProperty>,
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
	/// <https://schema.org/width>
	#[cfg_attr(feature = "serde", serde(rename = "width"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#width: Vec<WidthProperty>,
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
/// This trait is for properties from <https://schema.org/OfferShippingDetails>.
pub trait OfferShippingDetailsTrait {
	/// Get <https://schema.org/deliveryTime> from [`Self`] as borrowed slice.
	fn r#delivery_time(&self) -> &[DeliveryTimeProperty];
	/// Get <https://schema.org/depth> from [`Self`] as borrowed slice.
	fn r#depth(&self) -> &[DepthProperty];
	/// Get <https://schema.org/doesNotShip> from [`Self`] as borrowed slice.
	fn r#does_not_ship(&self) -> &[DoesNotShipProperty];
	/// Get <https://schema.org/hasShippingService> from [`Self`] as borrowed slice.
	fn r#has_shipping_service(&self) -> &[HasShippingServiceProperty];
	/// Get <https://schema.org/height> from [`Self`] as borrowed slice.
	fn r#height(&self) -> &[HeightProperty];
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn r#provider(&self) -> &[ProviderProperty];
	/// Get <https://schema.org/shippingDestination> from [`Self`] as borrowed slice.
	fn r#shipping_destination(&self) -> &[ShippingDestinationProperty];
	/// Get <https://schema.org/shippingLabel> from [`Self`] as borrowed slice.
	fn r#shipping_label(&self) -> &[ShippingLabelProperty];
	/// Get <https://schema.org/shippingOrigin> from [`Self`] as borrowed slice.
	fn r#shipping_origin(&self) -> &[ShippingOriginProperty];
	/// Get <https://schema.org/shippingRate> from [`Self`] as borrowed slice.
	fn r#shipping_rate(&self) -> &[ShippingRateProperty];
	/// Get <https://schema.org/shippingSettingsLink> from [`Self`] as borrowed slice.
	fn r#shipping_settings_link(&self) -> &[ShippingSettingsLinkProperty];
	/// Get <https://schema.org/transitTimeLabel> from [`Self`] as borrowed slice.
	fn r#transit_time_label(&self) -> &[TransitTimeLabelProperty];
	/// Get <https://schema.org/validForMemberTier> from [`Self`] as borrowed slice.
	fn r#valid_for_member_tier(&self) -> &[ValidForMemberTierProperty];
	/// Get <https://schema.org/weight> from [`Self`] as borrowed slice.
	fn r#weight(&self) -> &[WeightProperty];
	/// Get <https://schema.org/width> from [`Self`] as borrowed slice.
	fn r#width(&self) -> &[WidthProperty];
}
impl OfferShippingDetailsTrait for OfferShippingDetails {
	fn r#delivery_time(&self) -> &[DeliveryTimeProperty] {
		self.r#delivery_time.as_slice()
	}
	fn r#depth(&self) -> &[DepthProperty] {
		self.r#depth.as_slice()
	}
	fn r#does_not_ship(&self) -> &[DoesNotShipProperty] {
		self.r#does_not_ship.as_slice()
	}
	fn r#has_shipping_service(&self) -> &[HasShippingServiceProperty] {
		self.r#has_shipping_service.as_slice()
	}
	fn r#height(&self) -> &[HeightProperty] {
		self.r#height.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#shipping_destination(&self) -> &[ShippingDestinationProperty] {
		self.r#shipping_destination.as_slice()
	}
	fn r#shipping_label(&self) -> &[ShippingLabelProperty] {
		self.r#shipping_label.as_slice()
	}
	fn r#shipping_origin(&self) -> &[ShippingOriginProperty] {
		self.r#shipping_origin.as_slice()
	}
	fn r#shipping_rate(&self) -> &[ShippingRateProperty] {
		self.r#shipping_rate.as_slice()
	}
	fn r#shipping_settings_link(&self) -> &[ShippingSettingsLinkProperty] {
		self.r#shipping_settings_link.as_slice()
	}
	fn r#transit_time_label(&self) -> &[TransitTimeLabelProperty] {
		self.r#transit_time_label.as_slice()
	}
	fn r#valid_for_member_tier(&self) -> &[ValidForMemberTierProperty] {
		self.r#valid_for_member_tier.as_slice()
	}
	fn r#weight(&self) -> &[WeightProperty] {
		self.r#weight.as_slice()
	}
	fn r#width(&self) -> &[WidthProperty] {
		self.r#width.as_slice()
	}
}
impl StructuredValueTrait for OfferShippingDetails {}
impl ThingTrait for OfferShippingDetails {
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
