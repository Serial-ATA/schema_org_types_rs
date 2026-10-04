use super::*;
/// <https://schema.org/ShippingRateSettings>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ShippingRateSettings {
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
	/// <https://schema.org/freeShippingThreshold>
	#[cfg_attr(feature = "serde", serde(rename = "freeShippingThreshold"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#free_shipping_threshold: Vec<FreeShippingThresholdProperty>,
	/// <https://schema.org/isUnlabelledFallback>
	#[cfg_attr(feature = "serde", serde(rename = "isUnlabelledFallback"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_unlabelled_fallback: Vec<IsUnlabelledFallbackProperty>,
	/// <https://schema.org/minimumOrderValue>
	#[cfg_attr(feature = "serde", serde(rename = "minimumOrderValue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#minimum_order_value: Vec<MinimumOrderValueProperty>,
	/// <https://schema.org/orderPercentage>
	#[cfg_attr(feature = "serde", serde(rename = "orderPercentage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#order_percentage: Vec<OrderPercentageProperty>,
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
	/// <https://schema.org/weightPercentage>
	#[cfg_attr(feature = "serde", serde(rename = "weightPercentage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#weight_percentage: Vec<WeightPercentageProperty>,
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
/// This trait is for properties from <https://schema.org/ShippingRateSettings>.
pub trait ShippingRateSettingsTrait {
	/// Get <https://schema.org/doesNotShip> from [`Self`] as borrowed slice.
	fn get_does_not_ship(&self) -> &[DoesNotShipProperty];
	/// Take <https://schema.org/doesNotShip> from [`Self`] as owned vector.
	fn take_does_not_ship(&mut self) -> Vec<DoesNotShipProperty>;
	/// Get <https://schema.org/freeShippingThreshold> from [`Self`] as borrowed slice.
	fn get_free_shipping_threshold(&self) -> &[FreeShippingThresholdProperty];
	/// Take <https://schema.org/freeShippingThreshold> from [`Self`] as owned vector.
	fn take_free_shipping_threshold(&mut self) -> Vec<FreeShippingThresholdProperty>;
	/// Get <https://schema.org/isUnlabelledFallback> from [`Self`] as borrowed slice.
	fn get_is_unlabelled_fallback(&self) -> &[IsUnlabelledFallbackProperty];
	/// Take <https://schema.org/isUnlabelledFallback> from [`Self`] as owned vector.
	fn take_is_unlabelled_fallback(&mut self) -> Vec<IsUnlabelledFallbackProperty>;
	/// Get <https://schema.org/minimumOrderValue> from [`Self`] as borrowed slice.
	fn get_minimum_order_value(&self) -> &[MinimumOrderValueProperty];
	/// Take <https://schema.org/minimumOrderValue> from [`Self`] as owned vector.
	fn take_minimum_order_value(&mut self) -> Vec<MinimumOrderValueProperty>;
	/// Get <https://schema.org/orderPercentage> from [`Self`] as borrowed slice.
	fn get_order_percentage(&self) -> &[OrderPercentageProperty];
	/// Take <https://schema.org/orderPercentage> from [`Self`] as owned vector.
	fn take_order_percentage(&mut self) -> Vec<OrderPercentageProperty>;
	/// Get <https://schema.org/shippingDestination> from [`Self`] as borrowed slice.
	fn get_shipping_destination(&self) -> &[ShippingDestinationProperty];
	/// Take <https://schema.org/shippingDestination> from [`Self`] as owned vector.
	fn take_shipping_destination(&mut self) -> Vec<ShippingDestinationProperty>;
	/// Get <https://schema.org/shippingLabel> from [`Self`] as borrowed slice.
	fn get_shipping_label(&self) -> &[ShippingLabelProperty];
	/// Take <https://schema.org/shippingLabel> from [`Self`] as owned vector.
	fn take_shipping_label(&mut self) -> Vec<ShippingLabelProperty>;
	/// Get <https://schema.org/shippingRate> from [`Self`] as borrowed slice.
	fn get_shipping_rate(&self) -> &[ShippingRateProperty];
	/// Take <https://schema.org/shippingRate> from [`Self`] as owned vector.
	fn take_shipping_rate(&mut self) -> Vec<ShippingRateProperty>;
	/// Get <https://schema.org/weightPercentage> from [`Self`] as borrowed slice.
	fn get_weight_percentage(&self) -> &[WeightPercentageProperty];
	/// Take <https://schema.org/weightPercentage> from [`Self`] as owned vector.
	fn take_weight_percentage(&mut self) -> Vec<WeightPercentageProperty>;
}
impl ShippingRateSettingsTrait for ShippingRateSettings {
	fn get_does_not_ship(&self) -> &[DoesNotShipProperty] {
		self.r#does_not_ship.as_slice()
	}
	fn take_does_not_ship(&mut self) -> Vec<DoesNotShipProperty> {
		std::mem::take(&mut self.r#does_not_ship)
	}
	fn get_free_shipping_threshold(&self) -> &[FreeShippingThresholdProperty] {
		self.r#free_shipping_threshold.as_slice()
	}
	fn take_free_shipping_threshold(&mut self) -> Vec<FreeShippingThresholdProperty> {
		std::mem::take(&mut self.r#free_shipping_threshold)
	}
	fn get_is_unlabelled_fallback(&self) -> &[IsUnlabelledFallbackProperty] {
		self.r#is_unlabelled_fallback.as_slice()
	}
	fn take_is_unlabelled_fallback(&mut self) -> Vec<IsUnlabelledFallbackProperty> {
		std::mem::take(&mut self.r#is_unlabelled_fallback)
	}
	fn get_minimum_order_value(&self) -> &[MinimumOrderValueProperty] {
		self.r#minimum_order_value.as_slice()
	}
	fn take_minimum_order_value(&mut self) -> Vec<MinimumOrderValueProperty> {
		std::mem::take(&mut self.r#minimum_order_value)
	}
	fn get_order_percentage(&self) -> &[OrderPercentageProperty] {
		self.r#order_percentage.as_slice()
	}
	fn take_order_percentage(&mut self) -> Vec<OrderPercentageProperty> {
		std::mem::take(&mut self.r#order_percentage)
	}
	fn get_shipping_destination(&self) -> &[ShippingDestinationProperty] {
		self.r#shipping_destination.as_slice()
	}
	fn take_shipping_destination(&mut self) -> Vec<ShippingDestinationProperty> {
		std::mem::take(&mut self.r#shipping_destination)
	}
	fn get_shipping_label(&self) -> &[ShippingLabelProperty] {
		self.r#shipping_label.as_slice()
	}
	fn take_shipping_label(&mut self) -> Vec<ShippingLabelProperty> {
		std::mem::take(&mut self.r#shipping_label)
	}
	fn get_shipping_rate(&self) -> &[ShippingRateProperty] {
		self.r#shipping_rate.as_slice()
	}
	fn take_shipping_rate(&mut self) -> Vec<ShippingRateProperty> {
		std::mem::take(&mut self.r#shipping_rate)
	}
	fn get_weight_percentage(&self) -> &[WeightPercentageProperty] {
		self.r#weight_percentage.as_slice()
	}
	fn take_weight_percentage(&mut self) -> Vec<WeightPercentageProperty> {
		std::mem::take(&mut self.r#weight_percentage)
	}
}
impl StructuredValueTrait for ShippingRateSettings {}
impl ThingTrait for ShippingRateSettings {
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
