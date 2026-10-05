use super::*;
/// <https://schema.org/Invoice>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Invoice {
	/// <https://schema.org/accountId>
	#[cfg_attr(feature = "serde", serde(rename = "accountId"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#account_id: Vec<AccountIdProperty>,
	/// <https://schema.org/billingPeriod>
	#[cfg_attr(feature = "serde", serde(rename = "billingPeriod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#billing_period: Vec<BillingPeriodProperty>,
	/// <https://schema.org/broker>
	#[cfg_attr(feature = "serde", serde(rename = "broker"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#broker: Vec<BrokerProperty>,
	/// <https://schema.org/category>
	#[cfg_attr(feature = "serde", serde(rename = "category"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#category: Vec<CategoryProperty>,
	/// <https://schema.org/confirmationNumber>
	#[cfg_attr(feature = "serde", serde(rename = "confirmationNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#confirmation_number: Vec<ConfirmationNumberProperty>,
	/// <https://schema.org/customer>
	#[cfg_attr(feature = "serde", serde(rename = "customer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#customer: Vec<CustomerProperty>,
	/// <https://schema.org/minimumPaymentDue>
	#[cfg_attr(feature = "serde", serde(rename = "minimumPaymentDue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#minimum_payment_due: Vec<MinimumPaymentDueProperty>,
	/// <https://schema.org/paymentDue>
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	#[cfg_attr(feature = "serde", serde(rename = "paymentDue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_due: Vec<PaymentDueProperty>,
	/// <https://schema.org/paymentDueDate>
	#[cfg_attr(feature = "serde", serde(rename = "paymentDueDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_due_date: Vec<PaymentDueDateProperty>,
	/// <https://schema.org/paymentMethod>
	#[cfg_attr(feature = "serde", serde(rename = "paymentMethod"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_method: Vec<PaymentMethodProperty>,
	/// <https://schema.org/paymentMethodId>
	#[cfg_attr(feature = "serde", serde(rename = "paymentMethodId"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_method_id: Vec<PaymentMethodIdProperty>,
	/// <https://schema.org/paymentStatus>
	#[cfg_attr(feature = "serde", serde(rename = "paymentStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_status: Vec<PaymentStatusProperty>,
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
	/// <https://schema.org/referencesOrder>
	#[cfg_attr(feature = "serde", serde(rename = "referencesOrder"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#references_order: Vec<ReferencesOrderProperty>,
	/// <https://schema.org/scheduledPaymentDate>
	#[cfg_attr(feature = "serde", serde(rename = "scheduledPaymentDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#scheduled_payment_date: Vec<ScheduledPaymentDateProperty>,
	/// <https://schema.org/totalPaymentDue>
	#[cfg_attr(feature = "serde", serde(rename = "totalPaymentDue"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#total_payment_due: Vec<TotalPaymentDueProperty>,
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
/// This trait is for properties from <https://schema.org/Invoice>.
pub trait InvoiceTrait {
	/// Get <https://schema.org/accountId> from [`Self`] as borrowed slice.
	fn r#account_id(&self) -> &[AccountIdProperty];
	/// Get <https://schema.org/billingPeriod> from [`Self`] as borrowed slice.
	fn r#billing_period(&self) -> &[BillingPeriodProperty];
	/// Get <https://schema.org/broker> from [`Self`] as borrowed slice.
	fn r#broker(&self) -> &[BrokerProperty];
	/// Get <https://schema.org/category> from [`Self`] as borrowed slice.
	fn r#category(&self) -> &[CategoryProperty];
	/// Get <https://schema.org/confirmationNumber> from [`Self`] as borrowed slice.
	fn r#confirmation_number(&self) -> &[ConfirmationNumberProperty];
	/// Get <https://schema.org/customer> from [`Self`] as borrowed slice.
	fn r#customer(&self) -> &[CustomerProperty];
	/// Get <https://schema.org/minimumPaymentDue> from [`Self`] as borrowed slice.
	fn r#minimum_payment_due(&self) -> &[MinimumPaymentDueProperty];
	/// Get <https://schema.org/paymentDue> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	fn r#payment_due(&self) -> &[PaymentDueProperty];
	/// Get <https://schema.org/paymentDueDate> from [`Self`] as borrowed slice.
	fn r#payment_due_date(&self) -> &[PaymentDueDateProperty];
	/// Get <https://schema.org/paymentMethod> from [`Self`] as borrowed slice.
	fn r#payment_method(&self) -> &[PaymentMethodProperty];
	/// Get <https://schema.org/paymentMethodId> from [`Self`] as borrowed slice.
	fn r#payment_method_id(&self) -> &[PaymentMethodIdProperty];
	/// Get <https://schema.org/paymentStatus> from [`Self`] as borrowed slice.
	fn r#payment_status(&self) -> &[PaymentStatusProperty];
	/// Get <https://schema.org/provider> from [`Self`] as borrowed slice.
	fn r#provider(&self) -> &[ProviderProperty];
	/// Get <https://schema.org/referencesOrder> from [`Self`] as borrowed slice.
	fn r#references_order(&self) -> &[ReferencesOrderProperty];
	/// Get <https://schema.org/scheduledPaymentDate> from [`Self`] as borrowed slice.
	fn r#scheduled_payment_date(&self) -> &[ScheduledPaymentDateProperty];
	/// Get <https://schema.org/totalPaymentDue> from [`Self`] as borrowed slice.
	fn r#total_payment_due(&self) -> &[TotalPaymentDueProperty];
}
impl InvoiceTrait for Invoice {
	fn r#account_id(&self) -> &[AccountIdProperty] {
		self.r#account_id.as_slice()
	}
	fn r#billing_period(&self) -> &[BillingPeriodProperty] {
		self.r#billing_period.as_slice()
	}
	fn r#broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn r#category(&self) -> &[CategoryProperty] {
		self.r#category.as_slice()
	}
	fn r#confirmation_number(&self) -> &[ConfirmationNumberProperty] {
		self.r#confirmation_number.as_slice()
	}
	fn r#customer(&self) -> &[CustomerProperty] {
		self.r#customer.as_slice()
	}
	fn r#minimum_payment_due(&self) -> &[MinimumPaymentDueProperty] {
		self.r#minimum_payment_due.as_slice()
	}
	fn r#payment_due(&self) -> &[PaymentDueProperty] {
		self.r#payment_due.as_slice()
	}
	fn r#payment_due_date(&self) -> &[PaymentDueDateProperty] {
		self.r#payment_due_date.as_slice()
	}
	fn r#payment_method(&self) -> &[PaymentMethodProperty] {
		self.r#payment_method.as_slice()
	}
	fn r#payment_method_id(&self) -> &[PaymentMethodIdProperty] {
		self.r#payment_method_id.as_slice()
	}
	fn r#payment_status(&self) -> &[PaymentStatusProperty] {
		self.r#payment_status.as_slice()
	}
	fn r#provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn r#references_order(&self) -> &[ReferencesOrderProperty] {
		self.r#references_order.as_slice()
	}
	fn r#scheduled_payment_date(&self) -> &[ScheduledPaymentDateProperty] {
		self.r#scheduled_payment_date.as_slice()
	}
	fn r#total_payment_due(&self) -> &[TotalPaymentDueProperty] {
		self.r#total_payment_due.as_slice()
	}
}
impl ThingTrait for Invoice {
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
