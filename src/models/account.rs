use crate::models::{AssetKind, Currency, Direction, Request};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone, Default)]
pub struct GetPositionsRequest {
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<AssetKind>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetPositionsResponse {
    pub average_price: f64,
    pub average_price_usd: Option<f64>,
    pub delta: f64,
    pub direction: Direction,
    pub estimated_liquidation_price: Option<f64>,
    pub floating_profit_loss: f64,
    pub floating_profit_loss_usd: Option<f64>,
    pub index_price: f64,
    pub initial_margin: f64,
    pub instrument_name: String,
    pub kind: AssetKind,
    pub maintenance_margin: f64,
    pub mark_price: f64,
    pub open_orders_margin: f64,
    pub realized_profit_loss: f64,
    pub settlement_price: f64,
    pub size: f64,
    pub size_currency: Option<f64>,
    pub total_profit_loss: f64,
}

impl Request for GetPositionsRequest {
    const METHOD: &'static str = "private/get_positions";
    type Response = Vec<GetPositionsResponse>;
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct GetAccountSummaryRequest {
    pub currency: Currency,
    pub extended: bool,
}
impl GetAccountSummaryRequest {
    pub fn abridged(currency: Currency) -> Self {
        Self {
            currency,
            ..Default::default()
        }
    }
    pub fn extended(currency: Currency) -> Self {
        Self {
            currency,
            extended: true,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAccountSummaryResponse {
    pub id: Option<u64>,
    pub system_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub tfa_enabled: Option<bool>,
    pub portfolio_margin_enabled: Option<bool>,
    pub deposit_address: String,
    pub currency: Currency,
    pub r#type: Option<String>,
    pub session_funding: f64,

    pub maintenance_margin: f64,
    pub projected_initial_margin: Option<f64>,
    pub projected_maintenance_margin: Option<f64>,
    pub initial_margin: f64,
    pub margin_balance: f64,
    pub balance: f64,
    pub equity: f64,
    pub available_withdrawal_funds: f64,
    pub available_funds: f64,

    pub futures_session_upl: f64,
    pub futures_session_rpl: f64,
    pub futures_pl: f64,

    pub options_gamma: f64,
    pub options_vega: f64,
    pub options_theta: f64,
    pub options_delta: f64,
    pub options_session_upl: f64,
    pub options_session_rpl: f64,
    pub options_pl: f64,

    pub delta_total: f64,

    pub session_upl: f64,
    pub session_rpl: f64,
    pub total_pl: f64,
}

impl Request for GetAccountSummaryRequest {
    const METHOD: &'static str = "private/get_account_summary";
    type Response = GetAccountSummaryResponse;
}

#[derive(Serialize)]
pub struct GetSubaccountsRequest {
    pub with_portfolio: bool,
}

impl GetSubaccountsRequest {
    pub fn new() -> Self {
        Self {
            with_portfolio: false,
        }
    }

    pub fn with_portfolio() -> Self {
        Self {
            with_portfolio: true,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Portfolio {
    pub available_funds: f64,
    pub available_withdrawal_funds: f64,
    pub balance: f64,
    pub currency: Currency,
    pub equity: f64,
    pub initial_margin: f64,
    pub maintenance_margin: f64,
    pub margin_balance: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetSubaccountsResponse {
    pub email: String,
    pub id: u64,
    pub is_password: bool,
    pub login_enabled: bool,
    pub not_confirmed_email: Option<String>,
    pub portfolio: HashMap<Currency, Portfolio>,
    pub receive_notifications: bool,
    pub system_name: String,
    pub tfa_enabled: bool,
    pub r#type: String,
    pub username: String,
}

impl Request for GetSubaccountsRequest {
    const METHOD: &'static str = "private/get_subaccounts";
    type Response = Vec<GetSubaccountsResponse>;
}