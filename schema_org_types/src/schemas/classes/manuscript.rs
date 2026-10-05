use super::*;
/// <https://schema.org/Manuscript>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Manuscript {
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
/// This trait is for properties from <https://schema.org/Manuscript>.
pub trait ManuscriptTrait {}
impl ManuscriptTrait for Manuscript {}
impl CreativeWorkTrait for Manuscript {
	fn r#about(&self) -> &[AboutProperty] {
		self.r#about.as_slice()
	}
	fn r#abstract(&self) -> &[AbstractProperty] {
		self.r#abstract.as_slice()
	}
	fn r#access_mode(&self) -> &[AccessModeProperty] {
		self.r#access_mode.as_slice()
	}
	fn r#access_mode_sufficient(&self) -> &[AccessModeSufficientProperty] {
		self.r#access_mode_sufficient.as_slice()
	}
	fn r#accessibility_api(&self) -> &[AccessibilityApiProperty] {
		self.r#accessibility_api.as_slice()
	}
	fn r#accessibility_control(&self) -> &[AccessibilityControlProperty] {
		self.r#accessibility_control.as_slice()
	}
	fn r#accessibility_feature(&self) -> &[AccessibilityFeatureProperty] {
		self.r#accessibility_feature.as_slice()
	}
	fn r#accessibility_hazard(&self) -> &[AccessibilityHazardProperty] {
		self.r#accessibility_hazard.as_slice()
	}
	fn r#accessibility_summary(&self) -> &[AccessibilitySummaryProperty] {
		self.r#accessibility_summary.as_slice()
	}
	fn r#accountable_person(&self) -> &[AccountablePersonProperty] {
		self.r#accountable_person.as_slice()
	}
	fn r#acquire_license_page(&self) -> &[AcquireLicensePageProperty] {
		self.r#acquire_license_page.as_slice()
	}
	fn r#aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn r#alternative_headline(&self) -> &[AlternativeHeadlineProperty] {
		self.r#alternative_headline.as_slice()
	}
	fn r#archived_at(&self) -> &[ArchivedAtProperty] {
		self.r#archived_at.as_slice()
	}
	fn r#assesses(&self) -> &[AssessesProperty] {
		self.r#assesses.as_slice()
	}
	fn r#associated_media(&self) -> &[AssociatedMediaProperty] {
		self.r#associated_media.as_slice()
	}
	fn r#audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn r#audio(&self) -> &[AudioProperty] {
		self.r#audio.as_slice()
	}
	fn r#author(&self) -> &[AuthorProperty] {
		self.r#author.as_slice()
	}
	fn r#award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn r#awards(&self) -> &[AwardsProperty] {
		self.r#awards.as_slice()
	}
	fn r#character(&self) -> &[CharacterProperty] {
		self.r#character.as_slice()
	}
	fn r#citation(&self) -> &[CitationProperty] {
		self.r#citation.as_slice()
	}
	fn r#comment(&self) -> &[CommentProperty] {
		self.r#comment.as_slice()
	}
	fn r#comment_count(&self) -> &[CommentCountProperty] {
		self.r#comment_count.as_slice()
	}
	fn r#conditions_of_access(&self) -> &[ConditionsOfAccessProperty] {
		self.r#conditions_of_access.as_slice()
	}
	fn r#content_location(&self) -> &[ContentLocationProperty] {
		self.r#content_location.as_slice()
	}
	fn r#content_rating(&self) -> &[ContentRatingProperty] {
		self.r#content_rating.as_slice()
	}
	fn r#content_reference_time(&self) -> &[ContentReferenceTimeProperty] {
		self.r#content_reference_time.as_slice()
	}
	fn r#contributor(&self) -> &[ContributorProperty] {
		self.r#contributor.as_slice()
	}
	fn r#copyright_holder(&self) -> &[CopyrightHolderProperty] {
		self.r#copyright_holder.as_slice()
	}
	fn r#copyright_notice(&self) -> &[CopyrightNoticeProperty] {
		self.r#copyright_notice.as_slice()
	}
	fn r#copyright_year(&self) -> &[CopyrightYearProperty] {
		self.r#copyright_year.as_slice()
	}
	fn r#correction(&self) -> &[CorrectionProperty] {
		self.r#correction.as_slice()
	}
	fn r#country_of_origin(&self) -> &[CountryOfOriginProperty] {
		self.r#country_of_origin.as_slice()
	}
	fn r#creative_work_status(&self) -> &[CreativeWorkStatusProperty] {
		self.r#creative_work_status.as_slice()
	}
	fn r#creator(&self) -> &[CreatorProperty] {
		self.r#creator.as_slice()
	}
	fn r#credit_text(&self) -> &[CreditTextProperty] {
		self.r#credit_text.as_slice()
	}
	fn r#date_created(&self) -> &[DateCreatedProperty] {
		self.r#date_created.as_slice()
	}
	fn r#date_modified(&self) -> &[DateModifiedProperty] {
		self.r#date_modified.as_slice()
	}
	fn r#date_published(&self) -> &[DatePublishedProperty] {
		self.r#date_published.as_slice()
	}
	fn r#digital_source_type(&self) -> &[DigitalSourceTypeProperty] {
		self.r#digital_source_type.as_slice()
	}
	fn r#discussion_url(&self) -> &[DiscussionUrlProperty] {
		self.r#discussion_url.as_slice()
	}
	fn r#display_location(&self) -> &[DisplayLocationProperty] {
		self.r#display_location.as_slice()
	}
	fn r#edit_eidr(&self) -> &[EditEidrProperty] {
		self.r#edit_eidr.as_slice()
	}
	fn r#editor(&self) -> &[EditorProperty] {
		self.r#editor.as_slice()
	}
	fn r#educational_alignment(&self) -> &[EducationalAlignmentProperty] {
		self.r#educational_alignment.as_slice()
	}
	fn r#educational_level(&self) -> &[EducationalLevelProperty] {
		self.r#educational_level.as_slice()
	}
	fn r#educational_use(&self) -> &[EducationalUseProperty] {
		self.r#educational_use.as_slice()
	}
	fn r#encoding(&self) -> &[EncodingProperty] {
		self.r#encoding.as_slice()
	}
	fn r#encoding_format(&self) -> &[EncodingFormatProperty] {
		self.r#encoding_format.as_slice()
	}
	fn r#encodings(&self) -> &[EncodingsProperty] {
		self.r#encodings.as_slice()
	}
	fn r#example_of_work(&self) -> &[ExampleOfWorkProperty] {
		self.r#example_of_work.as_slice()
	}
	fn r#expires(&self) -> &[ExpiresProperty] {
		self.r#expires.as_slice()
	}
	fn r#file_format(&self) -> &[FileFormatProperty] {
		self.r#file_format.as_slice()
	}
	fn r#funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn r#funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn r#genre(&self) -> &[GenreProperty] {
		self.r#genre.as_slice()
	}
	fn r#has_part(&self) -> &[HasPartProperty] {
		self.r#has_part.as_slice()
	}
	fn r#headline(&self) -> &[HeadlineProperty] {
		self.r#headline.as_slice()
	}
	fn r#in_language(&self) -> &[InLanguageProperty] {
		self.r#in_language.as_slice()
	}
	fn r#interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn r#interactivity_type(&self) -> &[InteractivityTypeProperty] {
		self.r#interactivity_type.as_slice()
	}
	fn r#interpreted_as_claim(&self) -> &[InterpretedAsClaimProperty] {
		self.r#interpreted_as_claim.as_slice()
	}
	fn r#is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn r#is_based_on(&self) -> &[IsBasedOnProperty] {
		self.r#is_based_on.as_slice()
	}
	fn r#is_based_on_url(&self) -> &[IsBasedOnUrlProperty] {
		self.r#is_based_on_url.as_slice()
	}
	fn r#is_family_friendly(&self) -> &[IsFamilyFriendlyProperty] {
		self.r#is_family_friendly.as_slice()
	}
	fn r#is_part_of(&self) -> &[IsPartOfProperty] {
		self.r#is_part_of.as_slice()
	}
	fn r#keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn r#learning_resource_type(&self) -> &[LearningResourceTypeProperty] {
		self.r#learning_resource_type.as_slice()
	}
	fn r#license(&self) -> &[LicenseProperty] {
		self.r#license.as_slice()
	}
	fn r#location_created(&self) -> &[LocationCreatedProperty] {
		self.r#location_created.as_slice()
	}
	fn r#main_entity(&self) -> &[MainEntityProperty] {
		self.r#main_entity.as_slice()
	}
	fn r#maintainer(&self) -> &[MaintainerProperty] {
		self.r#maintainer.as_slice()
	}
	fn r#material(&self) -> &[MaterialProperty] {
		self.r#material.as_slice()
	}
	fn r#material_extent(&self) -> &[MaterialExtentProperty] {
		self.r#material_extent.as_slice()
	}
	fn r#mentions(&self) -> &[MentionsProperty] {
		self.r#mentions.as_slice()
	}
	fn r#offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn r#pattern(&self) -> &[PatternProperty] {
		self.r#pattern.as_slice()
	}
	fn r#position(&self) -> &[PositionProperty] {
		self.r#position.as_slice()
	}
	fn r#producer(&self) -> &[ProducerProperty] {
		self.r#producer.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#publication(&self) -> &[PublicationProperty] {
		self.r#publication.as_slice()
	}
	fn r#publisher(&self) -> &[PublisherProperty] {
		self.r#publisher.as_slice()
	}
	fn r#publisher_imprint(&self) -> &[PublisherImprintProperty] {
		self.r#publisher_imprint.as_slice()
	}
	fn r#publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn r#recorded_at(&self) -> &[RecordedAtProperty] {
		self.r#recorded_at.as_slice()
	}
	fn r#released_event(&self) -> &[ReleasedEventProperty] {
		self.r#released_event.as_slice()
	}
	fn r#review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn r#reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn r#schema_version(&self) -> &[SchemaVersionProperty] {
		self.r#schema_version.as_slice()
	}
	fn r#sd_date_published(&self) -> &[SdDatePublishedProperty] {
		self.r#sd_date_published.as_slice()
	}
	fn r#sd_license(&self) -> &[SdLicenseProperty] {
		self.r#sd_license.as_slice()
	}
	fn r#sd_publisher(&self) -> &[SdPublisherProperty] {
		self.r#sd_publisher.as_slice()
	}
	fn r#size(&self) -> &[SizeProperty] {
		self.r#size.as_slice()
	}
	fn r#source_organization(&self) -> &[SourceOrganizationProperty] {
		self.r#source_organization.as_slice()
	}
	fn r#spatial(&self) -> &[SpatialProperty] {
		self.r#spatial.as_slice()
	}
	fn r#spatial_coverage(&self) -> &[SpatialCoverageProperty] {
		self.r#spatial_coverage.as_slice()
	}
	fn r#sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn r#teaches(&self) -> &[TeachesProperty] {
		self.r#teaches.as_slice()
	}
	fn r#temporal(&self) -> &[TemporalProperty] {
		self.r#temporal.as_slice()
	}
	fn r#temporal_coverage(&self) -> &[TemporalCoverageProperty] {
		self.r#temporal_coverage.as_slice()
	}
	fn r#text(&self) -> &[TextProperty] {
		self.r#text.as_slice()
	}
	fn r#thumbnail(&self) -> &[ThumbnailProperty] {
		self.r#thumbnail.as_slice()
	}
	fn r#thumbnail_url(&self) -> &[ThumbnailUrlProperty] {
		self.r#thumbnail_url.as_slice()
	}
	fn r#time_required(&self) -> &[TimeRequiredProperty] {
		self.r#time_required.as_slice()
	}
	fn r#translation_of_work(&self) -> &[TranslationOfWorkProperty] {
		self.r#translation_of_work.as_slice()
	}
	fn r#translator(&self) -> &[TranslatorProperty] {
		self.r#translator.as_slice()
	}
	fn r#typical_age_range(&self) -> &[TypicalAgeRangeProperty] {
		self.r#typical_age_range.as_slice()
	}
	fn r#usage_info(&self) -> &[UsageInfoProperty] {
		self.r#usage_info.as_slice()
	}
	fn r#version(&self) -> &[VersionProperty] {
		self.r#version.as_slice()
	}
	fn r#video(&self) -> &[VideoProperty] {
		self.r#video.as_slice()
	}
	fn r#word_count(&self) -> &[WordCountProperty] {
		self.r#word_count.as_slice()
	}
	fn r#work_example(&self) -> &[WorkExampleProperty] {
		self.r#work_example.as_slice()
	}
	fn r#work_translation(&self) -> &[WorkTranslationProperty] {
		self.r#work_translation.as_slice()
	}
}
impl ThingTrait for Manuscript {
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
