use crate::QueryResponse;
use by_macros::api_model;
use chrono::Datelike;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use by_axum::aide;
#[cfg(feature = "server")]
use schemars::JsonSchema;

#[api_model(base = "/topics/v1", database = skip, iter_type=QueryResponse)]
pub struct Topic {
    #[api_model(summary)]
    pub id: String,
    #[api_model(summary)]
    pub created_at: i64,
    #[api_model(summary)]
    pub updated_at: i64,
    #[api_model(summary)]
    pub deleted_at: Option<u64>,

    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, action = create)]
    pub content: String,

    #[api_model(summary, action = create)]
    pub started_at: i64,
    // The end time of the vote
    #[api_model(summary, action = create)]
    pub ended_at: i64,

    // The number of voters
    #[api_model(summary)]
    pub voters: i64,
    // The number of required votes
    #[api_model(summary)]
    pub requirement: i64,

    // Donation amount
    #[api_model(summary)]
    pub amount: i64,

    #[api_model(summary)]
    pub voted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MyInfo {
    // If my_commitment is 1, it shows 0.01 ETH in the UI
    pub my_commitment: i64,
}

impl TopicSummary {
    pub fn day(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!("{:02}", start.day())
    }

    pub fn month(&self) -> String {
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();

        match start.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sep",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "Unknown",
        }
        .to_string()
    }

    pub fn date(&self) -> String {
        format!("{}/{}", self.month(), self.day())
    }

    pub fn period(&self) -> String {
        // to "12/15 - 1/22"
        let start = chrono::DateTime::from_timestamp(self.started_at, 0)
            .unwrap_or_default()
            .naive_local();
        let end = chrono::DateTime::from_timestamp(self.ended_at, 0)
            .unwrap_or_default()
            .naive_local();

        format!(
            "{:02}/{:02} - {:02}/{:02}",
            start.month(),
            start.day(),
            end.month(),
            end.day()
        )
    }

    pub fn amount(&self) -> String {
        self.amount.to_formatted_string(&Locale::en)
    }
}
