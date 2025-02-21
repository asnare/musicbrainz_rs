use crate::date_format;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[serde(default)]
pub struct LifeSpan {
    pub ended: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub begin: Option<NaiveDate>,
    #[serde(default)]
    #[serde(deserialize_with = "date_format::deserialize_opt")]
    pub end: Option<NaiveDate>,
}
