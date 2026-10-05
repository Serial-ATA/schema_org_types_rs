use super::*;
/// <https://schema.org/Order>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", ::serde_with::serde_as)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Order {
	/// <https://schema.org/acceptedOffer>
	#[cfg_attr(feature = "serde", serde(rename = "acceptedOffer"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#accepted_offer: Vec<AcceptedOfferProperty>,
	/// <https://schema.org/billingAddress>
	#[cfg_attr(feature = "serde", serde(rename = "billingAddress"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#billing_address: Vec<BillingAddressProperty>,
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
	/// <https://schema.org/discount>
	#[cfg_attr(feature = "serde", serde(rename = "discount"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#discount: Vec<DiscountProperty>,
	/// <https://schema.org/discountCode>
	#[cfg_attr(feature = "serde", serde(rename = "discountCode"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#discount_code: Vec<DiscountCodeProperty>,
	/// <https://schema.org/discountCurrency>
	#[cfg_attr(feature = "serde", serde(rename = "discountCurrency"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#discount_currency: Vec<DiscountCurrencyProperty>,
	/// <https://schema.org/isGift>
	#[cfg_attr(feature = "serde", serde(rename = "isGift"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#is_gift: Vec<IsGiftProperty>,
	/// <https://schema.org/merchant>
	#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
	#[cfg_attr(feature = "serde", serde(rename = "merchant"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#merchant: Vec<MerchantProperty>,
	/// <https://schema.org/orderDate>
	#[cfg_attr(feature = "serde", serde(rename = "orderDate"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#order_date: Vec<OrderDateProperty>,
	/// <https://schema.org/orderDelivery>
	#[cfg_attr(feature = "serde", serde(rename = "orderDelivery"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#order_delivery: Vec<OrderDeliveryProperty>,
	/// <https://schema.org/orderNumber>
	#[cfg_attr(feature = "serde", serde(rename = "orderNumber"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#order_number: Vec<OrderNumberProperty>,
	/// <https://schema.org/orderStatus>
	#[cfg_attr(feature = "serde", serde(rename = "orderStatus"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#order_status: Vec<OrderStatusProperty>,
	/// <https://schema.org/orderedItem>
	#[cfg_attr(feature = "serde", serde(rename = "orderedItem"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#ordered_item: Vec<OrderedItemProperty>,
	/// <https://schema.org/partOfInvoice>
	#[cfg_attr(feature = "serde", serde(rename = "partOfInvoice"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#part_of_invoice: Vec<PartOfInvoiceProperty>,
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
	/// <https://schema.org/paymentUrl>
	#[cfg_attr(feature = "serde", serde(rename = "paymentUrl"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#payment_url: Vec<PaymentUrlProperty>,
	/// <https://schema.org/seller>
	#[cfg_attr(feature = "serde", serde(rename = "seller"))]
	#[cfg_attr(
		feature = "serde",
		serde(skip_serializing_if = "Vec::is_empty", default)
	)]
	#[cfg_attr(
		feature = "serde",
		serde_as(as = "::serde_with::OneOrMany<::serde_with::Same>")
	)]
	pub r#seller: Vec<SellerProperty>,
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
/// This trait is for properties from <https://schema.org/Order>.
pub trait OrderTrait {
	/// Get <https://schema.org/acceptedOffer> from [`Self`] as borrowed slice.
	fn r#accepted_offer(&self) -> &[AcceptedOfferProperty];
	/// Get <https://schema.org/billingAddress> from [`Self`] as borrowed slice.
	fn r#billing_address(&self) -> &[BillingAddressProperty];
	/// Get <https://schema.org/broker> from [`Self`] as borrowed slice.
	fn r#broker(&self) -> &[BrokerProperty];
	/// Get <https://schema.org/confirmationNumber> from [`Self`] as borrowed slice.
	fn r#confirmation_number(&self) -> &[ConfirmationNumberProperty];
	/// Get <https://schema.org/customer> from [`Self`] as borrowed slice.
	fn r#customer(&self) -> &[CustomerProperty];
	/// Get <https://schema.org/discount> from [`Self`] as borrowed slice.
	fn r#discount(&self) -> &[DiscountProperty];
	/// Get <https://schema.org/discountCode> from [`Self`] as borrowed slice.
	fn r#discount_code(&self) -> &[DiscountCodeProperty];
	/// Get <https://schema.org/discountCurrency> from [`Self`] as borrowed slice.
	fn r#discount_currency(&self) -> &[DiscountCurrencyProperty];
	/// Get <https://schema.org/isGift> from [`Self`] as borrowed slice.
	fn r#is_gift(&self) -> &[IsGiftProperty];
	/// Get <https://schema.org/merchant> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
	fn r#merchant(&self) -> &[MerchantProperty];
	/// Get <https://schema.org/orderDate> from [`Self`] as borrowed slice.
	fn r#order_date(&self) -> &[OrderDateProperty];
	/// Get <https://schema.org/orderDelivery> from [`Self`] as borrowed slice.
	fn r#order_delivery(&self) -> &[OrderDeliveryProperty];
	/// Get <https://schema.org/orderNumber> from [`Self`] as borrowed slice.
	fn r#order_number(&self) -> &[OrderNumberProperty];
	/// Get <https://schema.org/orderStatus> from [`Self`] as borrowed slice.
	fn r#order_status(&self) -> &[OrderStatusProperty];
	/// Get <https://schema.org/orderedItem> from [`Self`] as borrowed slice.
	fn r#ordered_item(&self) -> &[OrderedItemProperty];
	/// Get <https://schema.org/partOfInvoice> from [`Self`] as borrowed slice.
	fn r#part_of_invoice(&self) -> &[PartOfInvoiceProperty];
	/// Get <https://schema.org/paymentDue> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	fn r#payment_due(&self) -> &[PaymentDueProperty];
	/// Get <https://schema.org/paymentDueDate> from [`Self`] as borrowed slice.
	fn r#payment_due_date(&self) -> &[PaymentDueDateProperty];
	/// Get <https://schema.org/paymentMethod> from [`Self`] as borrowed slice.
	fn r#payment_method(&self) -> &[PaymentMethodProperty];
	/// Get <https://schema.org/paymentMethodId> from [`Self`] as borrowed slice.
	fn r#payment_method_id(&self) -> &[PaymentMethodIdProperty];
	/// Get <https://schema.org/paymentUrl> from [`Self`] as borrowed slice.
	fn r#payment_url(&self) -> &[PaymentUrlProperty];
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn r#seller(&self) -> &[SellerProperty];
}
impl OrderTrait for Order {
	fn r#accepted_offer(&self) -> &[AcceptedOfferProperty] {
		self.r#accepted_offer.as_slice()
	}
	fn r#billing_address(&self) -> &[BillingAddressProperty] {
		self.r#billing_address.as_slice()
	}
	fn r#broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn r#confirmation_number(&self) -> &[ConfirmationNumberProperty] {
		self.r#confirmation_number.as_slice()
	}
	fn r#customer(&self) -> &[CustomerProperty] {
		self.r#customer.as_slice()
	}
	fn r#discount(&self) -> &[DiscountProperty] {
		self.r#discount.as_slice()
	}
	fn r#discount_code(&self) -> &[DiscountCodeProperty] {
		self.r#discount_code.as_slice()
	}
	fn r#discount_currency(&self) -> &[DiscountCurrencyProperty] {
		self.r#discount_currency.as_slice()
	}
	fn r#is_gift(&self) -> &[IsGiftProperty] {
		self.r#is_gift.as_slice()
	}
	fn r#merchant(&self) -> &[MerchantProperty] {
		self.r#merchant.as_slice()
	}
	fn r#order_date(&self) -> &[OrderDateProperty] {
		self.r#order_date.as_slice()
	}
	fn r#order_delivery(&self) -> &[OrderDeliveryProperty] {
		self.r#order_delivery.as_slice()
	}
	fn r#order_number(&self) -> &[OrderNumberProperty] {
		self.r#order_number.as_slice()
	}
	fn r#order_status(&self) -> &[OrderStatusProperty] {
		self.r#order_status.as_slice()
	}
	fn r#ordered_item(&self) -> &[OrderedItemProperty] {
		self.r#ordered_item.as_slice()
	}
	fn r#part_of_invoice(&self) -> &[PartOfInvoiceProperty] {
		self.r#part_of_invoice.as_slice()
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
	fn r#payment_url(&self) -> &[PaymentUrlProperty] {
		self.r#payment_url.as_slice()
	}
	fn r#seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
}
impl ThingTrait for Order {
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
