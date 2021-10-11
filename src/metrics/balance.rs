use lazy_static::lazy_static;
use prometheus::{Gauge, Registry};

use crate::api::{self, Balance};

lazy_static! {
    static ref ACCOUNT_BALANCE: Gauge =
        Gauge::new("account_balance", "The current Account-Balance").unwrap();
    static ref MONTH_TO_DATE_BALANCE: Gauge =
        Gauge::new("month_to_date_balance", "The current Balance with the Usage of the Month already subtracted from the Account-Balance").unwrap();
    static ref MONTH_TO_DATE_USAGE: Gauge =
        Gauge::new("month_to_date_usage", "The current Usage for this Month").unwrap();
}

pub fn register_metrics(registry: &Registry) {
    registry
        .register(Box::new(ACCOUNT_BALANCE.clone()))
        .unwrap();
    registry
        .register(Box::new(MONTH_TO_DATE_BALANCE.clone()))
        .unwrap();
    registry
        .register(Box::new(MONTH_TO_DATE_USAGE.clone()))
        .unwrap();
}

#[tracing::instrument(skip(client))]
pub async fn update(client: &api::API) {
    let balance = match client.load_resource::<Balance>().await {
        Ok(b) => b,
        Err(e) => {
            tracing::error!("Loading-Balance: {:?}", e);
            return;
        }
    };

    if let Ok(acc_balance) = balance.account_balance.parse() {
        ACCOUNT_BALANCE.set(acc_balance);
    }
    if let Ok(month_to_date_balance) = balance.month_to_date_balance.parse() {
        MONTH_TO_DATE_BALANCE.set(month_to_date_balance);
    }
    if let Ok(month_to_date_usage) = balance.month_to_date_usage.parse() {
        MONTH_TO_DATE_USAGE.set(month_to_date_usage);
    }
}
