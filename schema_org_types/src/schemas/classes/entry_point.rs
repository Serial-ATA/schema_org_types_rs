use super::*;
/// <https://schema.org/EntryPoint>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct EntryPoint {
	/// <https://schema.org/actionApplication>
	#[cfg_attr(feature = "serde", serde(rename = "actionApplication"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#action_application: Vec<ActionApplicationProperty>,
	/// <https://schema.org/actionPlatform>
	#[cfg_attr(feature = "serde", serde(rename = "actionPlatform"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#action_platform: Vec<ActionPlatformProperty>,
	/// <https://schema.org/application>
	#[deprecated = "This schema is superseded by <https://schema.org/actionApplication>."]
	#[cfg_attr(feature = "serde", serde(rename = "application"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#application: Vec<ApplicationProperty>,
	/// <https://schema.org/contentType>
	#[cfg_attr(feature = "serde", serde(rename = "contentType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#content_type: Vec<ContentTypeProperty>,
	/// <https://schema.org/encodingType>
	#[cfg_attr(feature = "serde", serde(rename = "encodingType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#encoding_type: Vec<EncodingTypeProperty>,
	/// <https://schema.org/httpMethod>
	#[cfg_attr(feature = "serde", serde(rename = "httpMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#http_method: Vec<HttpMethodProperty>,
	/// <https://schema.org/urlTemplate>
	#[cfg_attr(feature = "serde", serde(rename = "urlTemplate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#url_template: Vec<UrlTemplateProperty>,
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
/// This trait is for properties from <https://schema.org/EntryPoint>.
pub trait EntryPointTrait {
	/// Get <https://schema.org/actionApplication> from [`Self`] as borrowed slice.
	fn get_action_application(&self) -> &[ActionApplicationProperty];
	/// Take <https://schema.org/actionApplication> from [`Self`] as owned vector.
	fn take_action_application(&mut self) -> Vec<ActionApplicationProperty>;
	/// Get <https://schema.org/actionPlatform> from [`Self`] as borrowed slice.
	fn get_action_platform(&self) -> &[ActionPlatformProperty];
	/// Take <https://schema.org/actionPlatform> from [`Self`] as owned vector.
	fn take_action_platform(&mut self) -> Vec<ActionPlatformProperty>;
	/// Get <https://schema.org/application> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/actionApplication>."]
	fn get_application(&self) -> &[ApplicationProperty];
	/// Take <https://schema.org/application> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/actionApplication>."]
	fn take_application(&mut self) -> Vec<ApplicationProperty>;
	/// Get <https://schema.org/contentType> from [`Self`] as borrowed slice.
	fn get_content_type(&self) -> &[ContentTypeProperty];
	/// Take <https://schema.org/contentType> from [`Self`] as owned vector.
	fn take_content_type(&mut self) -> Vec<ContentTypeProperty>;
	/// Get <https://schema.org/encodingType> from [`Self`] as borrowed slice.
	fn get_encoding_type(&self) -> &[EncodingTypeProperty];
	/// Take <https://schema.org/encodingType> from [`Self`] as owned vector.
	fn take_encoding_type(&mut self) -> Vec<EncodingTypeProperty>;
	/// Get <https://schema.org/httpMethod> from [`Self`] as borrowed slice.
	fn get_http_method(&self) -> &[HttpMethodProperty];
	/// Take <https://schema.org/httpMethod> from [`Self`] as owned vector.
	fn take_http_method(&mut self) -> Vec<HttpMethodProperty>;
	/// Get <https://schema.org/urlTemplate> from [`Self`] as borrowed slice.
	fn get_url_template(&self) -> &[UrlTemplateProperty];
	/// Take <https://schema.org/urlTemplate> from [`Self`] as owned vector.
	fn take_url_template(&mut self) -> Vec<UrlTemplateProperty>;
}
impl EntryPointTrait for EntryPoint {
	fn get_action_application(&self) -> &[ActionApplicationProperty] {
		self.r#action_application.as_slice()
	}
	fn take_action_application(&mut self) -> Vec<ActionApplicationProperty> {
		std::mem::take(&mut self.r#action_application)
	}
	fn get_action_platform(&self) -> &[ActionPlatformProperty] {
		self.r#action_platform.as_slice()
	}
	fn take_action_platform(&mut self) -> Vec<ActionPlatformProperty> {
		std::mem::take(&mut self.r#action_platform)
	}
	fn get_application(&self) -> &[ApplicationProperty] {
		self.r#application.as_slice()
	}
	fn take_application(&mut self) -> Vec<ApplicationProperty> {
		std::mem::take(&mut self.r#application)
	}
	fn get_content_type(&self) -> &[ContentTypeProperty] {
		self.r#content_type.as_slice()
	}
	fn take_content_type(&mut self) -> Vec<ContentTypeProperty> {
		std::mem::take(&mut self.r#content_type)
	}
	fn get_encoding_type(&self) -> &[EncodingTypeProperty] {
		self.r#encoding_type.as_slice()
	}
	fn take_encoding_type(&mut self) -> Vec<EncodingTypeProperty> {
		std::mem::take(&mut self.r#encoding_type)
	}
	fn get_http_method(&self) -> &[HttpMethodProperty] {
		self.r#http_method.as_slice()
	}
	fn take_http_method(&mut self) -> Vec<HttpMethodProperty> {
		std::mem::take(&mut self.r#http_method)
	}
	fn get_url_template(&self) -> &[UrlTemplateProperty] {
		self.r#url_template.as_slice()
	}
	fn take_url_template(&mut self) -> Vec<UrlTemplateProperty> {
		std::mem::take(&mut self.r#url_template)
	}
}
impl ThingTrait for EntryPoint {
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
