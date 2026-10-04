use super::*;
/// <https://schema.org/WebPage>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct WebPage {
	/// <https://schema.org/breadcrumb>
	#[cfg_attr(feature = "serde", serde(rename = "breadcrumb"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#breadcrumb: Vec<BreadcrumbProperty>,
	/// <https://schema.org/lastReviewed>
	#[cfg_attr(feature = "serde", serde(rename = "lastReviewed"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#last_reviewed: Vec<LastReviewedProperty>,
	/// <https://schema.org/mainContentOfPage>
	#[cfg_attr(feature = "serde", serde(rename = "mainContentOfPage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#main_content_of_page: Vec<MainContentOfPageProperty>,
	/// <https://schema.org/primaryImageOfPage>
	#[cfg_attr(feature = "serde", serde(rename = "primaryImageOfPage"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#primary_image_of_page: Vec<PrimaryImageOfPageProperty>,
	/// <https://schema.org/relatedLink>
	#[cfg_attr(feature = "serde", serde(rename = "relatedLink"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#related_link: Vec<RelatedLinkProperty>,
	/// <https://schema.org/reviewedBy>
	#[cfg_attr(feature = "serde", serde(rename = "reviewedBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#reviewed_by: Vec<ReviewedByProperty>,
	/// <https://schema.org/significantLink>
	#[cfg_attr(feature = "serde", serde(rename = "significantLink"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#significant_link: Vec<SignificantLinkProperty>,
	/// <https://schema.org/significantLinks>
	#[deprecated = "This schema is superseded by <https://schema.org/significantLink>."]
	#[cfg_attr(feature = "serde", serde(rename = "significantLinks"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#significant_links: Vec<SignificantLinksProperty>,
	/// <https://schema.org/speakable>
	#[cfg_attr(feature = "serde", serde(rename = "speakable"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#speakable: Vec<SpeakableProperty>,
	/// <https://schema.org/specialty>
	#[cfg_attr(feature = "serde", serde(rename = "specialty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#specialty: Vec<SpecialtyProperty>,
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
/// This trait is for properties from <https://schema.org/WebPage>.
pub trait WebPageTrait {
	/// Get <https://schema.org/breadcrumb> from [`Self`] as borrowed slice.
	fn get_breadcrumb(&self) -> &[BreadcrumbProperty];
	/// Take <https://schema.org/breadcrumb> from [`Self`] as owned vector.
	fn take_breadcrumb(&mut self) -> Vec<BreadcrumbProperty>;
	/// Get <https://schema.org/lastReviewed> from [`Self`] as borrowed slice.
	fn get_last_reviewed(&self) -> &[LastReviewedProperty];
	/// Take <https://schema.org/lastReviewed> from [`Self`] as owned vector.
	fn take_last_reviewed(&mut self) -> Vec<LastReviewedProperty>;
	/// Get <https://schema.org/mainContentOfPage> from [`Self`] as borrowed slice.
	fn get_main_content_of_page(&self) -> &[MainContentOfPageProperty];
	/// Take <https://schema.org/mainContentOfPage> from [`Self`] as owned vector.
	fn take_main_content_of_page(&mut self) -> Vec<MainContentOfPageProperty>;
	/// Get <https://schema.org/primaryImageOfPage> from [`Self`] as borrowed slice.
	fn get_primary_image_of_page(&self) -> &[PrimaryImageOfPageProperty];
	/// Take <https://schema.org/primaryImageOfPage> from [`Self`] as owned vector.
	fn take_primary_image_of_page(&mut self) -> Vec<PrimaryImageOfPageProperty>;
	/// Get <https://schema.org/relatedLink> from [`Self`] as borrowed slice.
	fn get_related_link(&self) -> &[RelatedLinkProperty];
	/// Take <https://schema.org/relatedLink> from [`Self`] as owned vector.
	fn take_related_link(&mut self) -> Vec<RelatedLinkProperty>;
	/// Get <https://schema.org/reviewedBy> from [`Self`] as borrowed slice.
	fn get_reviewed_by(&self) -> &[ReviewedByProperty];
	/// Take <https://schema.org/reviewedBy> from [`Self`] as owned vector.
	fn take_reviewed_by(&mut self) -> Vec<ReviewedByProperty>;
	/// Get <https://schema.org/significantLink> from [`Self`] as borrowed slice.
	fn get_significant_link(&self) -> &[SignificantLinkProperty];
	/// Take <https://schema.org/significantLink> from [`Self`] as owned vector.
	fn take_significant_link(&mut self) -> Vec<SignificantLinkProperty>;
	/// Get <https://schema.org/significantLinks> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/significantLink>."]
	fn get_significant_links(&self) -> &[SignificantLinksProperty];
	/// Take <https://schema.org/significantLinks> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/significantLink>."]
	fn take_significant_links(&mut self) -> Vec<SignificantLinksProperty>;
	/// Get <https://schema.org/speakable> from [`Self`] as borrowed slice.
	fn get_speakable(&self) -> &[SpeakableProperty];
	/// Take <https://schema.org/speakable> from [`Self`] as owned vector.
	fn take_speakable(&mut self) -> Vec<SpeakableProperty>;
	/// Get <https://schema.org/specialty> from [`Self`] as borrowed slice.
	fn get_specialty(&self) -> &[SpecialtyProperty];
	/// Take <https://schema.org/specialty> from [`Self`] as owned vector.
	fn take_specialty(&mut self) -> Vec<SpecialtyProperty>;
}
impl WebPageTrait for WebPage {
	fn get_breadcrumb(&self) -> &[BreadcrumbProperty] {
		self.r#breadcrumb.as_slice()
	}
	fn take_breadcrumb(&mut self) -> Vec<BreadcrumbProperty> {
		std::mem::take(&mut self.r#breadcrumb)
	}
	fn get_last_reviewed(&self) -> &[LastReviewedProperty] {
		self.r#last_reviewed.as_slice()
	}
	fn take_last_reviewed(&mut self) -> Vec<LastReviewedProperty> {
		std::mem::take(&mut self.r#last_reviewed)
	}
	fn get_main_content_of_page(&self) -> &[MainContentOfPageProperty] {
		self.r#main_content_of_page.as_slice()
	}
	fn take_main_content_of_page(&mut self) -> Vec<MainContentOfPageProperty> {
		std::mem::take(&mut self.r#main_content_of_page)
	}
	fn get_primary_image_of_page(&self) -> &[PrimaryImageOfPageProperty] {
		self.r#primary_image_of_page.as_slice()
	}
	fn take_primary_image_of_page(&mut self) -> Vec<PrimaryImageOfPageProperty> {
		std::mem::take(&mut self.r#primary_image_of_page)
	}
	fn get_related_link(&self) -> &[RelatedLinkProperty] {
		self.r#related_link.as_slice()
	}
	fn take_related_link(&mut self) -> Vec<RelatedLinkProperty> {
		std::mem::take(&mut self.r#related_link)
	}
	fn get_reviewed_by(&self) -> &[ReviewedByProperty] {
		self.r#reviewed_by.as_slice()
	}
	fn take_reviewed_by(&mut self) -> Vec<ReviewedByProperty> {
		std::mem::take(&mut self.r#reviewed_by)
	}
	fn get_significant_link(&self) -> &[SignificantLinkProperty] {
		self.r#significant_link.as_slice()
	}
	fn take_significant_link(&mut self) -> Vec<SignificantLinkProperty> {
		std::mem::take(&mut self.r#significant_link)
	}
	fn get_significant_links(&self) -> &[SignificantLinksProperty] {
		self.r#significant_links.as_slice()
	}
	fn take_significant_links(&mut self) -> Vec<SignificantLinksProperty> {
		std::mem::take(&mut self.r#significant_links)
	}
	fn get_speakable(&self) -> &[SpeakableProperty] {
		self.r#speakable.as_slice()
	}
	fn take_speakable(&mut self) -> Vec<SpeakableProperty> {
		std::mem::take(&mut self.r#speakable)
	}
	fn get_specialty(&self) -> &[SpecialtyProperty] {
		self.r#specialty.as_slice()
	}
	fn take_specialty(&mut self) -> Vec<SpecialtyProperty> {
		std::mem::take(&mut self.r#specialty)
	}
}
impl CreativeWorkTrait for WebPage {
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
impl ThingTrait for WebPage {
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
