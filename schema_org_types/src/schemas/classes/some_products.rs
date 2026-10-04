use super::*;
/// <https://schema.org/SomeProducts>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct SomeProducts {
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
	/// <https://schema.org/color>
	#[cfg_attr(feature = "serde", serde(rename = "color"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#color: Vec<ColorProperty>,
	/// <https://schema.org/colorSwatch>
	#[cfg_attr(feature = "serde", serde(rename = "colorSwatch"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#color_swatch: Vec<ColorSwatchProperty>,
	/// <https://schema.org/consumerNotice>
	#[cfg_attr(feature = "serde", serde(rename = "consumerNotice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#consumer_notice: Vec<ConsumerNoticeProperty>,
	/// <https://schema.org/countryOfAssembly>
	#[cfg_attr(feature = "serde", serde(rename = "countryOfAssembly"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#country_of_assembly: Vec<CountryOfAssemblyProperty>,
	/// <https://schema.org/countryOfLastProcessing>
	#[cfg_attr(feature = "serde", serde(rename = "countryOfLastProcessing"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#country_of_last_processing: Vec<CountryOfLastProcessingProperty>,
	/// <https://schema.org/countryOfOrigin>
	#[cfg_attr(feature = "serde", serde(rename = "countryOfOrigin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#country_of_origin: Vec<CountryOfOriginProperty>,
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
	/// <https://schema.org/displayLocation>
	#[cfg_attr(feature = "serde", serde(rename = "displayLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#display_location: Vec<DisplayLocationProperty>,
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
	/// <https://schema.org/hasAdultConsideration>
	#[cfg_attr(feature = "serde", serde(rename = "hasAdultConsideration"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_adult_consideration: Vec<HasAdultConsiderationProperty>,
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
	/// <https://schema.org/hasDigitalProductPassport>
	#[cfg_attr(feature = "serde", serde(rename = "hasDigitalProductPassport"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_digital_product_passport: Vec<HasDigitalProductPassportProperty>,
	/// <https://schema.org/hasEnergyConsumptionDetails>
	#[cfg_attr(feature = "serde", serde(rename = "hasEnergyConsumptionDetails"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_energy_consumption_details: Vec<HasEnergyConsumptionDetailsProperty>,
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
	/// <https://schema.org/hasMeasurement>
	#[cfg_attr(feature = "serde", serde(rename = "hasMeasurement"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_measurement: Vec<HasMeasurementProperty>,
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
	/// <https://schema.org/importer>
	#[cfg_attr(feature = "serde", serde(rename = "importer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#importer: Vec<ImporterProperty>,
	/// <https://schema.org/inProductGroupWithID>
	#[cfg_attr(feature = "serde", serde(rename = "inProductGroupWithID"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#in_product_group_with_id: Vec<InProductGroupWithIdProperty>,
	/// <https://schema.org/isAccessoryOrSparePartFor>
	#[cfg_attr(feature = "serde", serde(rename = "isAccessoryOrSparePartFor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_accessory_or_spare_part_for: Vec<IsAccessoryOrSparePartForProperty>,
	/// <https://schema.org/isConsumableFor>
	#[cfg_attr(feature = "serde", serde(rename = "isConsumableFor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_consumable_for: Vec<IsConsumableForProperty>,
	/// <https://schema.org/isFamilyFriendly>
	#[cfg_attr(feature = "serde", serde(rename = "isFamilyFriendly"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	/// <https://schema.org/isOftenBoughtWith>
	#[cfg_attr(feature = "serde", serde(rename = "isOftenBoughtWith"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_often_bought_with: Vec<IsOftenBoughtWithProperty>,
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
	/// <https://schema.org/isVariantOf>
	#[cfg_attr(feature = "serde", serde(rename = "isVariantOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_variant_of: Vec<IsVariantOfProperty>,
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
	/// <https://schema.org/manufacturer>
	#[cfg_attr(feature = "serde", serde(rename = "manufacturer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#manufacturer: Vec<ManufacturerProperty>,
	/// <https://schema.org/material>
	#[cfg_attr(feature = "serde", serde(rename = "material"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#material: Vec<MaterialProperty>,
	/// <https://schema.org/mobileUrl>
	#[cfg_attr(feature = "serde", serde(rename = "mobileUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#mobile_url: Vec<MobileUrlProperty>,
	/// <https://schema.org/model>
	#[cfg_attr(feature = "serde", serde(rename = "model"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#model: Vec<ModelProperty>,
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
	/// <https://schema.org/negativeNotes>
	#[cfg_attr(feature = "serde", serde(rename = "negativeNotes"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#negative_notes: Vec<NegativeNotesProperty>,
	/// <https://schema.org/nsn>
	#[cfg_attr(feature = "serde", serde(rename = "nsn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#nsn: Vec<NsnProperty>,
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
	/// <https://schema.org/pattern>
	#[cfg_attr(feature = "serde", serde(rename = "pattern"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#pattern: Vec<PatternProperty>,
	/// <https://schema.org/positiveNotes>
	#[cfg_attr(feature = "serde", serde(rename = "positiveNotes"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#positive_notes: Vec<PositiveNotesProperty>,
	/// <https://schema.org/productID>
	#[cfg_attr(feature = "serde", serde(rename = "productID"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#product_id: Vec<ProductIdProperty>,
	/// <https://schema.org/productionDate>
	#[cfg_attr(feature = "serde", serde(rename = "productionDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#production_date: Vec<ProductionDateProperty>,
	/// <https://schema.org/purchaseDate>
	#[cfg_attr(feature = "serde", serde(rename = "purchaseDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#purchase_date: Vec<PurchaseDateProperty>,
	/// <https://schema.org/recycledContentPercentage>
	#[cfg_attr(feature = "serde", serde(rename = "recycledContentPercentage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#recycled_content_percentage: Vec<RecycledContentPercentageProperty>,
	/// <https://schema.org/releaseDate>
	#[cfg_attr(feature = "serde", serde(rename = "releaseDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#release_date: Vec<ReleaseDateProperty>,
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
	/// <https://schema.org/size>
	#[cfg_attr(feature = "serde", serde(rename = "size"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#size: Vec<SizeProperty>,
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
	/// <https://schema.org/specification>
	#[cfg_attr(feature = "serde", serde(rename = "specification"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#specification: Vec<SpecificationProperty>,
	/// <https://schema.org/substanceOfConcern>
	#[cfg_attr(feature = "serde", serde(rename = "substanceOfConcern"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#substance_of_concern: Vec<SubstanceOfConcernProperty>,
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
/// This trait is for properties from <https://schema.org/SomeProducts>.
pub trait SomeProductsTrait {
	/// Get <https://schema.org/inventoryLevel> from [`Self`] as borrowed slice.
	fn get_inventory_level(&self) -> &[InventoryLevelProperty];
	/// Take <https://schema.org/inventoryLevel> from [`Self`] as owned vector.
	fn take_inventory_level(&mut self) -> Vec<InventoryLevelProperty>;
}
impl SomeProductsTrait for SomeProducts {
	fn get_inventory_level(&self) -> &[InventoryLevelProperty] {
		self.r#inventory_level.as_slice()
	}
	fn take_inventory_level(&mut self) -> Vec<InventoryLevelProperty> {
		std::mem::take(&mut self.r#inventory_level)
	}
}
impl ProductTrait for SomeProducts {
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty> {
		std::mem::take(&mut self.r#additional_property)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_asin(&self) -> &[AsinProperty] {
		self.r#asin.as_slice()
	}
	fn take_asin(&mut self) -> Vec<AsinProperty> {
		std::mem::take(&mut self.r#asin)
	}
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
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
	fn get_category(&self) -> &[CategoryProperty] {
		self.r#category.as_slice()
	}
	fn take_category(&mut self) -> Vec<CategoryProperty> {
		std::mem::take(&mut self.r#category)
	}
	fn get_color(&self) -> &[ColorProperty] {
		self.r#color.as_slice()
	}
	fn take_color(&mut self) -> Vec<ColorProperty> {
		std::mem::take(&mut self.r#color)
	}
	fn get_color_swatch(&self) -> &[ColorSwatchProperty] {
		self.r#color_swatch.as_slice()
	}
	fn take_color_swatch(&mut self) -> Vec<ColorSwatchProperty> {
		std::mem::take(&mut self.r#color_swatch)
	}
	fn get_consumer_notice(&self) -> &[ConsumerNoticeProperty] {
		self.r#consumer_notice.as_slice()
	}
	fn take_consumer_notice(&mut self) -> Vec<ConsumerNoticeProperty> {
		std::mem::take(&mut self.r#consumer_notice)
	}
	fn get_country_of_assembly(&self) -> &[CountryOfAssemblyProperty] {
		self.r#country_of_assembly.as_slice()
	}
	fn take_country_of_assembly(&mut self) -> Vec<CountryOfAssemblyProperty> {
		std::mem::take(&mut self.r#country_of_assembly)
	}
	fn get_country_of_last_processing(&self) -> &[CountryOfLastProcessingProperty] {
		self.r#country_of_last_processing.as_slice()
	}
	fn take_country_of_last_processing(&mut self) -> Vec<CountryOfLastProcessingProperty> {
		std::mem::take(&mut self.r#country_of_last_processing)
	}
	fn get_country_of_origin(&self) -> &[CountryOfOriginProperty] {
		self.r#country_of_origin.as_slice()
	}
	fn take_country_of_origin(&mut self) -> Vec<CountryOfOriginProperty> {
		std::mem::take(&mut self.r#country_of_origin)
	}
	fn get_depth(&self) -> &[DepthProperty] {
		self.r#depth.as_slice()
	}
	fn take_depth(&mut self) -> Vec<DepthProperty> {
		std::mem::take(&mut self.r#depth)
	}
	fn get_display_location(&self) -> &[DisplayLocationProperty] {
		self.r#display_location.as_slice()
	}
	fn take_display_location(&mut self) -> Vec<DisplayLocationProperty> {
		std::mem::take(&mut self.r#display_location)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
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
	fn get_has_adult_consideration(&self) -> &[HasAdultConsiderationProperty] {
		self.r#has_adult_consideration.as_slice()
	}
	fn take_has_adult_consideration(&mut self) -> Vec<HasAdultConsiderationProperty> {
		std::mem::take(&mut self.r#has_adult_consideration)
	}
	fn get_has_certification(&self) -> &[HasCertificationProperty] {
		self.r#has_certification.as_slice()
	}
	fn take_has_certification(&mut self) -> Vec<HasCertificationProperty> {
		std::mem::take(&mut self.r#has_certification)
	}
	fn get_has_digital_product_passport(&self) -> &[HasDigitalProductPassportProperty] {
		self.r#has_digital_product_passport.as_slice()
	}
	fn take_has_digital_product_passport(&mut self) -> Vec<HasDigitalProductPassportProperty> {
		std::mem::take(&mut self.r#has_digital_product_passport)
	}
	fn get_has_energy_consumption_details(&self) -> &[HasEnergyConsumptionDetailsProperty] {
		self.r#has_energy_consumption_details.as_slice()
	}
	fn take_has_energy_consumption_details(&mut self) -> Vec<HasEnergyConsumptionDetailsProperty> {
		std::mem::take(&mut self.r#has_energy_consumption_details)
	}
	fn get_has_gs_1_digital_link(&self) -> &[HasGs1DigitalLinkProperty] {
		self.r#has_gs_1_digital_link.as_slice()
	}
	fn take_has_gs_1_digital_link(&mut self) -> Vec<HasGs1DigitalLinkProperty> {
		std::mem::take(&mut self.r#has_gs_1_digital_link)
	}
	fn get_has_measurement(&self) -> &[HasMeasurementProperty] {
		self.r#has_measurement.as_slice()
	}
	fn take_has_measurement(&mut self) -> Vec<HasMeasurementProperty> {
		std::mem::take(&mut self.r#has_measurement)
	}
	fn get_has_merchant_return_policy(&self) -> &[HasMerchantReturnPolicyProperty] {
		self.r#has_merchant_return_policy.as_slice()
	}
	fn take_has_merchant_return_policy(&mut self) -> Vec<HasMerchantReturnPolicyProperty> {
		std::mem::take(&mut self.r#has_merchant_return_policy)
	}
	fn get_has_product_return_policy(&self) -> &[HasProductReturnPolicyProperty] {
		self.r#has_product_return_policy.as_slice()
	}
	fn take_has_product_return_policy(&mut self) -> Vec<HasProductReturnPolicyProperty> {
		std::mem::take(&mut self.r#has_product_return_policy)
	}
	fn get_height(&self) -> &[HeightProperty] {
		self.r#height.as_slice()
	}
	fn take_height(&mut self) -> Vec<HeightProperty> {
		std::mem::take(&mut self.r#height)
	}
	fn get_importer(&self) -> &[ImporterProperty] {
		self.r#importer.as_slice()
	}
	fn take_importer(&mut self) -> Vec<ImporterProperty> {
		std::mem::take(&mut self.r#importer)
	}
	fn get_in_product_group_with_id(&self) -> &[InProductGroupWithIdProperty] {
		self.r#in_product_group_with_id.as_slice()
	}
	fn take_in_product_group_with_id(&mut self) -> Vec<InProductGroupWithIdProperty> {
		std::mem::take(&mut self.r#in_product_group_with_id)
	}
	fn get_is_accessory_or_spare_part_for(&self) -> &[IsAccessoryOrSparePartForProperty] {
		self.r#is_accessory_or_spare_part_for.as_slice()
	}
	fn take_is_accessory_or_spare_part_for(&mut self) -> Vec<IsAccessoryOrSparePartForProperty> {
		std::mem::take(&mut self.r#is_accessory_or_spare_part_for)
	}
	fn get_is_consumable_for(&self) -> &[IsConsumableForProperty] {
		self.r#is_consumable_for.as_slice()
	}
	fn take_is_consumable_for(&mut self) -> Vec<IsConsumableForProperty> {
		std::mem::take(&mut self.r#is_consumable_for)
	}
	fn get_is_family_friendly(&self) -> &[IsFamilyFriendlyProperty] {
		self.r#is_family_friendly.as_slice()
	}
	fn take_is_family_friendly(&mut self) -> Vec<IsFamilyFriendlyProperty> {
		std::mem::take(&mut self.r#is_family_friendly)
	}
	fn get_is_often_bought_with(&self) -> &[IsOftenBoughtWithProperty] {
		self.r#is_often_bought_with.as_slice()
	}
	fn take_is_often_bought_with(&mut self) -> Vec<IsOftenBoughtWithProperty> {
		std::mem::take(&mut self.r#is_often_bought_with)
	}
	fn get_is_related_to(&self) -> &[IsRelatedToProperty] {
		self.r#is_related_to.as_slice()
	}
	fn take_is_related_to(&mut self) -> Vec<IsRelatedToProperty> {
		std::mem::take(&mut self.r#is_related_to)
	}
	fn get_is_similar_to(&self) -> &[IsSimilarToProperty] {
		self.r#is_similar_to.as_slice()
	}
	fn take_is_similar_to(&mut self) -> Vec<IsSimilarToProperty> {
		std::mem::take(&mut self.r#is_similar_to)
	}
	fn get_is_variant_of(&self) -> &[IsVariantOfProperty] {
		self.r#is_variant_of.as_slice()
	}
	fn take_is_variant_of(&mut self) -> Vec<IsVariantOfProperty> {
		std::mem::take(&mut self.r#is_variant_of)
	}
	fn get_item_condition(&self) -> &[ItemConditionProperty] {
		self.r#item_condition.as_slice()
	}
	fn take_item_condition(&mut self) -> Vec<ItemConditionProperty> {
		std::mem::take(&mut self.r#item_condition)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn take_logo(&mut self) -> Vec<LogoProperty> {
		std::mem::take(&mut self.r#logo)
	}
	fn get_manufacturer(&self) -> &[ManufacturerProperty] {
		self.r#manufacturer.as_slice()
	}
	fn take_manufacturer(&mut self) -> Vec<ManufacturerProperty> {
		std::mem::take(&mut self.r#manufacturer)
	}
	fn get_material(&self) -> &[MaterialProperty] {
		self.r#material.as_slice()
	}
	fn take_material(&mut self) -> Vec<MaterialProperty> {
		std::mem::take(&mut self.r#material)
	}
	fn get_mobile_url(&self) -> &[MobileUrlProperty] {
		self.r#mobile_url.as_slice()
	}
	fn take_mobile_url(&mut self) -> Vec<MobileUrlProperty> {
		std::mem::take(&mut self.r#mobile_url)
	}
	fn get_model(&self) -> &[ModelProperty] {
		self.r#model.as_slice()
	}
	fn take_model(&mut self) -> Vec<ModelProperty> {
		std::mem::take(&mut self.r#model)
	}
	fn get_mpn(&self) -> &[MpnProperty] {
		self.r#mpn.as_slice()
	}
	fn take_mpn(&mut self) -> Vec<MpnProperty> {
		std::mem::take(&mut self.r#mpn)
	}
	fn get_negative_notes(&self) -> &[NegativeNotesProperty] {
		self.r#negative_notes.as_slice()
	}
	fn take_negative_notes(&mut self) -> Vec<NegativeNotesProperty> {
		std::mem::take(&mut self.r#negative_notes)
	}
	fn get_nsn(&self) -> &[NsnProperty] {
		self.r#nsn.as_slice()
	}
	fn take_nsn(&mut self) -> Vec<NsnProperty> {
		std::mem::take(&mut self.r#nsn)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_pattern(&self) -> &[PatternProperty] {
		self.r#pattern.as_slice()
	}
	fn take_pattern(&mut self) -> Vec<PatternProperty> {
		std::mem::take(&mut self.r#pattern)
	}
	fn get_positive_notes(&self) -> &[PositiveNotesProperty] {
		self.r#positive_notes.as_slice()
	}
	fn take_positive_notes(&mut self) -> Vec<PositiveNotesProperty> {
		std::mem::take(&mut self.r#positive_notes)
	}
	fn get_product_id(&self) -> &[ProductIdProperty] {
		self.r#product_id.as_slice()
	}
	fn take_product_id(&mut self) -> Vec<ProductIdProperty> {
		std::mem::take(&mut self.r#product_id)
	}
	fn get_production_date(&self) -> &[ProductionDateProperty] {
		self.r#production_date.as_slice()
	}
	fn take_production_date(&mut self) -> Vec<ProductionDateProperty> {
		std::mem::take(&mut self.r#production_date)
	}
	fn get_purchase_date(&self) -> &[PurchaseDateProperty] {
		self.r#purchase_date.as_slice()
	}
	fn take_purchase_date(&mut self) -> Vec<PurchaseDateProperty> {
		std::mem::take(&mut self.r#purchase_date)
	}
	fn get_recycled_content_percentage(&self) -> &[RecycledContentPercentageProperty] {
		self.r#recycled_content_percentage.as_slice()
	}
	fn take_recycled_content_percentage(&mut self) -> Vec<RecycledContentPercentageProperty> {
		std::mem::take(&mut self.r#recycled_content_percentage)
	}
	fn get_release_date(&self) -> &[ReleaseDateProperty] {
		self.r#release_date.as_slice()
	}
	fn take_release_date(&mut self) -> Vec<ReleaseDateProperty> {
		std::mem::take(&mut self.r#release_date)
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
	fn get_size(&self) -> &[SizeProperty] {
		self.r#size.as_slice()
	}
	fn take_size(&mut self) -> Vec<SizeProperty> {
		std::mem::take(&mut self.r#size)
	}
	fn get_sku(&self) -> &[SkuProperty] {
		self.r#sku.as_slice()
	}
	fn take_sku(&mut self) -> Vec<SkuProperty> {
		std::mem::take(&mut self.r#sku)
	}
	fn get_slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn take_slogan(&mut self) -> Vec<SloganProperty> {
		std::mem::take(&mut self.r#slogan)
	}
	fn get_specification(&self) -> &[SpecificationProperty] {
		self.r#specification.as_slice()
	}
	fn take_specification(&mut self) -> Vec<SpecificationProperty> {
		std::mem::take(&mut self.r#specification)
	}
	fn get_substance_of_concern(&self) -> &[SubstanceOfConcernProperty] {
		self.r#substance_of_concern.as_slice()
	}
	fn take_substance_of_concern(&mut self) -> Vec<SubstanceOfConcernProperty> {
		std::mem::take(&mut self.r#substance_of_concern)
	}
	fn get_weight(&self) -> &[WeightProperty] {
		self.r#weight.as_slice()
	}
	fn take_weight(&mut self) -> Vec<WeightProperty> {
		std::mem::take(&mut self.r#weight)
	}
	fn get_width(&self) -> &[WidthProperty] {
		self.r#width.as_slice()
	}
	fn take_width(&mut self) -> Vec<WidthProperty> {
		std::mem::take(&mut self.r#width)
	}
}
impl ThingTrait for SomeProducts {
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
