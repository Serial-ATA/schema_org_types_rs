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
	fn r#calories(&self) -> &[CaloriesProperty];
	/// Get <https://schema.org/carbohydrateContent> from [`Self`] as borrowed slice.
	fn r#carbohydrate_content(&self) -> &[CarbohydrateContentProperty];
	/// Get <https://schema.org/cholesterolContent> from [`Self`] as borrowed slice.
	fn r#cholesterol_content(&self) -> &[CholesterolContentProperty];
	/// Get <https://schema.org/fatContent> from [`Self`] as borrowed slice.
	fn r#fat_content(&self) -> &[FatContentProperty];
	/// Get <https://schema.org/fiberContent> from [`Self`] as borrowed slice.
	fn r#fiber_content(&self) -> &[FiberContentProperty];
	/// Get <https://schema.org/proteinContent> from [`Self`] as borrowed slice.
	fn r#protein_content(&self) -> &[ProteinContentProperty];
	/// Get <https://schema.org/saturatedFatContent> from [`Self`] as borrowed slice.
	fn r#saturated_fat_content(&self) -> &[SaturatedFatContentProperty];
	/// Get <https://schema.org/servingSize> from [`Self`] as borrowed slice.
	fn r#serving_size(&self) -> &[ServingSizeProperty];
	/// Get <https://schema.org/sodiumContent> from [`Self`] as borrowed slice.
	fn r#sodium_content(&self) -> &[SodiumContentProperty];
	/// Get <https://schema.org/sugarContent> from [`Self`] as borrowed slice.
	fn r#sugar_content(&self) -> &[SugarContentProperty];
	/// Get <https://schema.org/transFatContent> from [`Self`] as borrowed slice.
	fn r#trans_fat_content(&self) -> &[TransFatContentProperty];
	/// Get <https://schema.org/unsaturatedFatContent> from [`Self`] as borrowed slice.
	fn r#unsaturated_fat_content(&self) -> &[UnsaturatedFatContentProperty];
}
impl NutritionInformationTrait for NutritionInformation {
	fn r#calories(&self) -> &[CaloriesProperty] {
		self.r#calories.as_slice()
	}
	fn r#carbohydrate_content(&self) -> &[CarbohydrateContentProperty] {
		self.r#carbohydrate_content.as_slice()
	}
	fn r#cholesterol_content(&self) -> &[CholesterolContentProperty] {
		self.r#cholesterol_content.as_slice()
	}
	fn r#fat_content(&self) -> &[FatContentProperty] {
		self.r#fat_content.as_slice()
	}
	fn r#fiber_content(&self) -> &[FiberContentProperty] {
		self.r#fiber_content.as_slice()
	}
	fn r#protein_content(&self) -> &[ProteinContentProperty] {
		self.r#protein_content.as_slice()
	}
	fn r#saturated_fat_content(&self) -> &[SaturatedFatContentProperty] {
		self.r#saturated_fat_content.as_slice()
	}
	fn r#serving_size(&self) -> &[ServingSizeProperty] {
		self.r#serving_size.as_slice()
	}
	fn r#sodium_content(&self) -> &[SodiumContentProperty] {
		self.r#sodium_content.as_slice()
	}
	fn r#sugar_content(&self) -> &[SugarContentProperty] {
		self.r#sugar_content.as_slice()
	}
	fn r#trans_fat_content(&self) -> &[TransFatContentProperty] {
		self.r#trans_fat_content.as_slice()
	}
	fn r#unsaturated_fat_content(&self) -> &[UnsaturatedFatContentProperty] {
		self.r#unsaturated_fat_content.as_slice()
	}
}
impl StructuredValueTrait for NutritionInformation {}
impl ThingTrait for NutritionInformation {
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
