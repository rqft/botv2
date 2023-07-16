use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UsageOptions {
    pub history: u8,
    pub concurrency: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
    pub billing_period_end: String,
    pub billion_period_start: String,
    pub concurrency: Option<UsageConcurrency>,
    pub daily: HashMap<String, u16>,
    pub daily_for: String,
    pub daily_processed: u16,
    pub daily_requests: u16,
    pub last_usage: f64,
    pub monthly: HashMap<String, u16>,
    pub monthly_limit: u16,
    pub monthly_processed: u16,
    pub monthly_requests: u16,
    pub total_processed: u16,
    pub total_requests: u16,
    pub weekly: HashMap<String, u16>,
    pub weekly_processed: u16,
    pub weekly_requests: u16,
}

#[derive(Serialize, Deserialize)]
pub struct UsageConcurrency {
    max: u16,
    now: u16,
}
