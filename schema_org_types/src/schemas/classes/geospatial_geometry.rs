use super::*;
/// <https://schema.org/GeospatialGeometry>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct GeospatialGeometry {
	/// <https://schema.org/geoContains>
	#[cfg_attr(feature = "serde", serde(rename = "geoContains"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_contains: Vec<GeoContainsProperty>,
	/// <https://schema.org/geoCoveredBy>
	#[cfg_attr(feature = "serde", serde(rename = "geoCoveredBy"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_covered_by: Vec<GeoCoveredByProperty>,
	/// <https://schema.org/geoCovers>
	#[cfg_attr(feature = "serde", serde(rename = "geoCovers"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_covers: Vec<GeoCoversProperty>,
	/// <https://schema.org/geoCrosses>
	#[cfg_attr(feature = "serde", serde(rename = "geoCrosses"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_crosses: Vec<GeoCrossesProperty>,
	/// <https://schema.org/geoDisjoint>
	#[cfg_attr(feature = "serde", serde(rename = "geoDisjoint"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_disjoint: Vec<GeoDisjointProperty>,
	/// <https://schema.org/geoEquals>
	#[cfg_attr(feature = "serde", serde(rename = "geoEquals"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_equals: Vec<GeoEqualsProperty>,
	/// <https://schema.org/geoIntersects>
	#[cfg_attr(feature = "serde", serde(rename = "geoIntersects"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_intersects: Vec<GeoIntersectsProperty>,
	/// <https://schema.org/geoOverlaps>
	#[cfg_attr(feature = "serde", serde(rename = "geoOverlaps"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_overlaps: Vec<GeoOverlapsProperty>,
	/// <https://schema.org/geoTouches>
	#[cfg_attr(feature = "serde", serde(rename = "geoTouches"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_touches: Vec<GeoTouchesProperty>,
	/// <https://schema.org/geoWithin>
	#[cfg_attr(feature = "serde", serde(rename = "geoWithin"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#geo_within: Vec<GeoWithinProperty>,
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
/// This trait is for properties from <https://schema.org/GeospatialGeometry>.
pub trait GeospatialGeometryTrait {
	/// Get <https://schema.org/geoContains> from [`Self`] as borrowed slice.
	fn get_geo_contains(&self) -> &[GeoContainsProperty];
	/// Take <https://schema.org/geoContains> from [`Self`] as owned vector.
	fn take_geo_contains(&mut self) -> Vec<GeoContainsProperty>;
	/// Get <https://schema.org/geoCoveredBy> from [`Self`] as borrowed slice.
	fn get_geo_covered_by(&self) -> &[GeoCoveredByProperty];
	/// Take <https://schema.org/geoCoveredBy> from [`Self`] as owned vector.
	fn take_geo_covered_by(&mut self) -> Vec<GeoCoveredByProperty>;
	/// Get <https://schema.org/geoCovers> from [`Self`] as borrowed slice.
	fn get_geo_covers(&self) -> &[GeoCoversProperty];
	/// Take <https://schema.org/geoCovers> from [`Self`] as owned vector.
	fn take_geo_covers(&mut self) -> Vec<GeoCoversProperty>;
	/// Get <https://schema.org/geoCrosses> from [`Self`] as borrowed slice.
	fn get_geo_crosses(&self) -> &[GeoCrossesProperty];
	/// Take <https://schema.org/geoCrosses> from [`Self`] as owned vector.
	fn take_geo_crosses(&mut self) -> Vec<GeoCrossesProperty>;
	/// Get <https://schema.org/geoDisjoint> from [`Self`] as borrowed slice.
	fn get_geo_disjoint(&self) -> &[GeoDisjointProperty];
	/// Take <https://schema.org/geoDisjoint> from [`Self`] as owned vector.
	fn take_geo_disjoint(&mut self) -> Vec<GeoDisjointProperty>;
	/// Get <https://schema.org/geoEquals> from [`Self`] as borrowed slice.
	fn get_geo_equals(&self) -> &[GeoEqualsProperty];
	/// Take <https://schema.org/geoEquals> from [`Self`] as owned vector.
	fn take_geo_equals(&mut self) -> Vec<GeoEqualsProperty>;
	/// Get <https://schema.org/geoIntersects> from [`Self`] as borrowed slice.
	fn get_geo_intersects(&self) -> &[GeoIntersectsProperty];
	/// Take <https://schema.org/geoIntersects> from [`Self`] as owned vector.
	fn take_geo_intersects(&mut self) -> Vec<GeoIntersectsProperty>;
	/// Get <https://schema.org/geoOverlaps> from [`Self`] as borrowed slice.
	fn get_geo_overlaps(&self) -> &[GeoOverlapsProperty];
	/// Take <https://schema.org/geoOverlaps> from [`Self`] as owned vector.
	fn take_geo_overlaps(&mut self) -> Vec<GeoOverlapsProperty>;
	/// Get <https://schema.org/geoTouches> from [`Self`] as borrowed slice.
	fn get_geo_touches(&self) -> &[GeoTouchesProperty];
	/// Take <https://schema.org/geoTouches> from [`Self`] as owned vector.
	fn take_geo_touches(&mut self) -> Vec<GeoTouchesProperty>;
	/// Get <https://schema.org/geoWithin> from [`Self`] as borrowed slice.
	fn get_geo_within(&self) -> &[GeoWithinProperty];
	/// Take <https://schema.org/geoWithin> from [`Self`] as owned vector.
	fn take_geo_within(&mut self) -> Vec<GeoWithinProperty>;
}
impl GeospatialGeometryTrait for GeospatialGeometry {
	fn get_geo_contains(&self) -> &[GeoContainsProperty] {
		self.r#geo_contains.as_slice()
	}
	fn take_geo_contains(&mut self) -> Vec<GeoContainsProperty> {
		std::mem::take(&mut self.r#geo_contains)
	}
	fn get_geo_covered_by(&self) -> &[GeoCoveredByProperty] {
		self.r#geo_covered_by.as_slice()
	}
	fn take_geo_covered_by(&mut self) -> Vec<GeoCoveredByProperty> {
		std::mem::take(&mut self.r#geo_covered_by)
	}
	fn get_geo_covers(&self) -> &[GeoCoversProperty] {
		self.r#geo_covers.as_slice()
	}
	fn take_geo_covers(&mut self) -> Vec<GeoCoversProperty> {
		std::mem::take(&mut self.r#geo_covers)
	}
	fn get_geo_crosses(&self) -> &[GeoCrossesProperty] {
		self.r#geo_crosses.as_slice()
	}
	fn take_geo_crosses(&mut self) -> Vec<GeoCrossesProperty> {
		std::mem::take(&mut self.r#geo_crosses)
	}
	fn get_geo_disjoint(&self) -> &[GeoDisjointProperty] {
		self.r#geo_disjoint.as_slice()
	}
	fn take_geo_disjoint(&mut self) -> Vec<GeoDisjointProperty> {
		std::mem::take(&mut self.r#geo_disjoint)
	}
	fn get_geo_equals(&self) -> &[GeoEqualsProperty] {
		self.r#geo_equals.as_slice()
	}
	fn take_geo_equals(&mut self) -> Vec<GeoEqualsProperty> {
		std::mem::take(&mut self.r#geo_equals)
	}
	fn get_geo_intersects(&self) -> &[GeoIntersectsProperty] {
		self.r#geo_intersects.as_slice()
	}
	fn take_geo_intersects(&mut self) -> Vec<GeoIntersectsProperty> {
		std::mem::take(&mut self.r#geo_intersects)
	}
	fn get_geo_overlaps(&self) -> &[GeoOverlapsProperty] {
		self.r#geo_overlaps.as_slice()
	}
	fn take_geo_overlaps(&mut self) -> Vec<GeoOverlapsProperty> {
		std::mem::take(&mut self.r#geo_overlaps)
	}
	fn get_geo_touches(&self) -> &[GeoTouchesProperty] {
		self.r#geo_touches.as_slice()
	}
	fn take_geo_touches(&mut self) -> Vec<GeoTouchesProperty> {
		std::mem::take(&mut self.r#geo_touches)
	}
	fn get_geo_within(&self) -> &[GeoWithinProperty] {
		self.r#geo_within.as_slice()
	}
	fn take_geo_within(&mut self) -> Vec<GeoWithinProperty> {
		std::mem::take(&mut self.r#geo_within)
	}
}
impl ThingTrait for GeospatialGeometry {
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
