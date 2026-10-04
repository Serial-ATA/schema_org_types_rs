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
	fn get_accepted_offer(&self) -> &[AcceptedOfferProperty];
	/// Take <https://schema.org/acceptedOffer> from [`Self`] as owned vector.
	fn take_accepted_offer(&mut self) -> Vec<AcceptedOfferProperty>;
	/// Get <https://schema.org/billingAddress> from [`Self`] as borrowed slice.
	fn get_billing_address(&self) -> &[BillingAddressProperty];
	/// Take <https://schema.org/billingAddress> from [`Self`] as owned vector.
	fn take_billing_address(&mut self) -> Vec<BillingAddressProperty>;
	/// Get <https://schema.org/broker> from [`Self`] as borrowed slice.
	fn get_broker(&self) -> &[BrokerProperty];
	/// Take <https://schema.org/broker> from [`Self`] as owned vector.
	fn take_broker(&mut self) -> Vec<BrokerProperty>;
	/// Get <https://schema.org/confirmationNumber> from [`Self`] as borrowed slice.
	fn get_confirmation_number(&self) -> &[ConfirmationNumberProperty];
	/// Take <https://schema.org/confirmationNumber> from [`Self`] as owned vector.
	fn take_confirmation_number(&mut self) -> Vec<ConfirmationNumberProperty>;
	/// Get <https://schema.org/customer> from [`Self`] as borrowed slice.
	fn get_customer(&self) -> &[CustomerProperty];
	/// Take <https://schema.org/customer> from [`Self`] as owned vector.
	fn take_customer(&mut self) -> Vec<CustomerProperty>;
	/// Get <https://schema.org/discount> from [`Self`] as borrowed slice.
	fn get_discount(&self) -> &[DiscountProperty];
	/// Take <https://schema.org/discount> from [`Self`] as owned vector.
	fn take_discount(&mut self) -> Vec<DiscountProperty>;
	/// Get <https://schema.org/discountCode> from [`Self`] as borrowed slice.
	fn get_discount_code(&self) -> &[DiscountCodeProperty];
	/// Take <https://schema.org/discountCode> from [`Self`] as owned vector.
	fn take_discount_code(&mut self) -> Vec<DiscountCodeProperty>;
	/// Get <https://schema.org/discountCurrency> from [`Self`] as borrowed slice.
	fn get_discount_currency(&self) -> &[DiscountCurrencyProperty];
	/// Take <https://schema.org/discountCurrency> from [`Self`] as owned vector.
	fn take_discount_currency(&mut self) -> Vec<DiscountCurrencyProperty>;
	/// Get <https://schema.org/isGift> from [`Self`] as borrowed slice.
	fn get_is_gift(&self) -> &[IsGiftProperty];
	/// Take <https://schema.org/isGift> from [`Self`] as owned vector.
	fn take_is_gift(&mut self) -> Vec<IsGiftProperty>;
	/// Get <https://schema.org/merchant> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
	fn get_merchant(&self) -> &[MerchantProperty];
	/// Take <https://schema.org/merchant> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/seller>."]
	fn take_merchant(&mut self) -> Vec<MerchantProperty>;
	/// Get <https://schema.org/orderDate> from [`Self`] as borrowed slice.
	fn get_order_date(&self) -> &[OrderDateProperty];
	/// Take <https://schema.org/orderDate> from [`Self`] as owned vector.
	fn take_order_date(&mut self) -> Vec<OrderDateProperty>;
	/// Get <https://schema.org/orderDelivery> from [`Self`] as borrowed slice.
	fn get_order_delivery(&self) -> &[OrderDeliveryProperty];
	/// Take <https://schema.org/orderDelivery> from [`Self`] as owned vector.
	fn take_order_delivery(&mut self) -> Vec<OrderDeliveryProperty>;
	/// Get <https://schema.org/orderNumber> from [`Self`] as borrowed slice.
	fn get_order_number(&self) -> &[OrderNumberProperty];
	/// Take <https://schema.org/orderNumber> from [`Self`] as owned vector.
	fn take_order_number(&mut self) -> Vec<OrderNumberProperty>;
	/// Get <https://schema.org/orderStatus> from [`Self`] as borrowed slice.
	fn get_order_status(&self) -> &[OrderStatusProperty];
	/// Take <https://schema.org/orderStatus> from [`Self`] as owned vector.
	fn take_order_status(&mut self) -> Vec<OrderStatusProperty>;
	/// Get <https://schema.org/orderedItem> from [`Self`] as borrowed slice.
	fn get_ordered_item(&self) -> &[OrderedItemProperty];
	/// Take <https://schema.org/orderedItem> from [`Self`] as owned vector.
	fn take_ordered_item(&mut self) -> Vec<OrderedItemProperty>;
	/// Get <https://schema.org/partOfInvoice> from [`Self`] as borrowed slice.
	fn get_part_of_invoice(&self) -> &[PartOfInvoiceProperty];
	/// Take <https://schema.org/partOfInvoice> from [`Self`] as owned vector.
	fn take_part_of_invoice(&mut self) -> Vec<PartOfInvoiceProperty>;
	/// Get <https://schema.org/paymentDue> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	fn get_payment_due(&self) -> &[PaymentDueProperty];
	/// Take <https://schema.org/paymentDue> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/paymentDueDate>."]
	fn take_payment_due(&mut self) -> Vec<PaymentDueProperty>;
	/// Get <https://schema.org/paymentDueDate> from [`Self`] as borrowed slice.
	fn get_payment_due_date(&self) -> &[PaymentDueDateProperty];
	/// Take <https://schema.org/paymentDueDate> from [`Self`] as owned vector.
	fn take_payment_due_date(&mut self) -> Vec<PaymentDueDateProperty>;
	/// Get <https://schema.org/paymentMethod> from [`Self`] as borrowed slice.
	fn get_payment_method(&self) -> &[PaymentMethodProperty];
	/// Take <https://schema.org/paymentMethod> from [`Self`] as owned vector.
	fn take_payment_method(&mut self) -> Vec<PaymentMethodProperty>;
	/// Get <https://schema.org/paymentMethodId> from [`Self`] as borrowed slice.
	fn get_payment_method_id(&self) -> &[PaymentMethodIdProperty];
	/// Take <https://schema.org/paymentMethodId> from [`Self`] as owned vector.
	fn take_payment_method_id(&mut self) -> Vec<PaymentMethodIdProperty>;
	/// Get <https://schema.org/paymentUrl> from [`Self`] as borrowed slice.
	fn get_payment_url(&self) -> &[PaymentUrlProperty];
	/// Take <https://schema.org/paymentUrl> from [`Self`] as owned vector.
	fn take_payment_url(&mut self) -> Vec<PaymentUrlProperty>;
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn get_seller(&self) -> &[SellerProperty];
	/// Take <https://schema.org/seller> from [`Self`] as owned vector.
	fn take_seller(&mut self) -> Vec<SellerProperty>;
}
impl OrderTrait for Order {
	fn get_accepted_offer(&self) -> &[AcceptedOfferProperty] {
		self.r#accepted_offer.as_slice()
	}
	fn take_accepted_offer(&mut self) -> Vec<AcceptedOfferProperty> {
		std::mem::take(&mut self.r#accepted_offer)
	}
	fn get_billing_address(&self) -> &[BillingAddressProperty] {
		self.r#billing_address.as_slice()
	}
	fn take_billing_address(&mut self) -> Vec<BillingAddressProperty> {
		std::mem::take(&mut self.r#billing_address)
	}
	fn get_broker(&self) -> &[BrokerProperty] {
		self.r#broker.as_slice()
	}
	fn take_broker(&mut self) -> Vec<BrokerProperty> {
		std::mem::take(&mut self.r#broker)
	}
	fn get_confirmation_number(&self) -> &[ConfirmationNumberProperty] {
		self.r#confirmation_number.as_slice()
	}
	fn take_confirmation_number(&mut self) -> Vec<ConfirmationNumberProperty> {
		std::mem::take(&mut self.r#confirmation_number)
	}
	fn get_customer(&self) -> &[CustomerProperty] {
		self.r#customer.as_slice()
	}
	fn take_customer(&mut self) -> Vec<CustomerProperty> {
		std::mem::take(&mut self.r#customer)
	}
	fn get_discount(&self) -> &[DiscountProperty] {
		self.r#discount.as_slice()
	}
	fn take_discount(&mut self) -> Vec<DiscountProperty> {
		std::mem::take(&mut self.r#discount)
	}
	fn get_discount_code(&self) -> &[DiscountCodeProperty] {
		self.r#discount_code.as_slice()
	}
	fn take_discount_code(&mut self) -> Vec<DiscountCodeProperty> {
		std::mem::take(&mut self.r#discount_code)
	}
	fn get_discount_currency(&self) -> &[DiscountCurrencyProperty] {
		self.r#discount_currency.as_slice()
	}
	fn take_discount_currency(&mut self) -> Vec<DiscountCurrencyProperty> {
		std::mem::take(&mut self.r#discount_currency)
	}
	fn get_is_gift(&self) -> &[IsGiftProperty] {
		self.r#is_gift.as_slice()
	}
	fn take_is_gift(&mut self) -> Vec<IsGiftProperty> {
		std::mem::take(&mut self.r#is_gift)
	}
	fn get_merchant(&self) -> &[MerchantProperty] {
		self.r#merchant.as_slice()
	}
	fn take_merchant(&mut self) -> Vec<MerchantProperty> {
		std::mem::take(&mut self.r#merchant)
	}
	fn get_order_date(&self) -> &[OrderDateProperty] {
		self.r#order_date.as_slice()
	}
	fn take_order_date(&mut self) -> Vec<OrderDateProperty> {
		std::mem::take(&mut self.r#order_date)
	}
	fn get_order_delivery(&self) -> &[OrderDeliveryProperty] {
		self.r#order_delivery.as_slice()
	}
	fn take_order_delivery(&mut self) -> Vec<OrderDeliveryProperty> {
		std::mem::take(&mut self.r#order_delivery)
	}
	fn get_order_number(&self) -> &[OrderNumberProperty] {
		self.r#order_number.as_slice()
	}
	fn take_order_number(&mut self) -> Vec<OrderNumberProperty> {
		std::mem::take(&mut self.r#order_number)
	}
	fn get_order_status(&self) -> &[OrderStatusProperty] {
		self.r#order_status.as_slice()
	}
	fn take_order_status(&mut self) -> Vec<OrderStatusProperty> {
		std::mem::take(&mut self.r#order_status)
	}
	fn get_ordered_item(&self) -> &[OrderedItemProperty] {
		self.r#ordered_item.as_slice()
	}
	fn take_ordered_item(&mut self) -> Vec<OrderedItemProperty> {
		std::mem::take(&mut self.r#ordered_item)
	}
	fn get_part_of_invoice(&self) -> &[PartOfInvoiceProperty] {
		self.r#part_of_invoice.as_slice()
	}
	fn take_part_of_invoice(&mut self) -> Vec<PartOfInvoiceProperty> {
		std::mem::take(&mut self.r#part_of_invoice)
	}
	fn get_payment_due(&self) -> &[PaymentDueProperty] {
		self.r#payment_due.as_slice()
	}
	fn take_payment_due(&mut self) -> Vec<PaymentDueProperty> {
		std::mem::take(&mut self.r#payment_due)
	}
	fn get_payment_due_date(&self) -> &[PaymentDueDateProperty] {
		self.r#payment_due_date.as_slice()
	}
	fn take_payment_due_date(&mut self) -> Vec<PaymentDueDateProperty> {
		std::mem::take(&mut self.r#payment_due_date)
	}
	fn get_payment_method(&self) -> &[PaymentMethodProperty] {
		self.r#payment_method.as_slice()
	}
	fn take_payment_method(&mut self) -> Vec<PaymentMethodProperty> {
		std::mem::take(&mut self.r#payment_method)
	}
	fn get_payment_method_id(&self) -> &[PaymentMethodIdProperty] {
		self.r#payment_method_id.as_slice()
	}
	fn take_payment_method_id(&mut self) -> Vec<PaymentMethodIdProperty> {
		std::mem::take(&mut self.r#payment_method_id)
	}
	fn get_payment_url(&self) -> &[PaymentUrlProperty] {
		self.r#payment_url.as_slice()
	}
	fn take_payment_url(&mut self) -> Vec<PaymentUrlProperty> {
		std::mem::take(&mut self.r#payment_url)
	}
	fn get_seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn take_seller(&mut self) -> Vec<SellerProperty> {
		std::mem::take(&mut self.r#seller)
	}
}
impl ThingTrait for Order {
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
