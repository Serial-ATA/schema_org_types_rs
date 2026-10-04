use super::*;
/// <https://schema.org/CreativeWork>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct CreativeWork {
	/// <https://schema.org/about>
	#[cfg_attr(feature = "serde", serde(rename = "about"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#about: Vec<AboutProperty>,
	/// <https://schema.org/abstract>
	#[cfg_attr(feature = "serde", serde(rename = "abstract"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#abstract: Vec<AbstractProperty>,
	/// <https://schema.org/accessMode>
	#[cfg_attr(feature = "serde", serde(rename = "accessMode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#access_mode: Vec<AccessModeProperty>,
	/// <https://schema.org/accessModeSufficient>
	#[cfg_attr(feature = "serde", serde(rename = "accessModeSufficient"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#access_mode_sufficient: Vec<AccessModeSufficientProperty>,
	/// <https://schema.org/accessibilityAPI>
	#[cfg_attr(feature = "serde", serde(rename = "accessibilityAPI"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accessibility_api: Vec<AccessibilityApiProperty>,
	/// <https://schema.org/accessibilityControl>
	#[cfg_attr(feature = "serde", serde(rename = "accessibilityControl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accessibility_control: Vec<AccessibilityControlProperty>,
	/// <https://schema.org/accessibilityFeature>
	#[cfg_attr(feature = "serde", serde(rename = "accessibilityFeature"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accessibility_feature: Vec<AccessibilityFeatureProperty>,
	/// <https://schema.org/accessibilityHazard>
	#[cfg_attr(feature = "serde", serde(rename = "accessibilityHazard"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accessibility_hazard: Vec<AccessibilityHazardProperty>,
	/// <https://schema.org/accessibilitySummary>
	#[cfg_attr(feature = "serde", serde(rename = "accessibilitySummary"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accessibility_summary: Vec<AccessibilitySummaryProperty>,
	/// <https://schema.org/accountablePerson>
	#[cfg_attr(feature = "serde", serde(rename = "accountablePerson"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accountable_person: Vec<AccountablePersonProperty>,
	/// <https://schema.org/acquireLicensePage>
	#[cfg_attr(feature = "serde", serde(rename = "acquireLicensePage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#acquire_license_page: Vec<AcquireLicensePageProperty>,
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
	/// <https://schema.org/alternativeHeadline>
	#[cfg_attr(feature = "serde", serde(rename = "alternativeHeadline"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#alternative_headline: Vec<AlternativeHeadlineProperty>,
	/// <https://schema.org/archivedAt>
	#[cfg_attr(feature = "serde", serde(rename = "archivedAt"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#archived_at: Vec<ArchivedAtProperty>,
	/// <https://schema.org/assesses>
	#[cfg_attr(feature = "serde", serde(rename = "assesses"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#assesses: Vec<AssessesProperty>,
	/// <https://schema.org/associatedMedia>
	#[cfg_attr(feature = "serde", serde(rename = "associatedMedia"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#associated_media: Vec<AssociatedMediaProperty>,
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
	/// <https://schema.org/audio>
	#[cfg_attr(feature = "serde", serde(rename = "audio"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#audio: Vec<AudioProperty>,
	/// <https://schema.org/author>
	#[cfg_attr(feature = "serde", serde(rename = "author"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#author: Vec<AuthorProperty>,
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
	/// <https://schema.org/character>
	#[cfg_attr(feature = "serde", serde(rename = "character"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#character: Vec<CharacterProperty>,
	/// <https://schema.org/citation>
	#[cfg_attr(feature = "serde", serde(rename = "citation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#citation: Vec<CitationProperty>,
	/// <https://schema.org/comment>
	#[cfg_attr(feature = "serde", serde(rename = "comment"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#comment: Vec<CommentProperty>,
	/// <https://schema.org/commentCount>
	#[cfg_attr(feature = "serde", serde(rename = "commentCount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#comment_count: Vec<CommentCountProperty>,
	/// <https://schema.org/conditionsOfAccess>
	#[cfg_attr(feature = "serde", serde(rename = "conditionsOfAccess"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#conditions_of_access: Vec<ConditionsOfAccessProperty>,
	/// <https://schema.org/contentLocation>
	#[cfg_attr(feature = "serde", serde(rename = "contentLocation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#content_location: Vec<ContentLocationProperty>,
	/// <https://schema.org/contentRating>
	#[cfg_attr(feature = "serde", serde(rename = "contentRating"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#content_rating: Vec<ContentRatingProperty>,
	/// <https://schema.org/contentReferenceTime>
	#[cfg_attr(feature = "serde", serde(rename = "contentReferenceTime"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#content_reference_time: Vec<ContentReferenceTimeProperty>,
	/// <https://schema.org/contributor>
	#[cfg_attr(feature = "serde", serde(rename = "contributor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#contributor: Vec<ContributorProperty>,
	/// <https://schema.org/copyrightHolder>
	#[cfg_attr(feature = "serde", serde(rename = "copyrightHolder"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#copyright_holder: Vec<CopyrightHolderProperty>,
	/// <https://schema.org/copyrightNotice>
	#[cfg_attr(feature = "serde", serde(rename = "copyrightNotice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#copyright_notice: Vec<CopyrightNoticeProperty>,
	/// <https://schema.org/copyrightYear>
	#[cfg_attr(feature = "serde", serde(rename = "copyrightYear"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#copyright_year: Vec<CopyrightYearProperty>,
	/// <https://schema.org/correction>
	#[cfg_attr(feature = "serde", serde(rename = "correction"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#correction: Vec<CorrectionProperty>,
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
	/// <https://schema.org/creativeWorkStatus>
	#[cfg_attr(feature = "serde", serde(rename = "creativeWorkStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#creative_work_status: Vec<CreativeWorkStatusProperty>,
	/// <https://schema.org/creator>
	#[cfg_attr(feature = "serde", serde(rename = "creator"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#creator: Vec<CreatorProperty>,
	/// <https://schema.org/creditText>
	#[cfg_attr(feature = "serde", serde(rename = "creditText"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#credit_text: Vec<CreditTextProperty>,
	/// <https://schema.org/dateCreated>
	#[cfg_attr(feature = "serde", serde(rename = "dateCreated"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#date_created: Vec<DateCreatedProperty>,
	/// <https://schema.org/dateModified>
	#[cfg_attr(feature = "serde", serde(rename = "dateModified"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#date_modified: Vec<DateModifiedProperty>,
	/// <https://schema.org/datePublished>
	#[cfg_attr(feature = "serde", serde(rename = "datePublished"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#date_published: Vec<DatePublishedProperty>,
	/// <https://schema.org/digitalSourceType>
	#[cfg_attr(feature = "serde", serde(rename = "digitalSourceType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#digital_source_type: Vec<DigitalSourceTypeProperty>,
	/// <https://schema.org/discussionUrl>
	#[cfg_attr(feature = "serde", serde(rename = "discussionUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#discussion_url: Vec<DiscussionUrlProperty>,
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
	/// <https://schema.org/editEIDR>
	#[cfg_attr(feature = "serde", serde(rename = "editEIDR"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#edit_eidr: Vec<EditEidrProperty>,
	/// <https://schema.org/editor>
	#[cfg_attr(feature = "serde", serde(rename = "editor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#editor: Vec<EditorProperty>,
	/// <https://schema.org/educationalAlignment>
	#[cfg_attr(feature = "serde", serde(rename = "educationalAlignment"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#educational_alignment: Vec<EducationalAlignmentProperty>,
	/// <https://schema.org/educationalLevel>
	#[cfg_attr(feature = "serde", serde(rename = "educationalLevel"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#educational_level: Vec<EducationalLevelProperty>,
	/// <https://schema.org/educationalUse>
	#[cfg_attr(feature = "serde", serde(rename = "educationalUse"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#educational_use: Vec<EducationalUseProperty>,
	/// <https://schema.org/encoding>
	#[cfg_attr(feature = "serde", serde(rename = "encoding"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#encoding: Vec<EncodingProperty>,
	/// <https://schema.org/encodingFormat>
	#[cfg_attr(feature = "serde", serde(rename = "encodingFormat"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#encoding_format: Vec<EncodingFormatProperty>,
	/// <https://schema.org/encodings>
	#[deprecated = "This schema is superseded by <https://schema.org/encoding>."]
	#[cfg_attr(feature = "serde", serde(rename = "encodings"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#encodings: Vec<EncodingsProperty>,
	/// <https://schema.org/exampleOfWork>
	#[cfg_attr(feature = "serde", serde(rename = "exampleOfWork"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#example_of_work: Vec<ExampleOfWorkProperty>,
	/// <https://schema.org/expires>
	#[cfg_attr(feature = "serde", serde(rename = "expires"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#expires: Vec<ExpiresProperty>,
	/// <https://schema.org/fileFormat>
	#[deprecated = "This schema is superseded by <https://schema.org/encodingFormat>."]
	#[cfg_attr(feature = "serde", serde(rename = "fileFormat"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#file_format: Vec<FileFormatProperty>,
	/// <https://schema.org/funder>
	#[cfg_attr(feature = "serde", serde(rename = "funder"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#funder: Vec<FunderProperty>,
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
	/// <https://schema.org/genre>
	#[cfg_attr(feature = "serde", serde(rename = "genre"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#genre: Vec<GenreProperty>,
	/// <https://schema.org/hasPart>
	#[cfg_attr(feature = "serde", serde(rename = "hasPart"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#has_part: Vec<HasPartProperty>,
	/// <https://schema.org/headline>
	#[cfg_attr(feature = "serde", serde(rename = "headline"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#headline: Vec<HeadlineProperty>,
	/// <https://schema.org/inLanguage>
	#[cfg_attr(feature = "serde", serde(rename = "inLanguage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#in_language: Vec<InLanguageProperty>,
	/// <https://schema.org/interactionStatistic>
	#[cfg_attr(feature = "serde", serde(rename = "interactionStatistic"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	/// <https://schema.org/interactivityType>
	#[cfg_attr(feature = "serde", serde(rename = "interactivityType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#interactivity_type: Vec<InteractivityTypeProperty>,
	/// <https://schema.org/interpretedAsClaim>
	#[cfg_attr(feature = "serde", serde(rename = "interpretedAsClaim"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#interpreted_as_claim: Vec<InterpretedAsClaimProperty>,
	/// <https://schema.org/isAccessibleForFree>
	#[cfg_attr(feature = "serde", serde(rename = "isAccessibleForFree"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	/// <https://schema.org/isBasedOn>
	#[cfg_attr(feature = "serde", serde(rename = "isBasedOn"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_based_on: Vec<IsBasedOnProperty>,
	/// <https://schema.org/isBasedOnUrl>
	#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
	#[cfg_attr(feature = "serde", serde(rename = "isBasedOnUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_based_on_url: Vec<IsBasedOnUrlProperty>,
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
	/// <https://schema.org/isPartOf>
	#[cfg_attr(feature = "serde", serde(rename = "isPartOf"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_part_of: Vec<IsPartOfProperty>,
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
	/// <https://schema.org/learningResourceType>
	#[cfg_attr(feature = "serde", serde(rename = "learningResourceType"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#learning_resource_type: Vec<LearningResourceTypeProperty>,
	/// <https://schema.org/license>
	#[cfg_attr(feature = "serde", serde(rename = "license"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#license: Vec<LicenseProperty>,
	/// <https://schema.org/locationCreated>
	#[cfg_attr(feature = "serde", serde(rename = "locationCreated"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#location_created: Vec<LocationCreatedProperty>,
	/// <https://schema.org/mainEntity>
	#[cfg_attr(feature = "serde", serde(rename = "mainEntity"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#main_entity: Vec<MainEntityProperty>,
	/// <https://schema.org/maintainer>
	#[cfg_attr(feature = "serde", serde(rename = "maintainer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#maintainer: Vec<MaintainerProperty>,
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
	/// <https://schema.org/materialExtent>
	#[cfg_attr(feature = "serde", serde(rename = "materialExtent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#material_extent: Vec<MaterialExtentProperty>,
	/// <https://schema.org/mentions>
	#[cfg_attr(feature = "serde", serde(rename = "mentions"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#mentions: Vec<MentionsProperty>,
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
	/// <https://schema.org/position>
	#[cfg_attr(feature = "serde", serde(rename = "position"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#position: Vec<PositionProperty>,
	/// <https://schema.org/producer>
	#[cfg_attr(feature = "serde", serde(rename = "producer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#producer: Vec<ProducerProperty>,
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
	/// <https://schema.org/publication>
	#[cfg_attr(feature = "serde", serde(rename = "publication"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#publication: Vec<PublicationProperty>,
	/// <https://schema.org/publisher>
	#[cfg_attr(feature = "serde", serde(rename = "publisher"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#publisher: Vec<PublisherProperty>,
	/// <https://schema.org/publisherImprint>
	#[cfg_attr(feature = "serde", serde(rename = "publisherImprint"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#publisher_imprint: Vec<PublisherImprintProperty>,
	/// <https://schema.org/publishingPrinciples>
	#[cfg_attr(feature = "serde", serde(rename = "publishingPrinciples"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	/// <https://schema.org/recordedAt>
	#[cfg_attr(feature = "serde", serde(rename = "recordedAt"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#recorded_at: Vec<RecordedAtProperty>,
	/// <https://schema.org/releasedEvent>
	#[cfg_attr(feature = "serde", serde(rename = "releasedEvent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#released_event: Vec<ReleasedEventProperty>,
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
	/// <https://schema.org/schemaVersion>
	#[cfg_attr(feature = "serde", serde(rename = "schemaVersion"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#schema_version: Vec<SchemaVersionProperty>,
	/// <https://schema.org/sdDatePublished>
	#[cfg_attr(feature = "serde", serde(rename = "sdDatePublished"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sd_date_published: Vec<SdDatePublishedProperty>,
	/// <https://schema.org/sdLicense>
	#[cfg_attr(feature = "serde", serde(rename = "sdLicense"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sd_license: Vec<SdLicenseProperty>,
	/// <https://schema.org/sdPublisher>
	#[cfg_attr(feature = "serde", serde(rename = "sdPublisher"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sd_publisher: Vec<SdPublisherProperty>,
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
	/// <https://schema.org/sourceOrganization>
	#[cfg_attr(feature = "serde", serde(rename = "sourceOrganization"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#source_organization: Vec<SourceOrganizationProperty>,
	/// <https://schema.org/spatial>
	#[cfg_attr(feature = "serde", serde(rename = "spatial"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#spatial: Vec<SpatialProperty>,
	/// <https://schema.org/spatialCoverage>
	#[cfg_attr(feature = "serde", serde(rename = "spatialCoverage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#spatial_coverage: Vec<SpatialCoverageProperty>,
	/// <https://schema.org/sponsor>
	#[cfg_attr(feature = "serde", serde(rename = "sponsor"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sponsor: Vec<SponsorProperty>,
	/// <https://schema.org/teaches>
	#[cfg_attr(feature = "serde", serde(rename = "teaches"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#teaches: Vec<TeachesProperty>,
	/// <https://schema.org/temporal>
	#[cfg_attr(feature = "serde", serde(rename = "temporal"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#temporal: Vec<TemporalProperty>,
	/// <https://schema.org/temporalCoverage>
	#[cfg_attr(feature = "serde", serde(rename = "temporalCoverage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#temporal_coverage: Vec<TemporalCoverageProperty>,
	/// <https://schema.org/text>
	#[cfg_attr(feature = "serde", serde(rename = "text"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#text: Vec<TextProperty>,
	/// <https://schema.org/thumbnail>
	#[cfg_attr(feature = "serde", serde(rename = "thumbnail"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#thumbnail: Vec<ThumbnailProperty>,
	/// <https://schema.org/thumbnailUrl>
	#[cfg_attr(feature = "serde", serde(rename = "thumbnailUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#thumbnail_url: Vec<ThumbnailUrlProperty>,
	/// <https://schema.org/timeRequired>
	#[cfg_attr(feature = "serde", serde(rename = "timeRequired"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#time_required: Vec<TimeRequiredProperty>,
	/// <https://schema.org/translationOfWork>
	#[cfg_attr(feature = "serde", serde(rename = "translationOfWork"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#translation_of_work: Vec<TranslationOfWorkProperty>,
	/// <https://schema.org/translator>
	#[cfg_attr(feature = "serde", serde(rename = "translator"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#translator: Vec<TranslatorProperty>,
	/// <https://schema.org/typicalAgeRange>
	#[cfg_attr(feature = "serde", serde(rename = "typicalAgeRange"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
	/// <https://schema.org/usageInfo>
	#[cfg_attr(feature = "serde", serde(rename = "usageInfo"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#usage_info: Vec<UsageInfoProperty>,
	/// <https://schema.org/version>
	#[cfg_attr(feature = "serde", serde(rename = "version"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#version: Vec<VersionProperty>,
	/// <https://schema.org/video>
	#[cfg_attr(feature = "serde", serde(rename = "video"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#video: Vec<VideoProperty>,
	/// <https://schema.org/wordCount>
	#[cfg_attr(feature = "serde", serde(rename = "wordCount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#word_count: Vec<WordCountProperty>,
	/// <https://schema.org/workExample>
	#[cfg_attr(feature = "serde", serde(rename = "workExample"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#work_example: Vec<WorkExampleProperty>,
	/// <https://schema.org/workTranslation>
	#[cfg_attr(feature = "serde", serde(rename = "workTranslation"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#work_translation: Vec<WorkTranslationProperty>,
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
/// This trait is for properties from <https://schema.org/CreativeWork>.
pub trait CreativeWorkTrait {
	/// Get <https://schema.org/about> from [`Self`] as borrowed slice.
	fn get_about(&self) -> &[AboutProperty];
	/// Take <https://schema.org/about> from [`Self`] as owned vector.
	fn take_about(&mut self) -> Vec<AboutProperty>;
	/// Get <https://schema.org/abstract> from [`Self`] as borrowed slice.
	fn get_abstract(&self) -> &[AbstractProperty];
	/// Take <https://schema.org/abstract> from [`Self`] as owned vector.
	fn take_abstract(&mut self) -> Vec<AbstractProperty>;
	/// Get <https://schema.org/accessMode> from [`Self`] as borrowed slice.
	fn get_access_mode(&self) -> &[AccessModeProperty];
	/// Take <https://schema.org/accessMode> from [`Self`] as owned vector.
	fn take_access_mode(&mut self) -> Vec<AccessModeProperty>;
	/// Get <https://schema.org/accessModeSufficient> from [`Self`] as borrowed slice.
	fn get_access_mode_sufficient(&self) -> &[AccessModeSufficientProperty];
	/// Take <https://schema.org/accessModeSufficient> from [`Self`] as owned vector.
	fn take_access_mode_sufficient(&mut self) -> Vec<AccessModeSufficientProperty>;
	/// Get <https://schema.org/accessibilityAPI> from [`Self`] as borrowed slice.
	fn get_accessibility_api(&self) -> &[AccessibilityApiProperty];
	/// Take <https://schema.org/accessibilityAPI> from [`Self`] as owned vector.
	fn take_accessibility_api(&mut self) -> Vec<AccessibilityApiProperty>;
	/// Get <https://schema.org/accessibilityControl> from [`Self`] as borrowed slice.
	fn get_accessibility_control(&self) -> &[AccessibilityControlProperty];
	/// Take <https://schema.org/accessibilityControl> from [`Self`] as owned vector.
	fn take_accessibility_control(&mut self) -> Vec<AccessibilityControlProperty>;
	/// Get <https://schema.org/accessibilityFeature> from [`Self`] as borrowed slice.
	fn get_accessibility_feature(&self) -> &[AccessibilityFeatureProperty];
	/// Take <https://schema.org/accessibilityFeature> from [`Self`] as owned vector.
	fn take_accessibility_feature(&mut self) -> Vec<AccessibilityFeatureProperty>;
	/// Get <https://schema.org/accessibilityHazard> from [`Self`] as borrowed slice.
	fn get_accessibility_hazard(&self) -> &[AccessibilityHazardProperty];
	/// Take <https://schema.org/accessibilityHazard> from [`Self`] as owned vector.
	fn take_accessibility_hazard(&mut self) -> Vec<AccessibilityHazardProperty>;
	/// Get <https://schema.org/accessibilitySummary> from [`Self`] as borrowed slice.
	fn get_accessibility_summary(&self) -> &[AccessibilitySummaryProperty];
	/// Take <https://schema.org/accessibilitySummary> from [`Self`] as owned vector.
	fn take_accessibility_summary(&mut self) -> Vec<AccessibilitySummaryProperty>;
	/// Get <https://schema.org/accountablePerson> from [`Self`] as borrowed slice.
	fn get_accountable_person(&self) -> &[AccountablePersonProperty];
	/// Take <https://schema.org/accountablePerson> from [`Self`] as owned vector.
	fn take_accountable_person(&mut self) -> Vec<AccountablePersonProperty>;
	/// Get <https://schema.org/acquireLicensePage> from [`Self`] as borrowed slice.
	fn get_acquire_license_page(&self) -> &[AcquireLicensePageProperty];
	/// Take <https://schema.org/acquireLicensePage> from [`Self`] as owned vector.
	fn take_acquire_license_page(&mut self) -> Vec<AcquireLicensePageProperty>;
	/// Get <https://schema.org/aggregateRating> from [`Self`] as borrowed slice.
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty];
	/// Take <https://schema.org/aggregateRating> from [`Self`] as owned vector.
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty>;
	/// Get <https://schema.org/alternativeHeadline> from [`Self`] as borrowed slice.
	fn get_alternative_headline(&self) -> &[AlternativeHeadlineProperty];
	/// Take <https://schema.org/alternativeHeadline> from [`Self`] as owned vector.
	fn take_alternative_headline(&mut self) -> Vec<AlternativeHeadlineProperty>;
	/// Get <https://schema.org/archivedAt> from [`Self`] as borrowed slice.
	fn get_archived_at(&self) -> &[ArchivedAtProperty];
	/// Take <https://schema.org/archivedAt> from [`Self`] as owned vector.
	fn take_archived_at(&mut self) -> Vec<ArchivedAtProperty>;
	/// Get <https://schema.org/assesses> from [`Self`] as borrowed slice.
	fn get_assesses(&self) -> &[AssessesProperty];
	/// Take <https://schema.org/assesses> from [`Self`] as owned vector.
	fn take_assesses(&mut self) -> Vec<AssessesProperty>;
	/// Get <https://schema.org/associatedMedia> from [`Self`] as borrowed slice.
	fn get_associated_media(&self) -> &[AssociatedMediaProperty];
	/// Take <https://schema.org/associatedMedia> from [`Self`] as owned vector.
	fn take_associated_media(&mut self) -> Vec<AssociatedMediaProperty>;
	/// Get <https://schema.org/audience> from [`Self`] as borrowed slice.
	fn get_audience(&self) -> &[AudienceProperty];
	/// Take <https://schema.org/audience> from [`Self`] as owned vector.
	fn take_audience(&mut self) -> Vec<AudienceProperty>;
	/// Get <https://schema.org/audio> from [`Self`] as borrowed slice.
	fn get_audio(&self) -> &[AudioProperty];
	/// Take <https://schema.org/audio> from [`Self`] as owned vector.
	fn take_audio(&mut self) -> Vec<AudioProperty>;
	/// Get <https://schema.org/author> from [`Self`] as borrowed slice.
	fn get_author(&self) -> &[AuthorProperty];
	/// Take <https://schema.org/author> from [`Self`] as owned vector.
	fn take_author(&mut self) -> Vec<AuthorProperty>;
	/// Get <https://schema.org/award> from [`Self`] as borrowed slice.
	fn get_award(&self) -> &[AwardProperty];
	/// Take <https://schema.org/award> from [`Self`] as owned vector.
	fn take_award(&mut self) -> Vec<AwardProperty>;
	/// Get <https://schema.org/awards> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/award>."]
	fn get_awards(&self) -> &[AwardsProperty];
	/// Take <https://schema.org/awards> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/award>."]
	fn take_awards(&mut self) -> Vec<AwardsProperty>;
	/// Get <https://schema.org/character> from [`Self`] as borrowed slice.
	fn get_character(&self) -> &[CharacterProperty];
	/// Take <https://schema.org/character> from [`Self`] as owned vector.
	fn take_character(&mut self) -> Vec<CharacterProperty>;
	/// Get <https://schema.org/citation> from [`Self`] as borrowed slice.
	fn get_citation(&self) -> &[CitationProperty];
	/// Take <https://schema.org/citation> from [`Self`] as owned vector.
	fn take_citation(&mut self) -> Vec<CitationProperty>;
	/// Get <https://schema.org/comment> from [`Self`] as borrowed slice.
	fn get_comment(&self) -> &[CommentProperty];
	/// Take <https://schema.org/comment> from [`Self`] as owned vector.
	fn take_comment(&mut self) -> Vec<CommentProperty>;
	/// Get <https://schema.org/commentCount> from [`Self`] as borrowed slice.
	fn get_comment_count(&self) -> &[CommentCountProperty];
	/// Take <https://schema.org/commentCount> from [`Self`] as owned vector.
	fn take_comment_count(&mut self) -> Vec<CommentCountProperty>;
	/// Get <https://schema.org/conditionsOfAccess> from [`Self`] as borrowed slice.
	fn get_conditions_of_access(&self) -> &[ConditionsOfAccessProperty];
	/// Take <https://schema.org/conditionsOfAccess> from [`Self`] as owned vector.
	fn take_conditions_of_access(&mut self) -> Vec<ConditionsOfAccessProperty>;
	/// Get <https://schema.org/contentLocation> from [`Self`] as borrowed slice.
	fn get_content_location(&self) -> &[ContentLocationProperty];
	/// Take <https://schema.org/contentLocation> from [`Self`] as owned vector.
	fn take_content_location(&mut self) -> Vec<ContentLocationProperty>;
	/// Get <https://schema.org/contentRating> from [`Self`] as borrowed slice.
	fn get_content_rating(&self) -> &[ContentRatingProperty];
	/// Take <https://schema.org/contentRating> from [`Self`] as owned vector.
	fn take_content_rating(&mut self) -> Vec<ContentRatingProperty>;
	/// Get <https://schema.org/contentReferenceTime> from [`Self`] as borrowed slice.
	fn get_content_reference_time(&self) -> &[ContentReferenceTimeProperty];
	/// Take <https://schema.org/contentReferenceTime> from [`Self`] as owned vector.
	fn take_content_reference_time(&mut self) -> Vec<ContentReferenceTimeProperty>;
	/// Get <https://schema.org/contributor> from [`Self`] as borrowed slice.
	fn get_contributor(&self) -> &[ContributorProperty];
	/// Take <https://schema.org/contributor> from [`Self`] as owned vector.
	fn take_contributor(&mut self) -> Vec<ContributorProperty>;
	/// Get <https://schema.org/copyrightHolder> from [`Self`] as borrowed slice.
	fn get_copyright_holder(&self) -> &[CopyrightHolderProperty];
	/// Take <https://schema.org/copyrightHolder> from [`Self`] as owned vector.
	fn take_copyright_holder(&mut self) -> Vec<CopyrightHolderProperty>;
	/// Get <https://schema.org/copyrightNotice> from [`Self`] as borrowed slice.
	fn get_copyright_notice(&self) -> &[CopyrightNoticeProperty];
	/// Take <https://schema.org/copyrightNotice> from [`Self`] as owned vector.
	fn take_copyright_notice(&mut self) -> Vec<CopyrightNoticeProperty>;
	/// Get <https://schema.org/copyrightYear> from [`Self`] as borrowed slice.
	fn get_copyright_year(&self) -> &[CopyrightYearProperty];
	/// Take <https://schema.org/copyrightYear> from [`Self`] as owned vector.
	fn take_copyright_year(&mut self) -> Vec<CopyrightYearProperty>;
	/// Get <https://schema.org/correction> from [`Self`] as borrowed slice.
	fn get_correction(&self) -> &[CorrectionProperty];
	/// Take <https://schema.org/correction> from [`Self`] as owned vector.
	fn take_correction(&mut self) -> Vec<CorrectionProperty>;
	/// Get <https://schema.org/countryOfOrigin> from [`Self`] as borrowed slice.
	fn get_country_of_origin(&self) -> &[CountryOfOriginProperty];
	/// Take <https://schema.org/countryOfOrigin> from [`Self`] as owned vector.
	fn take_country_of_origin(&mut self) -> Vec<CountryOfOriginProperty>;
	/// Get <https://schema.org/creativeWorkStatus> from [`Self`] as borrowed slice.
	fn get_creative_work_status(&self) -> &[CreativeWorkStatusProperty];
	/// Take <https://schema.org/creativeWorkStatus> from [`Self`] as owned vector.
	fn take_creative_work_status(&mut self) -> Vec<CreativeWorkStatusProperty>;
	/// Get <https://schema.org/creator> from [`Self`] as borrowed slice.
	fn get_creator(&self) -> &[CreatorProperty];
	/// Take <https://schema.org/creator> from [`Self`] as owned vector.
	fn take_creator(&mut self) -> Vec<CreatorProperty>;
	/// Get <https://schema.org/creditText> from [`Self`] as borrowed slice.
	fn get_credit_text(&self) -> &[CreditTextProperty];
	/// Take <https://schema.org/creditText> from [`Self`] as owned vector.
	fn take_credit_text(&mut self) -> Vec<CreditTextProperty>;
	/// Get <https://schema.org/dateCreated> from [`Self`] as borrowed slice.
	fn get_date_created(&self) -> &[DateCreatedProperty];
	/// Take <https://schema.org/dateCreated> from [`Self`] as owned vector.
	fn take_date_created(&mut self) -> Vec<DateCreatedProperty>;
	/// Get <https://schema.org/dateModified> from [`Self`] as borrowed slice.
	fn get_date_modified(&self) -> &[DateModifiedProperty];
	/// Take <https://schema.org/dateModified> from [`Self`] as owned vector.
	fn take_date_modified(&mut self) -> Vec<DateModifiedProperty>;
	/// Get <https://schema.org/datePublished> from [`Self`] as borrowed slice.
	fn get_date_published(&self) -> &[DatePublishedProperty];
	/// Take <https://schema.org/datePublished> from [`Self`] as owned vector.
	fn take_date_published(&mut self) -> Vec<DatePublishedProperty>;
	/// Get <https://schema.org/digitalSourceType> from [`Self`] as borrowed slice.
	fn get_digital_source_type(&self) -> &[DigitalSourceTypeProperty];
	/// Take <https://schema.org/digitalSourceType> from [`Self`] as owned vector.
	fn take_digital_source_type(&mut self) -> Vec<DigitalSourceTypeProperty>;
	/// Get <https://schema.org/discussionUrl> from [`Self`] as borrowed slice.
	fn get_discussion_url(&self) -> &[DiscussionUrlProperty];
	/// Take <https://schema.org/discussionUrl> from [`Self`] as owned vector.
	fn take_discussion_url(&mut self) -> Vec<DiscussionUrlProperty>;
	/// Get <https://schema.org/displayLocation> from [`Self`] as borrowed slice.
	fn get_display_location(&self) -> &[DisplayLocationProperty];
	/// Take <https://schema.org/displayLocation> from [`Self`] as owned vector.
	fn take_display_location(&mut self) -> Vec<DisplayLocationProperty>;
	/// Get <https://schema.org/editEIDR> from [`Self`] as borrowed slice.
	fn get_edit_eidr(&self) -> &[EditEidrProperty];
	/// Take <https://schema.org/editEIDR> from [`Self`] as owned vector.
	fn take_edit_eidr(&mut self) -> Vec<EditEidrProperty>;
	/// Get <https://schema.org/editor> from [`Self`] as borrowed slice.
	fn get_editor(&self) -> &[EditorProperty];
	/// Take <https://schema.org/editor> from [`Self`] as owned vector.
	fn take_editor(&mut self) -> Vec<EditorProperty>;
	/// Get <https://schema.org/educationalAlignment> from [`Self`] as borrowed slice.
	fn get_educational_alignment(&self) -> &[EducationalAlignmentProperty];
	/// Take <https://schema.org/educationalAlignment> from [`Self`] as owned vector.
	fn take_educational_alignment(&mut self) -> Vec<EducationalAlignmentProperty>;
	/// Get <https://schema.org/educationalLevel> from [`Self`] as borrowed slice.
	fn get_educational_level(&self) -> &[EducationalLevelProperty];
	/// Take <https://schema.org/educationalLevel> from [`Self`] as owned vector.
	fn take_educational_level(&mut self) -> Vec<EducationalLevelProperty>;
	/// Get <https://schema.org/educationalUse> from [`Self`] as borrowed slice.
	fn get_educational_use(&self) -> &[EducationalUseProperty];
	/// Take <https://schema.org/educationalUse> from [`Self`] as owned vector.
	fn take_educational_use(&mut self) -> Vec<EducationalUseProperty>;
	/// Get <https://schema.org/encoding> from [`Self`] as borrowed slice.
	fn get_encoding(&self) -> &[EncodingProperty];
	/// Take <https://schema.org/encoding> from [`Self`] as owned vector.
	fn take_encoding(&mut self) -> Vec<EncodingProperty>;
	/// Get <https://schema.org/encodingFormat> from [`Self`] as borrowed slice.
	fn get_encoding_format(&self) -> &[EncodingFormatProperty];
	/// Take <https://schema.org/encodingFormat> from [`Self`] as owned vector.
	fn take_encoding_format(&mut self) -> Vec<EncodingFormatProperty>;
	/// Get <https://schema.org/encodings> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/encoding>."]
	fn get_encodings(&self) -> &[EncodingsProperty];
	/// Take <https://schema.org/encodings> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/encoding>."]
	fn take_encodings(&mut self) -> Vec<EncodingsProperty>;
	/// Get <https://schema.org/exampleOfWork> from [`Self`] as borrowed slice.
	fn get_example_of_work(&self) -> &[ExampleOfWorkProperty];
	/// Take <https://schema.org/exampleOfWork> from [`Self`] as owned vector.
	fn take_example_of_work(&mut self) -> Vec<ExampleOfWorkProperty>;
	/// Get <https://schema.org/expires> from [`Self`] as borrowed slice.
	fn get_expires(&self) -> &[ExpiresProperty];
	/// Take <https://schema.org/expires> from [`Self`] as owned vector.
	fn take_expires(&mut self) -> Vec<ExpiresProperty>;
	/// Get <https://schema.org/fileFormat> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/encodingFormat>."]
	fn get_file_format(&self) -> &[FileFormatProperty];
	/// Take <https://schema.org/fileFormat> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/encodingFormat>."]
	fn take_file_format(&mut self) -> Vec<FileFormatProperty>;
	/// Get <https://schema.org/funder> from [`Self`] as borrowed slice.
	fn get_funder(&self) -> &[FunderProperty];
	/// Take <https://schema.org/funder> from [`Self`] as owned vector.
	fn take_funder(&mut self) -> Vec<FunderProperty>;
	/// Get <https://schema.org/funding> from [`Self`] as borrowed slice.
	fn get_funding(&self) -> &[FundingProperty];
	/// Take <https://schema.org/funding> from [`Self`] as owned vector.
	fn take_funding(&mut self) -> Vec<FundingProperty>;
	/// Get <https://schema.org/genre> from [`Self`] as borrowed slice.
	fn get_genre(&self) -> &[GenreProperty];
	/// Take <https://schema.org/genre> from [`Self`] as owned vector.
	fn take_genre(&mut self) -> Vec<GenreProperty>;
	/// Get <https://schema.org/hasPart> from [`Self`] as borrowed slice.
	fn get_has_part(&self) -> &[HasPartProperty];
	/// Take <https://schema.org/hasPart> from [`Self`] as owned vector.
	fn take_has_part(&mut self) -> Vec<HasPartProperty>;
	/// Get <https://schema.org/headline> from [`Self`] as borrowed slice.
	fn get_headline(&self) -> &[HeadlineProperty];
	/// Take <https://schema.org/headline> from [`Self`] as owned vector.
	fn take_headline(&mut self) -> Vec<HeadlineProperty>;
	/// Get <https://schema.org/inLanguage> from [`Self`] as borrowed slice.
	fn get_in_language(&self) -> &[InLanguageProperty];
	/// Take <https://schema.org/inLanguage> from [`Self`] as owned vector.
	fn take_in_language(&mut self) -> Vec<InLanguageProperty>;
	/// Get <https://schema.org/interactionStatistic> from [`Self`] as borrowed slice.
	fn get_interaction_statistic(&self) -> &[InteractionStatisticProperty];
	/// Take <https://schema.org/interactionStatistic> from [`Self`] as owned vector.
	fn take_interaction_statistic(&mut self) -> Vec<InteractionStatisticProperty>;
	/// Get <https://schema.org/interactivityType> from [`Self`] as borrowed slice.
	fn get_interactivity_type(&self) -> &[InteractivityTypeProperty];
	/// Take <https://schema.org/interactivityType> from [`Self`] as owned vector.
	fn take_interactivity_type(&mut self) -> Vec<InteractivityTypeProperty>;
	/// Get <https://schema.org/interpretedAsClaim> from [`Self`] as borrowed slice.
	fn get_interpreted_as_claim(&self) -> &[InterpretedAsClaimProperty];
	/// Take <https://schema.org/interpretedAsClaim> from [`Self`] as owned vector.
	fn take_interpreted_as_claim(&mut self) -> Vec<InterpretedAsClaimProperty>;
	/// Get <https://schema.org/isAccessibleForFree> from [`Self`] as borrowed slice.
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty];
	/// Take <https://schema.org/isAccessibleForFree> from [`Self`] as owned vector.
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty>;
	/// Get <https://schema.org/isBasedOn> from [`Self`] as borrowed slice.
	fn get_is_based_on(&self) -> &[IsBasedOnProperty];
	/// Take <https://schema.org/isBasedOn> from [`Self`] as owned vector.
	fn take_is_based_on(&mut self) -> Vec<IsBasedOnProperty>;
	/// Get <https://schema.org/isBasedOnUrl> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
	fn get_is_based_on_url(&self) -> &[IsBasedOnUrlProperty];
	/// Take <https://schema.org/isBasedOnUrl> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/isBasedOn>."]
	fn take_is_based_on_url(&mut self) -> Vec<IsBasedOnUrlProperty>;
	/// Get <https://schema.org/isFamilyFriendly> from [`Self`] as borrowed slice.
	fn get_is_family_friendly(&self) -> &[IsFamilyFriendlyProperty];
	/// Take <https://schema.org/isFamilyFriendly> from [`Self`] as owned vector.
	fn take_is_family_friendly(&mut self) -> Vec<IsFamilyFriendlyProperty>;
	/// Get <https://schema.org/isPartOf> from [`Self`] as borrowed slice.
	fn get_is_part_of(&self) -> &[IsPartOfProperty];
	/// Take <https://schema.org/isPartOf> from [`Self`] as owned vector.
	fn take_is_part_of(&mut self) -> Vec<IsPartOfProperty>;
	/// Get <https://schema.org/keywords> from [`Self`] as borrowed slice.
	fn get_keywords(&self) -> &[KeywordsProperty];
	/// Take <https://schema.org/keywords> from [`Self`] as owned vector.
	fn take_keywords(&mut self) -> Vec<KeywordsProperty>;
	/// Get <https://schema.org/learningResourceType> from [`Self`] as borrowed slice.
	fn get_learning_resource_type(&self) -> &[LearningResourceTypeProperty];
	/// Take <https://schema.org/learningResourceType> from [`Self`] as owned vector.
	fn take_learning_resource_type(&mut self) -> Vec<LearningResourceTypeProperty>;
	/// Get <https://schema.org/license> from [`Self`] as borrowed slice.
	fn get_license(&self) -> &[LicenseProperty];
	/// Take <https://schema.org/license> from [`Self`] as owned vector.
	fn take_license(&mut self) -> Vec<LicenseProperty>;
	/// Get <https://schema.org/locationCreated> from [`Self`] as borrowed slice.
	fn get_location_created(&self) -> &[LocationCreatedProperty];
	/// Take <https://schema.org/locationCreated> from [`Self`] as owned vector.
	fn take_location_created(&mut self) -> Vec<LocationCreatedProperty>;
	/// Get <https://schema.org/mainEntity> from [`Self`] as borrowed slice.
	fn get_main_entity(&self) -> &[MainEntityProperty];
	/// Take <https://schema.org/mainEntity> from [`Self`] as owned vector.
	fn take_main_entity(&mut self) -> Vec<MainEntityProperty>;
	/// Get <https://schema.org/maintainer> from [`Self`] as borrowed slice.
	fn get_maintainer(&self) -> &[MaintainerProperty];
	/// Take <https://schema.org/maintainer> from [`Self`] as owned vector.
	fn take_maintainer(&mut self) -> Vec<MaintainerProperty>;
	/// Get <https://schema.org/material> from [`Self`] as borrowed slice.
	fn get_material(&self) -> &[MaterialProperty];
	/// Take <https://schema.org/material> from [`Self`] as owned vector.
	fn take_material(&mut self) -> Vec<MaterialProperty>;
	/// Get <https://schema.org/materialExtent> from [`Self`] as borrowed slice.
	fn get_material_extent(&self) -> &[MaterialExtentProperty];
	/// Take <https://schema.org/materialExtent> from [`Self`] as owned vector.
	fn take_material_extent(&mut self) -> Vec<MaterialExtentProperty>;
	/// Get <https://schema.org/mentions> from [`Self`] as borrowed slice.
	fn get_mentions(&self) -> &[MentionsProperty];
	/// Take <https://schema.org/mentions> from [`Self`] as owned vector.
	fn take_mentions(&mut self) -> Vec<MentionsProperty>;
	/// Get <https://schema.org/offers> from [`Self`] as borrowed slice.
	fn get_offers(&self) -> &[OffersProperty];
	/// Take <https://schema.org/offers> from [`Self`] as owned vector.
	fn take_offers(&mut self) -> Vec<OffersProperty>;
	/// Get <https://schema.org/pattern> from [`Self`] as borrowed slice.
	fn get_pattern(&self) -> &[PatternProperty];
	/// Take <https://schema.org/pattern> from [`Self`] as owned vector.
	fn take_pattern(&mut self) -> Vec<PatternProperty>;
	/// Get <https://schema.org/position> from [`Self`] as borrowed slice.
	fn get_position(&self) -> &[PositionProperty];
	/// Take <https://schema.org/position> from [`Self`] as owned vector.
	fn take_position(&mut self) -> Vec<PositionProperty>;
	/// Get <https://schema.org/producer> from [`Self`] as borrowed slice.
	fn get_producer(&self) -> &[ProducerProperty];
	/// Take <https://schema.org/producer> from [`Self`] as owned vector.
	fn take_producer(&mut self) -> Vec<ProducerProperty>;
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn get_provider(&self) -> &[ProviderProperty];
	/// Take <https://schema.org/provider> from [`Self`] as owned vector.
	fn take_provider(&mut self) -> Vec<ProviderProperty>;
	/// Get <https://schema.org/publication> from [`Self`] as borrowed slice.
	fn get_publication(&self) -> &[PublicationProperty];
	/// Take <https://schema.org/publication> from [`Self`] as owned vector.
	fn take_publication(&mut self) -> Vec<PublicationProperty>;
	/// Get <https://schema.org/publisher> from [`Self`] as borrowed slice.
	fn get_publisher(&self) -> &[PublisherProperty];
	/// Take <https://schema.org/publisher> from [`Self`] as owned vector.
	fn take_publisher(&mut self) -> Vec<PublisherProperty>;
	/// Get <https://schema.org/publisherImprint> from [`Self`] as borrowed slice.
	fn get_publisher_imprint(&self) -> &[PublisherImprintProperty];
	/// Take <https://schema.org/publisherImprint> from [`Self`] as owned vector.
	fn take_publisher_imprint(&mut self) -> Vec<PublisherImprintProperty>;
	/// Get <https://schema.org/publishingPrinciples> from [`Self`] as borrowed slice.
	fn get_publishing_principles(&self) -> &[PublishingPrinciplesProperty];
	/// Take <https://schema.org/publishingPrinciples> from [`Self`] as owned vector.
	fn take_publishing_principles(&mut self) -> Vec<PublishingPrinciplesProperty>;
	/// Get <https://schema.org/recordedAt> from [`Self`] as borrowed slice.
	fn get_recorded_at(&self) -> &[RecordedAtProperty];
	/// Take <https://schema.org/recordedAt> from [`Self`] as owned vector.
	fn take_recorded_at(&mut self) -> Vec<RecordedAtProperty>;
	/// Get <https://schema.org/releasedEvent> from [`Self`] as borrowed slice.
	fn get_released_event(&self) -> &[ReleasedEventProperty];
	/// Take <https://schema.org/releasedEvent> from [`Self`] as owned vector.
	fn take_released_event(&mut self) -> Vec<ReleasedEventProperty>;
	/// Get <https://schema.org/review> from [`Self`] as borrowed slice.
	fn get_review(&self) -> &[ReviewProperty];
	/// Take <https://schema.org/review> from [`Self`] as owned vector.
	fn take_review(&mut self) -> Vec<ReviewProperty>;
	/// Get <https://schema.org/reviews> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	fn get_reviews(&self) -> &[ReviewsProperty];
	/// Take <https://schema.org/reviews> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	fn take_reviews(&mut self) -> Vec<ReviewsProperty>;
	/// Get <https://schema.org/schemaVersion> from [`Self`] as borrowed slice.
	fn get_schema_version(&self) -> &[SchemaVersionProperty];
	/// Take <https://schema.org/schemaVersion> from [`Self`] as owned vector.
	fn take_schema_version(&mut self) -> Vec<SchemaVersionProperty>;
	/// Get <https://schema.org/sdDatePublished> from [`Self`] as borrowed slice.
	fn get_sd_date_published(&self) -> &[SdDatePublishedProperty];
	/// Take <https://schema.org/sdDatePublished> from [`Self`] as owned vector.
	fn take_sd_date_published(&mut self) -> Vec<SdDatePublishedProperty>;
	/// Get <https://schema.org/sdLicense> from [`Self`] as borrowed slice.
	fn get_sd_license(&self) -> &[SdLicenseProperty];
	/// Take <https://schema.org/sdLicense> from [`Self`] as owned vector.
	fn take_sd_license(&mut self) -> Vec<SdLicenseProperty>;
	/// Get <https://schema.org/sdPublisher> from [`Self`] as borrowed slice.
	fn get_sd_publisher(&self) -> &[SdPublisherProperty];
	/// Take <https://schema.org/sdPublisher> from [`Self`] as owned vector.
	fn take_sd_publisher(&mut self) -> Vec<SdPublisherProperty>;
	/// Get <https://schema.org/size> from [`Self`] as borrowed slice.
	fn get_size(&self) -> &[SizeProperty];
	/// Take <https://schema.org/size> from [`Self`] as owned vector.
	fn take_size(&mut self) -> Vec<SizeProperty>;
	/// Get <https://schema.org/sourceOrganization> from [`Self`] as borrowed slice.
	fn get_source_organization(&self) -> &[SourceOrganizationProperty];
	/// Take <https://schema.org/sourceOrganization> from [`Self`] as owned vector.
	fn take_source_organization(&mut self) -> Vec<SourceOrganizationProperty>;
	/// Get <https://schema.org/spatial> from [`Self`] as borrowed slice.
	fn get_spatial(&self) -> &[SpatialProperty];
	/// Take <https://schema.org/spatial> from [`Self`] as owned vector.
	fn take_spatial(&mut self) -> Vec<SpatialProperty>;
	/// Get <https://schema.org/spatialCoverage> from [`Self`] as borrowed slice.
	fn get_spatial_coverage(&self) -> &[SpatialCoverageProperty];
	/// Take <https://schema.org/spatialCoverage> from [`Self`] as owned vector.
	fn take_spatial_coverage(&mut self) -> Vec<SpatialCoverageProperty>;
	/// Get <https://schema.org/sponsor> from [`Self`] as borrowed slice.
	fn get_sponsor(&self) -> &[SponsorProperty];
	/// Take <https://schema.org/sponsor> from [`Self`] as owned vector.
	fn take_sponsor(&mut self) -> Vec<SponsorProperty>;
	/// Get <https://schema.org/teaches> from [`Self`] as borrowed slice.
	fn get_teaches(&self) -> &[TeachesProperty];
	/// Take <https://schema.org/teaches> from [`Self`] as owned vector.
	fn take_teaches(&mut self) -> Vec<TeachesProperty>;
	/// Get <https://schema.org/temporal> from [`Self`] as borrowed slice.
	fn get_temporal(&self) -> &[TemporalProperty];
	/// Take <https://schema.org/temporal> from [`Self`] as owned vector.
	fn take_temporal(&mut self) -> Vec<TemporalProperty>;
	/// Get <https://schema.org/temporalCoverage> from [`Self`] as borrowed slice.
	fn get_temporal_coverage(&self) -> &[TemporalCoverageProperty];
	/// Take <https://schema.org/temporalCoverage> from [`Self`] as owned vector.
	fn take_temporal_coverage(&mut self) -> Vec<TemporalCoverageProperty>;
	/// Get <https://schema.org/text> from [`Self`] as borrowed slice.
	fn get_text(&self) -> &[TextProperty];
	/// Take <https://schema.org/text> from [`Self`] as owned vector.
	fn take_text(&mut self) -> Vec<TextProperty>;
	/// Get <https://schema.org/thumbnail> from [`Self`] as borrowed slice.
	fn get_thumbnail(&self) -> &[ThumbnailProperty];
	/// Take <https://schema.org/thumbnail> from [`Self`] as owned vector.
	fn take_thumbnail(&mut self) -> Vec<ThumbnailProperty>;
	/// Get <https://schema.org/thumbnailUrl> from [`Self`] as borrowed slice.
	fn get_thumbnail_url(&self) -> &[ThumbnailUrlProperty];
	/// Take <https://schema.org/thumbnailUrl> from [`Self`] as owned vector.
	fn take_thumbnail_url(&mut self) -> Vec<ThumbnailUrlProperty>;
	/// Get <https://schema.org/timeRequired> from [`Self`] as borrowed slice.
	fn get_time_required(&self) -> &[TimeRequiredProperty];
	/// Take <https://schema.org/timeRequired> from [`Self`] as owned vector.
	fn take_time_required(&mut self) -> Vec<TimeRequiredProperty>;
	/// Get <https://schema.org/translationOfWork> from [`Self`] as borrowed slice.
	fn get_translation_of_work(&self) -> &[TranslationOfWorkProperty];
	/// Take <https://schema.org/translationOfWork> from [`Self`] as owned vector.
	fn take_translation_of_work(&mut self) -> Vec<TranslationOfWorkProperty>;
	/// Get <https://schema.org/translator> from [`Self`] as borrowed slice.
	fn get_translator(&self) -> &[TranslatorProperty];
	/// Take <https://schema.org/translator> from [`Self`] as owned vector.
	fn take_translator(&mut self) -> Vec<TranslatorProperty>;
	/// Get <https://schema.org/typicalAgeRange> from [`Self`] as borrowed slice.
	fn get_typical_age_range(&self) -> &[TypicalAgeRangeProperty];
	/// Take <https://schema.org/typicalAgeRange> from [`Self`] as owned vector.
	fn take_typical_age_range(&mut self) -> Vec<TypicalAgeRangeProperty>;
	/// Get <https://schema.org/usageInfo> from [`Self`] as borrowed slice.
	fn get_usage_info(&self) -> &[UsageInfoProperty];
	/// Take <https://schema.org/usageInfo> from [`Self`] as owned vector.
	fn take_usage_info(&mut self) -> Vec<UsageInfoProperty>;
	/// Get <https://schema.org/version> from [`Self`] as borrowed slice.
	fn get_version(&self) -> &[VersionProperty];
	/// Take <https://schema.org/version> from [`Self`] as owned vector.
	fn take_version(&mut self) -> Vec<VersionProperty>;
	/// Get <https://schema.org/video> from [`Self`] as borrowed slice.
	fn get_video(&self) -> &[VideoProperty];
	/// Take <https://schema.org/video> from [`Self`] as owned vector.
	fn take_video(&mut self) -> Vec<VideoProperty>;
	/// Get <https://schema.org/wordCount> from [`Self`] as borrowed slice.
	fn get_word_count(&self) -> &[WordCountProperty];
	/// Take <https://schema.org/wordCount> from [`Self`] as owned vector.
	fn take_word_count(&mut self) -> Vec<WordCountProperty>;
	/// Get <https://schema.org/workExample> from [`Self`] as borrowed slice.
	fn get_work_example(&self) -> &[WorkExampleProperty];
	/// Take <https://schema.org/workExample> from [`Self`] as owned vector.
	fn take_work_example(&mut self) -> Vec<WorkExampleProperty>;
	/// Get <https://schema.org/workTranslation> from [`Self`] as borrowed slice.
	fn get_work_translation(&self) -> &[WorkTranslationProperty];
	/// Take <https://schema.org/workTranslation> from [`Self`] as owned vector.
	fn take_work_translation(&mut self) -> Vec<WorkTranslationProperty>;
}
impl CreativeWorkTrait for CreativeWork {
	fn get_about(&self) -> &[AboutProperty] {
		self.r#about.as_slice()
	}
	fn take_about(&mut self) -> Vec<AboutProperty> {
		std::mem::take(&mut self.r#about)
	}
	fn get_abstract(&self) -> &[AbstractProperty] {
		self.r#abstract.as_slice()
	}
	fn take_abstract(&mut self) -> Vec<AbstractProperty> {
		std::mem::take(&mut self.r#abstract)
	}
	fn get_access_mode(&self) -> &[AccessModeProperty] {
		self.r#access_mode.as_slice()
	}
	fn take_access_mode(&mut self) -> Vec<AccessModeProperty> {
		std::mem::take(&mut self.r#access_mode)
	}
	fn get_access_mode_sufficient(&self) -> &[AccessModeSufficientProperty] {
		self.r#access_mode_sufficient.as_slice()
	}
	fn take_access_mode_sufficient(&mut self) -> Vec<AccessModeSufficientProperty> {
		std::mem::take(&mut self.r#access_mode_sufficient)
	}
	fn get_accessibility_api(&self) -> &[AccessibilityApiProperty] {
		self.r#accessibility_api.as_slice()
	}
	fn take_accessibility_api(&mut self) -> Vec<AccessibilityApiProperty> {
		std::mem::take(&mut self.r#accessibility_api)
	}
	fn get_accessibility_control(&self) -> &[AccessibilityControlProperty] {
		self.r#accessibility_control.as_slice()
	}
	fn take_accessibility_control(&mut self) -> Vec<AccessibilityControlProperty> {
		std::mem::take(&mut self.r#accessibility_control)
	}
	fn get_accessibility_feature(&self) -> &[AccessibilityFeatureProperty] {
		self.r#accessibility_feature.as_slice()
	}
	fn take_accessibility_feature(&mut self) -> Vec<AccessibilityFeatureProperty> {
		std::mem::take(&mut self.r#accessibility_feature)
	}
	fn get_accessibility_hazard(&self) -> &[AccessibilityHazardProperty] {
		self.r#accessibility_hazard.as_slice()
	}
	fn take_accessibility_hazard(&mut self) -> Vec<AccessibilityHazardProperty> {
		std::mem::take(&mut self.r#accessibility_hazard)
	}
	fn get_accessibility_summary(&self) -> &[AccessibilitySummaryProperty] {
		self.r#accessibility_summary.as_slice()
	}
	fn take_accessibility_summary(&mut self) -> Vec<AccessibilitySummaryProperty> {
		std::mem::take(&mut self.r#accessibility_summary)
	}
	fn get_accountable_person(&self) -> &[AccountablePersonProperty] {
		self.r#accountable_person.as_slice()
	}
	fn take_accountable_person(&mut self) -> Vec<AccountablePersonProperty> {
		std::mem::take(&mut self.r#accountable_person)
	}
	fn get_acquire_license_page(&self) -> &[AcquireLicensePageProperty] {
		self.r#acquire_license_page.as_slice()
	}
	fn take_acquire_license_page(&mut self) -> Vec<AcquireLicensePageProperty> {
		std::mem::take(&mut self.r#acquire_license_page)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_alternative_headline(&self) -> &[AlternativeHeadlineProperty] {
		self.r#alternative_headline.as_slice()
	}
	fn take_alternative_headline(&mut self) -> Vec<AlternativeHeadlineProperty> {
		std::mem::take(&mut self.r#alternative_headline)
	}
	fn get_archived_at(&self) -> &[ArchivedAtProperty] {
		self.r#archived_at.as_slice()
	}
	fn take_archived_at(&mut self) -> Vec<ArchivedAtProperty> {
		std::mem::take(&mut self.r#archived_at)
	}
	fn get_assesses(&self) -> &[AssessesProperty] {
		self.r#assesses.as_slice()
	}
	fn take_assesses(&mut self) -> Vec<AssessesProperty> {
		std::mem::take(&mut self.r#assesses)
	}
	fn get_associated_media(&self) -> &[AssociatedMediaProperty] {
		self.r#associated_media.as_slice()
	}
	fn take_associated_media(&mut self) -> Vec<AssociatedMediaProperty> {
		std::mem::take(&mut self.r#associated_media)
	}
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
	}
	fn get_audio(&self) -> &[AudioProperty] {
		self.r#audio.as_slice()
	}
	fn take_audio(&mut self) -> Vec<AudioProperty> {
		std::mem::take(&mut self.r#audio)
	}
	fn get_author(&self) -> &[AuthorProperty] {
		self.r#author.as_slice()
	}
	fn take_author(&mut self) -> Vec<AuthorProperty> {
		std::mem::take(&mut self.r#author)
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
	fn get_character(&self) -> &[CharacterProperty] {
		self.r#character.as_slice()
	}
	fn take_character(&mut self) -> Vec<CharacterProperty> {
		std::mem::take(&mut self.r#character)
	}
	fn get_citation(&self) -> &[CitationProperty] {
		self.r#citation.as_slice()
	}
	fn take_citation(&mut self) -> Vec<CitationProperty> {
		std::mem::take(&mut self.r#citation)
	}
	fn get_comment(&self) -> &[CommentProperty] {
		self.r#comment.as_slice()
	}
	fn take_comment(&mut self) -> Vec<CommentProperty> {
		std::mem::take(&mut self.r#comment)
	}
	fn get_comment_count(&self) -> &[CommentCountProperty] {
		self.r#comment_count.as_slice()
	}
	fn take_comment_count(&mut self) -> Vec<CommentCountProperty> {
		std::mem::take(&mut self.r#comment_count)
	}
	fn get_conditions_of_access(&self) -> &[ConditionsOfAccessProperty] {
		self.r#conditions_of_access.as_slice()
	}
	fn take_conditions_of_access(&mut self) -> Vec<ConditionsOfAccessProperty> {
		std::mem::take(&mut self.r#conditions_of_access)
	}
	fn get_content_location(&self) -> &[ContentLocationProperty] {
		self.r#content_location.as_slice()
	}
	fn take_content_location(&mut self) -> Vec<ContentLocationProperty> {
		std::mem::take(&mut self.r#content_location)
	}
	fn get_content_rating(&self) -> &[ContentRatingProperty] {
		self.r#content_rating.as_slice()
	}
	fn take_content_rating(&mut self) -> Vec<ContentRatingProperty> {
		std::mem::take(&mut self.r#content_rating)
	}
	fn get_content_reference_time(&self) -> &[ContentReferenceTimeProperty] {
		self.r#content_reference_time.as_slice()
	}
	fn take_content_reference_time(&mut self) -> Vec<ContentReferenceTimeProperty> {
		std::mem::take(&mut self.r#content_reference_time)
	}
	fn get_contributor(&self) -> &[ContributorProperty] {
		self.r#contributor.as_slice()
	}
	fn take_contributor(&mut self) -> Vec<ContributorProperty> {
		std::mem::take(&mut self.r#contributor)
	}
	fn get_copyright_holder(&self) -> &[CopyrightHolderProperty] {
		self.r#copyright_holder.as_slice()
	}
	fn take_copyright_holder(&mut self) -> Vec<CopyrightHolderProperty> {
		std::mem::take(&mut self.r#copyright_holder)
	}
	fn get_copyright_notice(&self) -> &[CopyrightNoticeProperty] {
		self.r#copyright_notice.as_slice()
	}
	fn take_copyright_notice(&mut self) -> Vec<CopyrightNoticeProperty> {
		std::mem::take(&mut self.r#copyright_notice)
	}
	fn get_copyright_year(&self) -> &[CopyrightYearProperty] {
		self.r#copyright_year.as_slice()
	}
	fn take_copyright_year(&mut self) -> Vec<CopyrightYearProperty> {
		std::mem::take(&mut self.r#copyright_year)
	}
	fn get_correction(&self) -> &[CorrectionProperty] {
		self.r#correction.as_slice()
	}
	fn take_correction(&mut self) -> Vec<CorrectionProperty> {
		std::mem::take(&mut self.r#correction)
	}
	fn get_country_of_origin(&self) -> &[CountryOfOriginProperty] {
		self.r#country_of_origin.as_slice()
	}
	fn take_country_of_origin(&mut self) -> Vec<CountryOfOriginProperty> {
		std::mem::take(&mut self.r#country_of_origin)
	}
	fn get_creative_work_status(&self) -> &[CreativeWorkStatusProperty] {
		self.r#creative_work_status.as_slice()
	}
	fn take_creative_work_status(&mut self) -> Vec<CreativeWorkStatusProperty> {
		std::mem::take(&mut self.r#creative_work_status)
	}
	fn get_creator(&self) -> &[CreatorProperty] {
		self.r#creator.as_slice()
	}
	fn take_creator(&mut self) -> Vec<CreatorProperty> {
		std::mem::take(&mut self.r#creator)
	}
	fn get_credit_text(&self) -> &[CreditTextProperty] {
		self.r#credit_text.as_slice()
	}
	fn take_credit_text(&mut self) -> Vec<CreditTextProperty> {
		std::mem::take(&mut self.r#credit_text)
	}
	fn get_date_created(&self) -> &[DateCreatedProperty] {
		self.r#date_created.as_slice()
	}
	fn take_date_created(&mut self) -> Vec<DateCreatedProperty> {
		std::mem::take(&mut self.r#date_created)
	}
	fn get_date_modified(&self) -> &[DateModifiedProperty] {
		self.r#date_modified.as_slice()
	}
	fn take_date_modified(&mut self) -> Vec<DateModifiedProperty> {
		std::mem::take(&mut self.r#date_modified)
	}
	fn get_date_published(&self) -> &[DatePublishedProperty] {
		self.r#date_published.as_slice()
	}
	fn take_date_published(&mut self) -> Vec<DatePublishedProperty> {
		std::mem::take(&mut self.r#date_published)
	}
	fn get_digital_source_type(&self) -> &[DigitalSourceTypeProperty] {
		self.r#digital_source_type.as_slice()
	}
	fn take_digital_source_type(&mut self) -> Vec<DigitalSourceTypeProperty> {
		std::mem::take(&mut self.r#digital_source_type)
	}
	fn get_discussion_url(&self) -> &[DiscussionUrlProperty] {
		self.r#discussion_url.as_slice()
	}
	fn take_discussion_url(&mut self) -> Vec<DiscussionUrlProperty> {
		std::mem::take(&mut self.r#discussion_url)
	}
	fn get_display_location(&self) -> &[DisplayLocationProperty] {
		self.r#display_location.as_slice()
	}
	fn take_display_location(&mut self) -> Vec<DisplayLocationProperty> {
		std::mem::take(&mut self.r#display_location)
	}
	fn get_edit_eidr(&self) -> &[EditEidrProperty] {
		self.r#edit_eidr.as_slice()
	}
	fn take_edit_eidr(&mut self) -> Vec<EditEidrProperty> {
		std::mem::take(&mut self.r#edit_eidr)
	}
	fn get_editor(&self) -> &[EditorProperty] {
		self.r#editor.as_slice()
	}
	fn take_editor(&mut self) -> Vec<EditorProperty> {
		std::mem::take(&mut self.r#editor)
	}
	fn get_educational_alignment(&self) -> &[EducationalAlignmentProperty] {
		self.r#educational_alignment.as_slice()
	}
	fn take_educational_alignment(&mut self) -> Vec<EducationalAlignmentProperty> {
		std::mem::take(&mut self.r#educational_alignment)
	}
	fn get_educational_level(&self) -> &[EducationalLevelProperty] {
		self.r#educational_level.as_slice()
	}
	fn take_educational_level(&mut self) -> Vec<EducationalLevelProperty> {
		std::mem::take(&mut self.r#educational_level)
	}
	fn get_educational_use(&self) -> &[EducationalUseProperty] {
		self.r#educational_use.as_slice()
	}
	fn take_educational_use(&mut self) -> Vec<EducationalUseProperty> {
		std::mem::take(&mut self.r#educational_use)
	}
	fn get_encoding(&self) -> &[EncodingProperty] {
		self.r#encoding.as_slice()
	}
	fn take_encoding(&mut self) -> Vec<EncodingProperty> {
		std::mem::take(&mut self.r#encoding)
	}
	fn get_encoding_format(&self) -> &[EncodingFormatProperty] {
		self.r#encoding_format.as_slice()
	}
	fn take_encoding_format(&mut self) -> Vec<EncodingFormatProperty> {
		std::mem::take(&mut self.r#encoding_format)
	}
	fn get_encodings(&self) -> &[EncodingsProperty] {
		self.r#encodings.as_slice()
	}
	fn take_encodings(&mut self) -> Vec<EncodingsProperty> {
		std::mem::take(&mut self.r#encodings)
	}
	fn get_example_of_work(&self) -> &[ExampleOfWorkProperty] {
		self.r#example_of_work.as_slice()
	}
	fn take_example_of_work(&mut self) -> Vec<ExampleOfWorkProperty> {
		std::mem::take(&mut self.r#example_of_work)
	}
	fn get_expires(&self) -> &[ExpiresProperty] {
		self.r#expires.as_slice()
	}
	fn take_expires(&mut self) -> Vec<ExpiresProperty> {
		std::mem::take(&mut self.r#expires)
	}
	fn get_file_format(&self) -> &[FileFormatProperty] {
		self.r#file_format.as_slice()
	}
	fn take_file_format(&mut self) -> Vec<FileFormatProperty> {
		std::mem::take(&mut self.r#file_format)
	}
	fn get_funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn take_funder(&mut self) -> Vec<FunderProperty> {
		std::mem::take(&mut self.r#funder)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_genre(&self) -> &[GenreProperty] {
		self.r#genre.as_slice()
	}
	fn take_genre(&mut self) -> Vec<GenreProperty> {
		std::mem::take(&mut self.r#genre)
	}
	fn get_has_part(&self) -> &[HasPartProperty] {
		self.r#has_part.as_slice()
	}
	fn take_has_part(&mut self) -> Vec<HasPartProperty> {
		std::mem::take(&mut self.r#has_part)
	}
	fn get_headline(&self) -> &[HeadlineProperty] {
		self.r#headline.as_slice()
	}
	fn take_headline(&mut self) -> Vec<HeadlineProperty> {
		std::mem::take(&mut self.r#headline)
	}
	fn get_in_language(&self) -> &[InLanguageProperty] {
		self.r#in_language.as_slice()
	}
	fn take_in_language(&mut self) -> Vec<InLanguageProperty> {
		std::mem::take(&mut self.r#in_language)
	}
	fn get_interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn take_interaction_statistic(&mut self) -> Vec<InteractionStatisticProperty> {
		std::mem::take(&mut self.r#interaction_statistic)
	}
	fn get_interactivity_type(&self) -> &[InteractivityTypeProperty] {
		self.r#interactivity_type.as_slice()
	}
	fn take_interactivity_type(&mut self) -> Vec<InteractivityTypeProperty> {
		std::mem::take(&mut self.r#interactivity_type)
	}
	fn get_interpreted_as_claim(&self) -> &[InterpretedAsClaimProperty] {
		self.r#interpreted_as_claim.as_slice()
	}
	fn take_interpreted_as_claim(&mut self) -> Vec<InterpretedAsClaimProperty> {
		std::mem::take(&mut self.r#interpreted_as_claim)
	}
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty> {
		std::mem::take(&mut self.r#is_accessible_for_free)
	}
	fn get_is_based_on(&self) -> &[IsBasedOnProperty] {
		self.r#is_based_on.as_slice()
	}
	fn take_is_based_on(&mut self) -> Vec<IsBasedOnProperty> {
		std::mem::take(&mut self.r#is_based_on)
	}
	fn get_is_based_on_url(&self) -> &[IsBasedOnUrlProperty] {
		self.r#is_based_on_url.as_slice()
	}
	fn take_is_based_on_url(&mut self) -> Vec<IsBasedOnUrlProperty> {
		std::mem::take(&mut self.r#is_based_on_url)
	}
	fn get_is_family_friendly(&self) -> &[IsFamilyFriendlyProperty] {
		self.r#is_family_friendly.as_slice()
	}
	fn take_is_family_friendly(&mut self) -> Vec<IsFamilyFriendlyProperty> {
		std::mem::take(&mut self.r#is_family_friendly)
	}
	fn get_is_part_of(&self) -> &[IsPartOfProperty] {
		self.r#is_part_of.as_slice()
	}
	fn take_is_part_of(&mut self) -> Vec<IsPartOfProperty> {
		std::mem::take(&mut self.r#is_part_of)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_learning_resource_type(&self) -> &[LearningResourceTypeProperty] {
		self.r#learning_resource_type.as_slice()
	}
	fn take_learning_resource_type(&mut self) -> Vec<LearningResourceTypeProperty> {
		std::mem::take(&mut self.r#learning_resource_type)
	}
	fn get_license(&self) -> &[LicenseProperty] {
		self.r#license.as_slice()
	}
	fn take_license(&mut self) -> Vec<LicenseProperty> {
		std::mem::take(&mut self.r#license)
	}
	fn get_location_created(&self) -> &[LocationCreatedProperty] {
		self.r#location_created.as_slice()
	}
	fn take_location_created(&mut self) -> Vec<LocationCreatedProperty> {
		std::mem::take(&mut self.r#location_created)
	}
	fn get_main_entity(&self) -> &[MainEntityProperty] {
		self.r#main_entity.as_slice()
	}
	fn take_main_entity(&mut self) -> Vec<MainEntityProperty> {
		std::mem::take(&mut self.r#main_entity)
	}
	fn get_maintainer(&self) -> &[MaintainerProperty] {
		self.r#maintainer.as_slice()
	}
	fn take_maintainer(&mut self) -> Vec<MaintainerProperty> {
		std::mem::take(&mut self.r#maintainer)
	}
	fn get_material(&self) -> &[MaterialProperty] {
		self.r#material.as_slice()
	}
	fn take_material(&mut self) -> Vec<MaterialProperty> {
		std::mem::take(&mut self.r#material)
	}
	fn get_material_extent(&self) -> &[MaterialExtentProperty] {
		self.r#material_extent.as_slice()
	}
	fn take_material_extent(&mut self) -> Vec<MaterialExtentProperty> {
		std::mem::take(&mut self.r#material_extent)
	}
	fn get_mentions(&self) -> &[MentionsProperty] {
		self.r#mentions.as_slice()
	}
	fn take_mentions(&mut self) -> Vec<MentionsProperty> {
		std::mem::take(&mut self.r#mentions)
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
	fn get_position(&self) -> &[PositionProperty] {
		self.r#position.as_slice()
	}
	fn take_position(&mut self) -> Vec<PositionProperty> {
		std::mem::take(&mut self.r#position)
	}
	fn get_producer(&self) -> &[ProducerProperty] {
		self.r#producer.as_slice()
	}
	fn take_producer(&mut self) -> Vec<ProducerProperty> {
		std::mem::take(&mut self.r#producer)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_publication(&self) -> &[PublicationProperty] {
		self.r#publication.as_slice()
	}
	fn take_publication(&mut self) -> Vec<PublicationProperty> {
		std::mem::take(&mut self.r#publication)
	}
	fn get_publisher(&self) -> &[PublisherProperty] {
		self.r#publisher.as_slice()
	}
	fn take_publisher(&mut self) -> Vec<PublisherProperty> {
		std::mem::take(&mut self.r#publisher)
	}
	fn get_publisher_imprint(&self) -> &[PublisherImprintProperty] {
		self.r#publisher_imprint.as_slice()
	}
	fn take_publisher_imprint(&mut self) -> Vec<PublisherImprintProperty> {
		std::mem::take(&mut self.r#publisher_imprint)
	}
	fn get_publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn take_publishing_principles(&mut self) -> Vec<PublishingPrinciplesProperty> {
		std::mem::take(&mut self.r#publishing_principles)
	}
	fn get_recorded_at(&self) -> &[RecordedAtProperty] {
		self.r#recorded_at.as_slice()
	}
	fn take_recorded_at(&mut self) -> Vec<RecordedAtProperty> {
		std::mem::take(&mut self.r#recorded_at)
	}
	fn get_released_event(&self) -> &[ReleasedEventProperty] {
		self.r#released_event.as_slice()
	}
	fn take_released_event(&mut self) -> Vec<ReleasedEventProperty> {
		std::mem::take(&mut self.r#released_event)
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
	fn get_schema_version(&self) -> &[SchemaVersionProperty] {
		self.r#schema_version.as_slice()
	}
	fn take_schema_version(&mut self) -> Vec<SchemaVersionProperty> {
		std::mem::take(&mut self.r#schema_version)
	}
	fn get_sd_date_published(&self) -> &[SdDatePublishedProperty] {
		self.r#sd_date_published.as_slice()
	}
	fn take_sd_date_published(&mut self) -> Vec<SdDatePublishedProperty> {
		std::mem::take(&mut self.r#sd_date_published)
	}
	fn get_sd_license(&self) -> &[SdLicenseProperty] {
		self.r#sd_license.as_slice()
	}
	fn take_sd_license(&mut self) -> Vec<SdLicenseProperty> {
		std::mem::take(&mut self.r#sd_license)
	}
	fn get_sd_publisher(&self) -> &[SdPublisherProperty] {
		self.r#sd_publisher.as_slice()
	}
	fn take_sd_publisher(&mut self) -> Vec<SdPublisherProperty> {
		std::mem::take(&mut self.r#sd_publisher)
	}
	fn get_size(&self) -> &[SizeProperty] {
		self.r#size.as_slice()
	}
	fn take_size(&mut self) -> Vec<SizeProperty> {
		std::mem::take(&mut self.r#size)
	}
	fn get_source_organization(&self) -> &[SourceOrganizationProperty] {
		self.r#source_organization.as_slice()
	}
	fn take_source_organization(&mut self) -> Vec<SourceOrganizationProperty> {
		std::mem::take(&mut self.r#source_organization)
	}
	fn get_spatial(&self) -> &[SpatialProperty] {
		self.r#spatial.as_slice()
	}
	fn take_spatial(&mut self) -> Vec<SpatialProperty> {
		std::mem::take(&mut self.r#spatial)
	}
	fn get_spatial_coverage(&self) -> &[SpatialCoverageProperty] {
		self.r#spatial_coverage.as_slice()
	}
	fn take_spatial_coverage(&mut self) -> Vec<SpatialCoverageProperty> {
		std::mem::take(&mut self.r#spatial_coverage)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_teaches(&self) -> &[TeachesProperty] {
		self.r#teaches.as_slice()
	}
	fn take_teaches(&mut self) -> Vec<TeachesProperty> {
		std::mem::take(&mut self.r#teaches)
	}
	fn get_temporal(&self) -> &[TemporalProperty] {
		self.r#temporal.as_slice()
	}
	fn take_temporal(&mut self) -> Vec<TemporalProperty> {
		std::mem::take(&mut self.r#temporal)
	}
	fn get_temporal_coverage(&self) -> &[TemporalCoverageProperty] {
		self.r#temporal_coverage.as_slice()
	}
	fn take_temporal_coverage(&mut self) -> Vec<TemporalCoverageProperty> {
		std::mem::take(&mut self.r#temporal_coverage)
	}
	fn get_text(&self) -> &[TextProperty] {
		self.r#text.as_slice()
	}
	fn take_text(&mut self) -> Vec<TextProperty> {
		std::mem::take(&mut self.r#text)
	}
	fn get_thumbnail(&self) -> &[ThumbnailProperty] {
		self.r#thumbnail.as_slice()
	}
	fn take_thumbnail(&mut self) -> Vec<ThumbnailProperty> {
		std::mem::take(&mut self.r#thumbnail)
	}
	fn get_thumbnail_url(&self) -> &[ThumbnailUrlProperty] {
		self.r#thumbnail_url.as_slice()
	}
	fn take_thumbnail_url(&mut self) -> Vec<ThumbnailUrlProperty> {
		std::mem::take(&mut self.r#thumbnail_url)
	}
	fn get_time_required(&self) -> &[TimeRequiredProperty] {
		self.r#time_required.as_slice()
	}
	fn take_time_required(&mut self) -> Vec<TimeRequiredProperty> {
		std::mem::take(&mut self.r#time_required)
	}
	fn get_translation_of_work(&self) -> &[TranslationOfWorkProperty] {
		self.r#translation_of_work.as_slice()
	}
	fn take_translation_of_work(&mut self) -> Vec<TranslationOfWorkProperty> {
		std::mem::take(&mut self.r#translation_of_work)
	}
	fn get_translator(&self) -> &[TranslatorProperty] {
		self.r#translator.as_slice()
	}
	fn take_translator(&mut self) -> Vec<TranslatorProperty> {
		std::mem::take(&mut self.r#translator)
	}
	fn get_typical_age_range(&self) -> &[TypicalAgeRangeProperty] {
		self.r#typical_age_range.as_slice()
	}
	fn take_typical_age_range(&mut self) -> Vec<TypicalAgeRangeProperty> {
		std::mem::take(&mut self.r#typical_age_range)
	}
	fn get_usage_info(&self) -> &[UsageInfoProperty] {
		self.r#usage_info.as_slice()
	}
	fn take_usage_info(&mut self) -> Vec<UsageInfoProperty> {
		std::mem::take(&mut self.r#usage_info)
	}
	fn get_version(&self) -> &[VersionProperty] {
		self.r#version.as_slice()
	}
	fn take_version(&mut self) -> Vec<VersionProperty> {
		std::mem::take(&mut self.r#version)
	}
	fn get_video(&self) -> &[VideoProperty] {
		self.r#video.as_slice()
	}
	fn take_video(&mut self) -> Vec<VideoProperty> {
		std::mem::take(&mut self.r#video)
	}
	fn get_word_count(&self) -> &[WordCountProperty] {
		self.r#word_count.as_slice()
	}
	fn take_word_count(&mut self) -> Vec<WordCountProperty> {
		std::mem::take(&mut self.r#word_count)
	}
	fn get_work_example(&self) -> &[WorkExampleProperty] {
		self.r#work_example.as_slice()
	}
	fn take_work_example(&mut self) -> Vec<WorkExampleProperty> {
		std::mem::take(&mut self.r#work_example)
	}
	fn get_work_translation(&self) -> &[WorkTranslationProperty] {
		self.r#work_translation.as_slice()
	}
	fn take_work_translation(&mut self) -> Vec<WorkTranslationProperty> {
		std::mem::take(&mut self.r#work_translation)
	}
}
impl ThingTrait for CreativeWork {
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
