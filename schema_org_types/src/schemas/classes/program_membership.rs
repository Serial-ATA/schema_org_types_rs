use super::*;
/// <https://schema.org/ProgramMembership>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ProgramMembership {
	/// <https://schema.org/hostingOrganization>
	#[cfg_attr(feature = "serde", serde(rename = "hostingOrganization"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#hosting_organization: Vec<HostingOrganizationProperty>,
	/// <https://schema.org/member>
	#[cfg_attr(feature = "serde", serde(rename = "member"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#member: Vec<MemberProperty>,
	/// <https://schema.org/members>
	#[deprecated = "This schema is superseded by <https://schema.org/member>."]
	#[cfg_attr(feature = "serde", serde(rename = "members"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#members: Vec<MembersProperty>,
	/// <https://schema.org/membershipNumber>
	#[cfg_attr(feature = "serde", serde(rename = "membershipNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#membership_number: Vec<MembershipNumberProperty>,
	/// <https://schema.org/membershipPointsEarned>
	#[cfg_attr(feature = "serde", serde(rename = "membershipPointsEarned"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#membership_points_earned: Vec<MembershipPointsEarnedProperty>,
	/// <https://schema.org/program>
	#[cfg_attr(feature = "serde", serde(rename = "program"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#program: Vec<ProgramProperty>,
	/// <https://schema.org/programName>
	#[cfg_attr(feature = "serde", serde(rename = "programName"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#program_name: Vec<ProgramNameProperty>,
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
/// This trait is for properties from <https://schema.org/ProgramMembership>.
pub trait ProgramMembershipTrait {
	/// Get <https://schema.org/hostingOrganization> from [`Self`] as borrowed slice.
	fn get_hosting_organization(&self) -> &[HostingOrganizationProperty];
	/// Take <https://schema.org/hostingOrganization> from [`Self`] as owned vector.
	fn take_hosting_organization(&mut self) -> Vec<HostingOrganizationProperty>;
	/// Get <https://schema.org/member> from [`Self`] as borrowed slice.
	fn get_member(&self) -> &[MemberProperty];
	/// Take <https://schema.org/member> from [`Self`] as owned vector.
	fn take_member(&mut self) -> Vec<MemberProperty>;
	/// Get <https://schema.org/members> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/member>."]
	fn get_members(&self) -> &[MembersProperty];
	/// Take <https://schema.org/members> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/member>."]
	fn take_members(&mut self) -> Vec<MembersProperty>;
	/// Get <https://schema.org/membershipNumber> from [`Self`] as borrowed slice.
	fn get_membership_number(&self) -> &[MembershipNumberProperty];
	/// Take <https://schema.org/membershipNumber> from [`Self`] as owned vector.
	fn take_membership_number(&mut self) -> Vec<MembershipNumberProperty>;
	/// Get <https://schema.org/membershipPointsEarned> from [`Self`] as borrowed slice.
	fn get_membership_points_earned(&self) -> &[MembershipPointsEarnedProperty];
	/// Take <https://schema.org/membershipPointsEarned> from [`Self`] as owned vector.
	fn take_membership_points_earned(&mut self) -> Vec<MembershipPointsEarnedProperty>;
	/// Get <https://schema.org/program> from [`Self`] as borrowed slice.
	fn get_program(&self) -> &[ProgramProperty];
	/// Take <https://schema.org/program> from [`Self`] as owned vector.
	fn take_program(&mut self) -> Vec<ProgramProperty>;
	/// Get <https://schema.org/programName> from [`Self`] as borrowed slice.
	fn get_program_name(&self) -> &[ProgramNameProperty];
	/// Take <https://schema.org/programName> from [`Self`] as owned vector.
	fn take_program_name(&mut self) -> Vec<ProgramNameProperty>;
}
impl ProgramMembershipTrait for ProgramMembership {
	fn get_hosting_organization(&self) -> &[HostingOrganizationProperty] {
		self.r#hosting_organization.as_slice()
	}
	fn take_hosting_organization(&mut self) -> Vec<HostingOrganizationProperty> {
		std::mem::take(&mut self.r#hosting_organization)
	}
	fn get_member(&self) -> &[MemberProperty] {
		self.r#member.as_slice()
	}
	fn take_member(&mut self) -> Vec<MemberProperty> {
		std::mem::take(&mut self.r#member)
	}
	fn get_members(&self) -> &[MembersProperty] {
		self.r#members.as_slice()
	}
	fn take_members(&mut self) -> Vec<MembersProperty> {
		std::mem::take(&mut self.r#members)
	}
	fn get_membership_number(&self) -> &[MembershipNumberProperty] {
		self.r#membership_number.as_slice()
	}
	fn take_membership_number(&mut self) -> Vec<MembershipNumberProperty> {
		std::mem::take(&mut self.r#membership_number)
	}
	fn get_membership_points_earned(&self) -> &[MembershipPointsEarnedProperty] {
		self.r#membership_points_earned.as_slice()
	}
	fn take_membership_points_earned(&mut self) -> Vec<MembershipPointsEarnedProperty> {
		std::mem::take(&mut self.r#membership_points_earned)
	}
	fn get_program(&self) -> &[ProgramProperty] {
		self.r#program.as_slice()
	}
	fn take_program(&mut self) -> Vec<ProgramProperty> {
		std::mem::take(&mut self.r#program)
	}
	fn get_program_name(&self) -> &[ProgramNameProperty] {
		self.r#program_name.as_slice()
	}
	fn take_program_name(&mut self) -> Vec<ProgramNameProperty> {
		std::mem::take(&mut self.r#program_name)
	}
}
impl ThingTrait for ProgramMembership {
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
