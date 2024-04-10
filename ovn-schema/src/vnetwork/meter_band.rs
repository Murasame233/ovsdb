use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Action {
    #[serde(rename = "drop")]
    Drop,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "MeterBandProxy", into = "MeterBandProxy")]
pub struct MeterBand {
    action: Action,
    burst_size: i64,
    external_ids: std::collections::BTreeMap<String, String>,
    rate: i64,
}
impl Entity for MeterBand {
    fn table_name() -> &'static str {
        "Meter_Band"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MeterBandProxy {
    action: Action,
    burst_size: i64,
    external_ids: ovsdb::protocol::Map<String, String>,
    rate: i64,
}
impl From<MeterBandProxy> for MeterBand {
    fn from(other: MeterBandProxy) -> Self {
        Self {
            action: other.action.into(),
            burst_size: other.burst_size,
            external_ids: other.external_ids.into(),
            rate: other.rate,
        }
    }
}
impl From<MeterBand> for MeterBandProxy {
    fn from(other: MeterBand) -> Self {
        Self {
            action: other.action.into(),
            burst_size: other.burst_size,
            external_ids: other.external_ids.into(),
            rate: other.rate,
        }
    }
}
