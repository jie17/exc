use exc_core::Str;
use serde::Serialize;

/// Earn Offers.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarnOffers {
    /// Product ID (Optional).
    /// Unique identifier for the product.
    product_id: Option<String>,
    /// Protocol Type (Optional).
    /// Describes the type of earning mechanism, such as 'staking: simple earn fixed' or 'defi: on-chain earn'.
    protocol_type: Option<String>,
    /// Investment Currency (Optional).
    /// Currency used for the investment, e.g., 'BTC'.
    ccy: Option<String>,
}

/// Earn Active Orders.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarnActiveOrders {
    /// Product ID (Optional).
    /// Represents the unique identifier for the product.
    product_id: Option<String>,
    /// Protocol Type (Optional).
    /// Describes the type of protocol, e.g., 'staking: simple earn fixed' or 'defi: on-chain earn'.
    protocol_type: Option<String>,
    /// Investment Currency (Optional).
    /// E.g., 'BTC', representing the currency used for the investment.
    ccy: Option<String>,
    /// Order State (Optional).
    /// Represents the current state of the order.
    state: Option<String>,
}

/// Investment Data.
#[derive(Debug, Serialize, Clone)]
pub struct InvestmentData {
    /// Investment currency, e.g., BTC.
    ccy: String,
    /// Investment amount.
    amt: String,
}

/// Earn Purchase.
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EarnPurchase {
    /// Product ID.
    pub(crate) product_id: Str,
    /// Investment data.
    #[serde(rename = "investData")]
    invest_data: Vec<InvestmentData>,
    /// Investment term.
    /// Must be specified for fixed-term product.
    term: Option<String>,
    /// Order tag.
    /// A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    tag: Option<String>,
}

/// Earn redeem.
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct EarnRedeem {
    /// Order ID.
    pub(crate) ord_id: Str,
    /// Protocol type.
    pub(crate) protocol_type: Str,
}
