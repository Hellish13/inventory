use serde::{Deserialize, Serialize};
use rusqlite::types::Value;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manager {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price: f64,
    pub quantity: i32,
    pub sku: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOffRequest {
    pub id: Option<i64>,
    pub manager_id: i64,
    pub admin_id: Option<i64>,
    pub request_date: String,
    pub approval_date: Option<String>,
    pub status: RequestStatus,
    pub reason: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOffItem {
    pub id: Option<i64>,
    pub request_id: i64,
    pub product_id: i64,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStatus {
    Pending,
    Approved,
    Rejected,
}

impl fmt::Display for RequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestStatus::Pending => write!(f, "pending"),
            RequestStatus::Approved => write!(f, "approved"),
            RequestStatus::Rejected => write!(f, "rejected"),
        }
    }
}

impl From<String> for RequestStatus {
    fn from(s: String) -> Self {
        match s.as_str() {
            "approved" => RequestStatus::Approved,
            "rejected" => RequestStatus::Rejected,
            _ => RequestStatus::Pending,
        }
    }
}

impl From<&str> for RequestStatus {
    fn from(s: &str) -> Self {
        match s {
            "approved" => RequestStatus::Approved,
            "rejected" => RequestStatus::Rejected,
            _ => RequestStatus::Pending,
        }
    }
}

impl From<Value> for RequestStatus {
    fn from(value: Value) -> Self {
        match value {
            Value::Text(s) => s.into(),
            _ => RequestStatus::Pending,
        }
    }
}

// DTO для связанных данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOffRequestWithDetails {
    pub request: WriteOffRequest,
    pub manager: Manager,
    pub admin: Option<Admin>,
    pub items: Vec<WriteOffItemWithProduct>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOffItemWithProduct {
    pub item: WriteOffItem,
    pub product: Product,
}