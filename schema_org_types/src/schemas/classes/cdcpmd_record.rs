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
	fn get_cvd_collection_date(&self) -> &[CvdCollectionDateProperty];
	/// Take <https://schema.org/cvdCollectionDate> from [`Self`] as owned vector.
	fn take_cvd_collection_date(&mut self) -> Vec<CvdCollectionDateProperty>;
	/// Get <https://schema.org/cvdFacilityCounty> from [`Self`] as borrowed slice.
	fn get_cvd_facility_county(&self) -> &[CvdFacilityCountyProperty];
	/// Take <https://schema.org/cvdFacilityCounty> from [`Self`] as owned vector.
	fn take_cvd_facility_county(&mut self) -> Vec<CvdFacilityCountyProperty>;
	/// Get <https://schema.org/cvdFacilityId> from [`Self`] as borrowed slice.
	fn get_cvd_facility_id(&self) -> &[CvdFacilityIdProperty];
	/// Take <https://schema.org/cvdFacilityId> from [`Self`] as owned vector.
	fn take_cvd_facility_id(&mut self) -> Vec<CvdFacilityIdProperty>;
	/// Get <https://schema.org/cvdNumBeds> from [`Self`] as borrowed slice.
	fn get_cvd_num_beds(&self) -> &[CvdNumBedsProperty];
	/// Take <https://schema.org/cvdNumBeds> from [`Self`] as owned vector.
	fn take_cvd_num_beds(&mut self) -> Vec<CvdNumBedsProperty>;
	/// Get <https://schema.org/cvdNumBedsOcc> from [`Self`] as borrowed slice.
	fn get_cvd_num_beds_occ(&self) -> &[CvdNumBedsOccProperty];
	/// Take <https://schema.org/cvdNumBedsOcc> from [`Self`] as owned vector.
	fn take_cvd_num_beds_occ(&mut self) -> Vec<CvdNumBedsOccProperty>;
	/// Get <https://schema.org/cvdNumC19Died> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_died(&self) -> &[CvdNumC19DiedProperty];
	/// Take <https://schema.org/cvdNumC19Died> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_died(&mut self) -> Vec<CvdNumC19DiedProperty>;
	/// Get <https://schema.org/cvdNumC19HOPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_ho_pats(&self) -> &[CvdNumC19HoPatsProperty];
	/// Take <https://schema.org/cvdNumC19HOPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_ho_pats(&mut self) -> Vec<CvdNumC19HoPatsProperty>;
	/// Get <https://schema.org/cvdNumC19HospPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_hosp_pats(&self) -> &[CvdNumC19HospPatsProperty];
	/// Take <https://schema.org/cvdNumC19HospPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_hosp_pats(&mut self) -> Vec<CvdNumC19HospPatsProperty>;
	/// Get <https://schema.org/cvdNumC19MechVentPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_mech_vent_pats(&self) -> &[CvdNumC19MechVentPatsProperty];
	/// Take <https://schema.org/cvdNumC19MechVentPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_mech_vent_pats(&mut self) -> Vec<CvdNumC19MechVentPatsProperty>;
	/// Get <https://schema.org/cvdNumC19OFMechVentPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_of_mech_vent_pats(&self) -> &[CvdNumC19OfMechVentPatsProperty];
	/// Take <https://schema.org/cvdNumC19OFMechVentPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_of_mech_vent_pats(&mut self) -> Vec<CvdNumC19OfMechVentPatsProperty>;
	/// Get <https://schema.org/cvdNumC19OverflowPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_overflow_pats(&self) -> &[CvdNumC19OverflowPatsProperty];
	/// Take <https://schema.org/cvdNumC19OverflowPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_overflow_pats(&mut self) -> Vec<CvdNumC19OverflowPatsProperty>;
	/// Get <https://schema.org/cvdNumICUBeds> from [`Self`] as borrowed slice.
	fn get_cvd_num_icu_beds(&self) -> &[CvdNumIcuBedsProperty];
	/// Take <https://schema.org/cvdNumICUBeds> from [`Self`] as owned vector.
	fn take_cvd_num_icu_beds(&mut self) -> Vec<CvdNumIcuBedsProperty>;
	/// Get <https://schema.org/cvdNumICUBedsOcc> from [`Self`] as borrowed slice.
	fn get_cvd_num_icu_beds_occ(&self) -> &[CvdNumIcuBedsOccProperty];
	/// Take <https://schema.org/cvdNumICUBedsOcc> from [`Self`] as owned vector.
	fn take_cvd_num_icu_beds_occ(&mut self) -> Vec<CvdNumIcuBedsOccProperty>;
	/// Get <https://schema.org/cvdNumTotBeds> from [`Self`] as borrowed slice.
	fn get_cvd_num_tot_beds(&self) -> &[CvdNumTotBedsProperty];
	/// Take <https://schema.org/cvdNumTotBeds> from [`Self`] as owned vector.
	fn take_cvd_num_tot_beds(&mut self) -> Vec<CvdNumTotBedsProperty>;
	/// Get <https://schema.org/cvdNumVent> from [`Self`] as borrowed slice.
	fn get_cvd_num_vent(&self) -> &[CvdNumVentProperty];
	/// Take <https://schema.org/cvdNumVent> from [`Self`] as owned vector.
	fn take_cvd_num_vent(&mut self) -> Vec<CvdNumVentProperty>;
	/// Get <https://schema.org/cvdNumVentUse> from [`Self`] as borrowed slice.
	fn get_cvd_num_vent_use(&self) -> &[CvdNumVentUseProperty];
	/// Take <https://schema.org/cvdNumVentUse> from [`Self`] as owned vector.
	fn take_cvd_num_vent_use(&mut self) -> Vec<CvdNumVentUseProperty>;
	/// Get <https://schema.org/datePosted> from [`Self`] as borrowed slice.
	fn get_date_posted(&self) -> &[DatePostedProperty];
	/// Take <https://schema.org/datePosted> from [`Self`] as owned vector.
	fn take_date_posted(&mut self) -> Vec<DatePostedProperty>;
}
impl CdcpmdRecordTrait for CdcpmdRecord {
	fn get_cvd_collection_date(&self) -> &[CvdCollectionDateProperty] {
		self.r#cvd_collection_date.as_slice()
	}
	fn take_cvd_collection_date(&mut self) -> Vec<CvdCollectionDateProperty> {
		std::mem::take(&mut self.r#cvd_collection_date)
	}
	fn get_cvd_facility_county(&self) -> &[CvdFacilityCountyProperty] {
		self.r#cvd_facility_county.as_slice()
	}
	fn take_cvd_facility_county(&mut self) -> Vec<CvdFacilityCountyProperty> {
		std::mem::take(&mut self.r#cvd_facility_county)
	}
	fn get_cvd_facility_id(&self) -> &[CvdFacilityIdProperty] {
		self.r#cvd_facility_id.as_slice()
	}
	fn take_cvd_facility_id(&mut self) -> Vec<CvdFacilityIdProperty> {
		std::mem::take(&mut self.r#cvd_facility_id)
	}
	fn get_cvd_num_beds(&self) -> &[CvdNumBedsProperty] {
		self.r#cvd_num_beds.as_slice()
	}
	fn take_cvd_num_beds(&mut self) -> Vec<CvdNumBedsProperty> {
		std::mem::take(&mut self.r#cvd_num_beds)
	}
	fn get_cvd_num_beds_occ(&self) -> &[CvdNumBedsOccProperty] {
		self.r#cvd_num_beds_occ.as_slice()
	}
	fn take_cvd_num_beds_occ(&mut self) -> Vec<CvdNumBedsOccProperty> {
		std::mem::take(&mut self.r#cvd_num_beds_occ)
	}
	fn get_cvd_num_c_19_died(&self) -> &[CvdNumC19DiedProperty] {
		self.r#cvd_num_c_19_died.as_slice()
	}
	fn take_cvd_num_c_19_died(&mut self) -> Vec<CvdNumC19DiedProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_died)
	}
	fn get_cvd_num_c_19_ho_pats(&self) -> &[CvdNumC19HoPatsProperty] {
		self.r#cvd_num_c_19_ho_pats.as_slice()
	}
	fn take_cvd_num_c_19_ho_pats(&mut self) -> Vec<CvdNumC19HoPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_ho_pats)
	}
	fn get_cvd_num_c_19_hosp_pats(&self) -> &[CvdNumC19HospPatsProperty] {
		self.r#cvd_num_c_19_hosp_pats.as_slice()
	}
	fn take_cvd_num_c_19_hosp_pats(&mut self) -> Vec<CvdNumC19HospPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_hosp_pats)
	}
	fn get_cvd_num_c_19_mech_vent_pats(&self) -> &[CvdNumC19MechVentPatsProperty] {
		self.r#cvd_num_c_19_mech_vent_pats.as_slice()
	}
	fn take_cvd_num_c_19_mech_vent_pats(&mut self) -> Vec<CvdNumC19MechVentPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_mech_vent_pats)
	}
	fn get_cvd_num_c_19_of_mech_vent_pats(&self) -> &[CvdNumC19OfMechVentPatsProperty] {
		self.r#cvd_num_c_19_of_mech_vent_pats.as_slice()
	}
	fn take_cvd_num_c_19_of_mech_vent_pats(&mut self) -> Vec<CvdNumC19OfMechVentPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_of_mech_vent_pats)
	}
	fn get_cvd_num_c_19_overflow_pats(&self) -> &[CvdNumC19OverflowPatsProperty] {
		self.r#cvd_num_c_19_overflow_pats.as_slice()
	}
	fn take_cvd_num_c_19_overflow_pats(&mut self) -> Vec<CvdNumC19OverflowPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_overflow_pats)
	}
	fn get_cvd_num_icu_beds(&self) -> &[CvdNumIcuBedsProperty] {
		self.r#cvd_num_icu_beds.as_slice()
	}
	fn take_cvd_num_icu_beds(&mut self) -> Vec<CvdNumIcuBedsProperty> {
		std::mem::take(&mut self.r#cvd_num_icu_beds)
	}
	fn get_cvd_num_icu_beds_occ(&self) -> &[CvdNumIcuBedsOccProperty] {
		self.r#cvd_num_icu_beds_occ.as_slice()
	}
	fn take_cvd_num_icu_beds_occ(&mut self) -> Vec<CvdNumIcuBedsOccProperty> {
		std::mem::take(&mut self.r#cvd_num_icu_beds_occ)
	}
	fn get_cvd_num_tot_beds(&self) -> &[CvdNumTotBedsProperty] {
		self.r#cvd_num_tot_beds.as_slice()
	}
	fn take_cvd_num_tot_beds(&mut self) -> Vec<CvdNumTotBedsProperty> {
		std::mem::take(&mut self.r#cvd_num_tot_beds)
	}
	fn get_cvd_num_vent(&self) -> &[CvdNumVentProperty] {
		self.r#cvd_num_vent.as_slice()
	}
	fn take_cvd_num_vent(&mut self) -> Vec<CvdNumVentProperty> {
		std::mem::take(&mut self.r#cvd_num_vent)
	}
	fn get_cvd_num_vent_use(&self) -> &[CvdNumVentUseProperty] {
		self.r#cvd_num_vent_use.as_slice()
	}
	fn take_cvd_num_vent_use(&mut self) -> Vec<CvdNumVentUseProperty> {
		std::mem::take(&mut self.r#cvd_num_vent_use)
	}
	fn get_date_posted(&self) -> &[DatePostedProperty] {
		self.r#date_posted.as_slice()
	}
	fn take_date_posted(&mut self) -> Vec<DatePostedProperty> {
		std::mem::take(&mut self.r#date_posted)
	}
}
impl StructuredValueTrait for CdcpmdRecord {}
impl ThingTrait for CdcpmdRecord {
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
