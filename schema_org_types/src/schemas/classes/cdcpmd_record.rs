use super::*;
/// <https://schema.org/CDCPMDRecord>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct CdcpmdRecord {
	/// <https://schema.org/cvdCollectionDate>
	#[cfg_attr(feature = "serde", serde(rename = "cvdCollectionDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_collection_date: Vec<CvdCollectionDateProperty>,
	/// <https://schema.org/cvdFacilityCounty>
	#[cfg_attr(feature = "serde", serde(rename = "cvdFacilityCounty"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_facility_county: Vec<CvdFacilityCountyProperty>,
	/// <https://schema.org/cvdFacilityId>
	#[cfg_attr(feature = "serde", serde(rename = "cvdFacilityId"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_facility_id: Vec<CvdFacilityIdProperty>,
	/// <https://schema.org/cvdNumBeds>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumBeds"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_beds: Vec<CvdNumBedsProperty>,
	/// <https://schema.org/cvdNumBedsOcc>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumBedsOcc"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_beds_occ: Vec<CvdNumBedsOccProperty>,
	/// <https://schema.org/cvdNumC19Died>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumC19Died"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_c_19_died: Vec<CvdNumC19DiedProperty>,
	/// <https://schema.org/cvdNumC19HOPats>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumC19HOPats"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_c_19_ho_pats: Vec<CvdNumC19HoPatsProperty>,
	/// <https://schema.org/cvdNumC19HospPats>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumC19HospPats"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_c_19_hosp_pats: Vec<CvdNumC19HospPatsProperty>,
	/// <https://schema.org/cvdNumC19MechVentPats>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumC19MechVentPats"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_c_19_mech_vent_pats: Vec<CvdNumC19MechVentPatsProperty>,
	/// <https://schema.org/cvdNumC19OFMechVentPats>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumC19OFMechVentPats"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_c_19_of_mech_vent_pats: Vec<CvdNumC19OfMechVentPatsProperty>,
	/// <https://schema.org/cvdNumC19OverflowPats>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumC19OverflowPats"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_c_19_overflow_pats: Vec<CvdNumC19OverflowPatsProperty>,
	/// <https://schema.org/cvdNumICUBeds>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumICUBeds"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_icu_beds: Vec<CvdNumIcuBedsProperty>,
	/// <https://schema.org/cvdNumICUBedsOcc>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumICUBedsOcc"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_icu_beds_occ: Vec<CvdNumIcuBedsOccProperty>,
	/// <https://schema.org/cvdNumTotBeds>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumTotBeds"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_tot_beds: Vec<CvdNumTotBedsProperty>,
	/// <https://schema.org/cvdNumVent>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumVent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_vent: Vec<CvdNumVentProperty>,
	/// <https://schema.org/cvdNumVentUse>
	#[cfg_attr(feature = "serde", serde(rename = "cvdNumVentUse"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cvd_num_vent_use: Vec<CvdNumVentUseProperty>,
	/// <https://schema.org/datePosted>
	#[cfg_attr(feature = "serde", serde(rename = "datePosted"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#date_posted: Vec<DatePostedProperty>,
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
/// This trait is for properties from <https://schema.org/CDCPMDRecord>.
pub trait CdcpmdRecordTrait {
	/// Get <https://schema.org/cvdCollectionDate> from [`Self`] as borrowed slice.
	fn r#cvd_collection_date(&self) -> &[CvdCollectionDateProperty];
	/// Get <https://schema.org/cvdFacilityCounty> from [`Self`] as borrowed slice.
	fn r#cvd_facility_county(&self) -> &[CvdFacilityCountyProperty];
	/// Get <https://schema.org/cvdFacilityId> from [`Self`] as borrowed slice.
	fn r#cvd_facility_id(&self) -> &[CvdFacilityIdProperty];
	/// Get <https://schema.org/cvdNumBeds> from [`Self`] as borrowed slice.
	fn r#cvd_num_beds(&self) -> &[CvdNumBedsProperty];
	/// Get <https://schema.org/cvdNumBedsOcc> from [`Self`] as borrowed slice.
	fn r#cvd_num_beds_occ(&self) -> &[CvdNumBedsOccProperty];
	/// Get <https://schema.org/cvdNumC19Died> from [`Self`] as borrowed slice.
	fn r#cvd_num_c_19_died(&self) -> &[CvdNumC19DiedProperty];
	/// Get <https://schema.org/cvdNumC19HOPats> from [`Self`] as borrowed slice.
	fn r#cvd_num_c_19_ho_pats(&self) -> &[CvdNumC19HoPatsProperty];
	/// Get <https://schema.org/cvdNumC19HospPats> from [`Self`] as borrowed slice.
	fn r#cvd_num_c_19_hosp_pats(&self) -> &[CvdNumC19HospPatsProperty];
	/// Get <https://schema.org/cvdNumC19MechVentPats> from [`Self`] as borrowed slice.
	fn r#cvd_num_c_19_mech_vent_pats(&self) -> &[CvdNumC19MechVentPatsProperty];
	/// Get <https://schema.org/cvdNumC19OFMechVentPats> from [`Self`] as borrowed slice.
	fn r#cvd_num_c_19_of_mech_vent_pats(&self) -> &[CvdNumC19OfMechVentPatsProperty];
	/// Get <https://schema.org/cvdNumC19OverflowPats> from [`Self`] as borrowed slice.
	fn r#cvd_num_c_19_overflow_pats(&self) -> &[CvdNumC19OverflowPatsProperty];
	/// Get <https://schema.org/cvdNumICUBeds> from [`Self`] as borrowed slice.
	fn r#cvd_num_icu_beds(&self) -> &[CvdNumIcuBedsProperty];
	/// Get <https://schema.org/cvdNumICUBedsOcc> from [`Self`] as borrowed slice.
	fn r#cvd_num_icu_beds_occ(&self) -> &[CvdNumIcuBedsOccProperty];
	/// Get <https://schema.org/cvdNumTotBeds> from [`Self`] as borrowed slice.
	fn r#cvd_num_tot_beds(&self) -> &[CvdNumTotBedsProperty];
	/// Get <https://schema.org/cvdNumVent> from [`Self`] as borrowed slice.
	fn r#cvd_num_vent(&self) -> &[CvdNumVentProperty];
	/// Get <https://schema.org/cvdNumVentUse> from [`Self`] as borrowed slice.
	fn r#cvd_num_vent_use(&self) -> &[CvdNumVentUseProperty];
	/// Get <https://schema.org/datePosted> from [`Self`] as borrowed slice.
	fn r#date_posted(&self) -> &[DatePostedProperty];
}
impl CdcpmdRecordTrait for CdcpmdRecord {
	fn r#cvd_collection_date(&self) -> &[CvdCollectionDateProperty] {
		self.r#cvd_collection_date.as_slice()
	}
	fn r#cvd_facility_county(&self) -> &[CvdFacilityCountyProperty] {
		self.r#cvd_facility_county.as_slice()
	}
	fn r#cvd_facility_id(&self) -> &[CvdFacilityIdProperty] {
		self.r#cvd_facility_id.as_slice()
	}
	fn r#cvd_num_beds(&self) -> &[CvdNumBedsProperty] {
		self.r#cvd_num_beds.as_slice()
	}
	fn r#cvd_num_beds_occ(&self) -> &[CvdNumBedsOccProperty] {
		self.r#cvd_num_beds_occ.as_slice()
	}
	fn r#cvd_num_c_19_died(&self) -> &[CvdNumC19DiedProperty] {
		self.r#cvd_num_c_19_died.as_slice()
	}
	fn r#cvd_num_c_19_ho_pats(&self) -> &[CvdNumC19HoPatsProperty] {
		self.r#cvd_num_c_19_ho_pats.as_slice()
	}
	fn r#cvd_num_c_19_hosp_pats(&self) -> &[CvdNumC19HospPatsProperty] {
		self.r#cvd_num_c_19_hosp_pats.as_slice()
	}
	fn r#cvd_num_c_19_mech_vent_pats(&self) -> &[CvdNumC19MechVentPatsProperty] {
		self.r#cvd_num_c_19_mech_vent_pats.as_slice()
	}
	fn r#cvd_num_c_19_of_mech_vent_pats(&self) -> &[CvdNumC19OfMechVentPatsProperty] {
		self.r#cvd_num_c_19_of_mech_vent_pats.as_slice()
	}
	fn r#cvd_num_c_19_overflow_pats(&self) -> &[CvdNumC19OverflowPatsProperty] {
		self.r#cvd_num_c_19_overflow_pats.as_slice()
	}
	fn r#cvd_num_icu_beds(&self) -> &[CvdNumIcuBedsProperty] {
		self.r#cvd_num_icu_beds.as_slice()
	}
	fn r#cvd_num_icu_beds_occ(&self) -> &[CvdNumIcuBedsOccProperty] {
		self.r#cvd_num_icu_beds_occ.as_slice()
	}
	fn r#cvd_num_tot_beds(&self) -> &[CvdNumTotBedsProperty] {
		self.r#cvd_num_tot_beds.as_slice()
	}
	fn r#cvd_num_vent(&self) -> &[CvdNumVentProperty] {
		self.r#cvd_num_vent.as_slice()
	}
	fn r#cvd_num_vent_use(&self) -> &[CvdNumVentUseProperty] {
		self.r#cvd_num_vent_use.as_slice()
	}
	fn r#date_posted(&self) -> &[DatePostedProperty] {
		self.r#date_posted.as_slice()
	}
}
impl StructuredValueTrait for CdcpmdRecord {}
impl ThingTrait for CdcpmdRecord {
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
