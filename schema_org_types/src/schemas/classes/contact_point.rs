use super::*;
/// <https://schema.org/ContactPoint>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ContactPoint {
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
	/// <https://schema.org/availableLanguage>
	#[cfg_attr(feature = "serde", serde(rename = "availableLanguage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#available_language: Vec<AvailableLanguageProperty>,
	/// <https://schema.org/contactOption>
	#[cfg_attr(feature = "serde", serde(rename = "contactOption"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contact_option: Vec<ContactOptionProperty>,
	/// <https://schema.org/contactType>
	#[cfg_attr(feature = "serde", serde(rename = "contactType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contact_type: Vec<ContactTypeProperty>,
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
	/// <https://schema.org/productSupported>
	#[cfg_attr(feature = "serde", serde(rename = "productSupported"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#product_supported: Vec<ProductSupportedProperty>,
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
/// This trait is for properties from <https://schema.org/ContactPoint>.
pub trait ContactPointTrait {
	/// Get <https://schema.org/areaServed> from [`Self`] as borrowed slice.
	fn get_area_served(&self) -> &[AreaServedProperty];
	/// Take <https://schema.org/areaServed> from [`Self`] as owned vector.
	fn take_area_served(&mut self) -> Vec<AreaServedProperty>;
	/// Get <https://schema.org/availableLanguage> from [`Self`] as borrowed slice.
	fn get_available_language(&self) -> &[AvailableLanguageProperty];
	/// Take <https://schema.org/availableLanguage> from [`Self`] as owned vector.
	fn take_available_language(&mut self) -> Vec<AvailableLanguageProperty>;
	/// Get <https://schema.org/contactOption> from [`Self`] as borrowed slice.
	fn get_contact_option(&self) -> &[ContactOptionProperty];
	/// Take <https://schema.org/contactOption> from [`Self`] as owned vector.
	fn take_contact_option(&mut self) -> Vec<ContactOptionProperty>;
	/// Get <https://schema.org/contactType> from [`Self`] as borrowed slice.
	fn get_contact_type(&self) -> &[ContactTypeProperty];
	/// Take <https://schema.org/contactType> from [`Self`] as owned vector.
	fn take_contact_type(&mut self) -> Vec<ContactTypeProperty>;
	/// Get <https://schema.org/email> from [`Self`] as borrowed slice.
	fn get_email(&self) -> &[EmailProperty];
	/// Take <https://schema.org/email> from [`Self`] as owned vector.
	fn take_email(&mut self) -> Vec<EmailProperty>;
	/// Get <https://schema.org/faxNumber> from [`Self`] as borrowed slice.
	fn get_fax_number(&self) -> &[FaxNumberProperty];
	/// Take <https://schema.org/faxNumber> from [`Self`] as owned vector.
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty>;
	/// Get <https://schema.org/hoursAvailable> from [`Self`] as borrowed slice.
	fn get_hours_available(&self) -> &[HoursAvailableProperty];
	/// Take <https://schema.org/hoursAvailable> from [`Self`] as owned vector.
	fn take_hours_available(&mut self) -> Vec<HoursAvailableProperty>;
	/// Get <https://schema.org/productSupported> from [`Self`] as borrowed slice.
	fn get_product_supported(&self) -> &[ProductSupportedProperty];
	/// Take <https://schema.org/productSupported> from [`Self`] as owned vector.
	fn take_product_supported(&mut self) -> Vec<ProductSupportedProperty>;
	/// Get <https://schema.org/serviceArea> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
	fn get_service_area(&self) -> &[ServiceAreaProperty];
	/// Take <https://schema.org/serviceArea> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/areaServed>."]
	fn take_service_area(&mut self) -> Vec<ServiceAreaProperty>;
	/// Get <https://schema.org/telephone> from [`Self`] as borrowed slice.
	fn get_telephone(&self) -> &[TelephoneProperty];
	/// Take <https://schema.org/telephone> from [`Self`] as owned vector.
	fn take_telephone(&mut self) -> Vec<TelephoneProperty>;
}
impl ContactPointTrait for ContactPoint {
	fn get_area_served(&self) -> &[AreaServedProperty] {
		self.r#area_served.as_slice()
	}
	fn take_area_served(&mut self) -> Vec<AreaServedProperty> {
		std::mem::take(&mut self.r#area_served)
	}
	fn get_available_language(&self) -> &[AvailableLanguageProperty] {
		self.r#available_language.as_slice()
	}
	fn take_available_language(&mut self) -> Vec<AvailableLanguageProperty> {
		std::mem::take(&mut self.r#available_language)
	}
	fn get_contact_option(&self) -> &[ContactOptionProperty] {
		self.r#contact_option.as_slice()
	}
	fn take_contact_option(&mut self) -> Vec<ContactOptionProperty> {
		std::mem::take(&mut self.r#contact_option)
	}
	fn get_contact_type(&self) -> &[ContactTypeProperty] {
		self.r#contact_type.as_slice()
	}
	fn take_contact_type(&mut self) -> Vec<ContactTypeProperty> {
		std::mem::take(&mut self.r#contact_type)
	}
	fn get_email(&self) -> &[EmailProperty] {
		self.r#email.as_slice()
	}
	fn take_email(&mut self) -> Vec<EmailProperty> {
		std::mem::take(&mut self.r#email)
	}
	fn get_fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty> {
		std::mem::take(&mut self.r#fax_number)
	}
	fn get_hours_available(&self) -> &[HoursAvailableProperty] {
		self.r#hours_available.as_slice()
	}
	fn take_hours_available(&mut self) -> Vec<HoursAvailableProperty> {
		std::mem::take(&mut self.r#hours_available)
	}
	fn get_product_supported(&self) -> &[ProductSupportedProperty] {
		self.r#product_supported.as_slice()
	}
	fn take_product_supported(&mut self) -> Vec<ProductSupportedProperty> {
		std::mem::take(&mut self.r#product_supported)
	}
	fn get_service_area(&self) -> &[ServiceAreaProperty] {
		self.r#service_area.as_slice()
	}
	fn take_service_area(&mut self) -> Vec<ServiceAreaProperty> {
		std::mem::take(&mut self.r#service_area)
	}
	fn get_telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn take_telephone(&mut self) -> Vec<TelephoneProperty> {
		std::mem::take(&mut self.r#telephone)
	}
}
impl StructuredValueTrait for ContactPoint {}
impl ThingTrait for ContactPoint {
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
