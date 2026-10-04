use super::*;
/// <https://schema.org/NutritionInformation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct NutritionInformation {
	/// <https://schema.org/calories>
	#[cfg_attr(feature = "serde", serde(rename = "calories"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#calories: Vec<CaloriesProperty>,
	/// <https://schema.org/carbohydrateContent>
	#[cfg_attr(feature = "serde", serde(rename = "carbohydrateContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#carbohydrate_content: Vec<CarbohydrateContentProperty>,
	/// <https://schema.org/cholesterolContent>
	#[cfg_attr(feature = "serde", serde(rename = "cholesterolContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#cholesterol_content: Vec<CholesterolContentProperty>,
	/// <https://schema.org/fatContent>
	#[cfg_attr(feature = "serde", serde(rename = "fatContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#fat_content: Vec<FatContentProperty>,
	/// <https://schema.org/fiberContent>
	#[cfg_attr(feature = "serde", serde(rename = "fiberContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#fiber_content: Vec<FiberContentProperty>,
	/// <https://schema.org/proteinContent>
	#[cfg_attr(feature = "serde", serde(rename = "proteinContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#protein_content: Vec<ProteinContentProperty>,
	/// <https://schema.org/saturatedFatContent>
	#[cfg_attr(feature = "serde", serde(rename = "saturatedFatContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#saturated_fat_content: Vec<SaturatedFatContentProperty>,
	/// <https://schema.org/servingSize>
	#[cfg_attr(feature = "serde", serde(rename = "servingSize"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#serving_size: Vec<ServingSizeProperty>,
	/// <https://schema.org/sodiumContent>
	#[cfg_attr(feature = "serde", serde(rename = "sodiumContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sodium_content: Vec<SodiumContentProperty>,
	/// <https://schema.org/sugarContent>
	#[cfg_attr(feature = "serde", serde(rename = "sugarContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#sugar_content: Vec<SugarContentProperty>,
	/// <https://schema.org/transFatContent>
	#[cfg_attr(feature = "serde", serde(rename = "transFatContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#trans_fat_content: Vec<TransFatContentProperty>,
	/// <https://schema.org/unsaturatedFatContent>
	#[cfg_attr(feature = "serde", serde(rename = "unsaturatedFatContent"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#unsaturated_fat_content: Vec<UnsaturatedFatContentProperty>,
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
/// This trait is for properties from <https://schema.org/NutritionInformation>.
pub trait NutritionInformationTrait {
	/// Get <https://schema.org/calories> from [`Self`] as borrowed slice.
	fn get_calories(&self) -> &[CaloriesProperty];
	/// Take <https://schema.org/calories> from [`Self`] as owned vector.
	fn take_calories(&mut self) -> Vec<CaloriesProperty>;
	/// Get <https://schema.org/carbohydrateContent> from [`Self`] as borrowed slice.
	fn get_carbohydrate_content(&self) -> &[CarbohydrateContentProperty];
	/// Take <https://schema.org/carbohydrateContent> from [`Self`] as owned vector.
	fn take_carbohydrate_content(&mut self) -> Vec<CarbohydrateContentProperty>;
	/// Get <https://schema.org/cholesterolContent> from [`Self`] as borrowed slice.
	fn get_cholesterol_content(&self) -> &[CholesterolContentProperty];
	/// Take <https://schema.org/cholesterolContent> from [`Self`] as owned vector.
	fn take_cholesterol_content(&mut self) -> Vec<CholesterolContentProperty>;
	/// Get <https://schema.org/fatContent> from [`Self`] as borrowed slice.
	fn get_fat_content(&self) -> &[FatContentProperty];
	/// Take <https://schema.org/fatContent> from [`Self`] as owned vector.
	fn take_fat_content(&mut self) -> Vec<FatContentProperty>;
	/// Get <https://schema.org/fiberContent> from [`Self`] as borrowed slice.
	fn get_fiber_content(&self) -> &[FiberContentProperty];
	/// Take <https://schema.org/fiberContent> from [`Self`] as owned vector.
	fn take_fiber_content(&mut self) -> Vec<FiberContentProperty>;
	/// Get <https://schema.org/proteinContent> from [`Self`] as borrowed slice.
	fn get_protein_content(&self) -> &[ProteinContentProperty];
	/// Take <https://schema.org/proteinContent> from [`Self`] as owned vector.
	fn take_protein_content(&mut self) -> Vec<ProteinContentProperty>;
	/// Get <https://schema.org/saturatedFatContent> from [`Self`] as borrowed slice.
	fn get_saturated_fat_content(&self) -> &[SaturatedFatContentProperty];
	/// Take <https://schema.org/saturatedFatContent> from [`Self`] as owned vector.
	fn take_saturated_fat_content(&mut self) -> Vec<SaturatedFatContentProperty>;
	/// Get <https://schema.org/servingSize> from [`Self`] as borrowed slice.
	fn get_serving_size(&self) -> &[ServingSizeProperty];
	/// Take <https://schema.org/servingSize> from [`Self`] as owned vector.
	fn take_serving_size(&mut self) -> Vec<ServingSizeProperty>;
	/// Get <https://schema.org/sodiumContent> from [`Self`] as borrowed slice.
	fn get_sodium_content(&self) -> &[SodiumContentProperty];
	/// Take <https://schema.org/sodiumContent> from [`Self`] as owned vector.
	fn take_sodium_content(&mut self) -> Vec<SodiumContentProperty>;
	/// Get <https://schema.org/sugarContent> from [`Self`] as borrowed slice.
	fn get_sugar_content(&self) -> &[SugarContentProperty];
	/// Take <https://schema.org/sugarContent> from [`Self`] as owned vector.
	fn take_sugar_content(&mut self) -> Vec<SugarContentProperty>;
	/// Get <https://schema.org/transFatContent> from [`Self`] as borrowed slice.
	fn get_trans_fat_content(&self) -> &[TransFatContentProperty];
	/// Take <https://schema.org/transFatContent> from [`Self`] as owned vector.
	fn take_trans_fat_content(&mut self) -> Vec<TransFatContentProperty>;
	/// Get <https://schema.org/unsaturatedFatContent> from [`Self`] as borrowed slice.
	fn get_unsaturated_fat_content(&self) -> &[UnsaturatedFatContentProperty];
	/// Take <https://schema.org/unsaturatedFatContent> from [`Self`] as owned vector.
	fn take_unsaturated_fat_content(&mut self) -> Vec<UnsaturatedFatContentProperty>;
}
impl NutritionInformationTrait for NutritionInformation {
	fn get_calories(&self) -> &[CaloriesProperty] {
		self.r#calories.as_slice()
	}
	fn take_calories(&mut self) -> Vec<CaloriesProperty> {
		std::mem::take(&mut self.r#calories)
	}
	fn get_carbohydrate_content(&self) -> &[CarbohydrateContentProperty] {
		self.r#carbohydrate_content.as_slice()
	}
	fn take_carbohydrate_content(&mut self) -> Vec<CarbohydrateContentProperty> {
		std::mem::take(&mut self.r#carbohydrate_content)
	}
	fn get_cholesterol_content(&self) -> &[CholesterolContentProperty] {
		self.r#cholesterol_content.as_slice()
	}
	fn take_cholesterol_content(&mut self) -> Vec<CholesterolContentProperty> {
		std::mem::take(&mut self.r#cholesterol_content)
	}
	fn get_fat_content(&self) -> &[FatContentProperty] {
		self.r#fat_content.as_slice()
	}
	fn take_fat_content(&mut self) -> Vec<FatContentProperty> {
		std::mem::take(&mut self.r#fat_content)
	}
	fn get_fiber_content(&self) -> &[FiberContentProperty] {
		self.r#fiber_content.as_slice()
	}
	fn take_fiber_content(&mut self) -> Vec<FiberContentProperty> {
		std::mem::take(&mut self.r#fiber_content)
	}
	fn get_protein_content(&self) -> &[ProteinContentProperty] {
		self.r#protein_content.as_slice()
	}
	fn take_protein_content(&mut self) -> Vec<ProteinContentProperty> {
		std::mem::take(&mut self.r#protein_content)
	}
	fn get_saturated_fat_content(&self) -> &[SaturatedFatContentProperty] {
		self.r#saturated_fat_content.as_slice()
	}
	fn take_saturated_fat_content(&mut self) -> Vec<SaturatedFatContentProperty> {
		std::mem::take(&mut self.r#saturated_fat_content)
	}
	fn get_serving_size(&self) -> &[ServingSizeProperty] {
		self.r#serving_size.as_slice()
	}
	fn take_serving_size(&mut self) -> Vec<ServingSizeProperty> {
		std::mem::take(&mut self.r#serving_size)
	}
	fn get_sodium_content(&self) -> &[SodiumContentProperty] {
		self.r#sodium_content.as_slice()
	}
	fn take_sodium_content(&mut self) -> Vec<SodiumContentProperty> {
		std::mem::take(&mut self.r#sodium_content)
	}
	fn get_sugar_content(&self) -> &[SugarContentProperty] {
		self.r#sugar_content.as_slice()
	}
	fn take_sugar_content(&mut self) -> Vec<SugarContentProperty> {
		std::mem::take(&mut self.r#sugar_content)
	}
	fn get_trans_fat_content(&self) -> &[TransFatContentProperty] {
		self.r#trans_fat_content.as_slice()
	}
	fn take_trans_fat_content(&mut self) -> Vec<TransFatContentProperty> {
		std::mem::take(&mut self.r#trans_fat_content)
	}
	fn get_unsaturated_fat_content(&self) -> &[UnsaturatedFatContentProperty] {
		self.r#unsaturated_fat_content.as_slice()
	}
	fn take_unsaturated_fat_content(&mut self) -> Vec<UnsaturatedFatContentProperty> {
		std::mem::take(&mut self.r#unsaturated_fat_content)
	}
}
impl StructuredValueTrait for NutritionInformation {}
impl ThingTrait for NutritionInformation {
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
