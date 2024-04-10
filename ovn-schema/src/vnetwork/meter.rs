use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Unit {
    #[serde(rename = "kbps")]
    Kbps,
    #[serde(rename = "pktps")]
    Pktps,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "MeterProxy", into = "MeterProxy")]
pub struct Meter {
    bands: Vec<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    fair: Option<bool>,
    name: String,
    unit: Unit,
}
impl Entity for Meter {
    fn table_name() -> &'static str {
        "Meter"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MeterProxy {
    bands: ovsdb::protocol::UuidSet,
    external_ids: ovsdb::protocol::Map<String, String>,
    fair: ovsdb::protocol::Optional<bool>,
    name: String,
    unit: Unit,
}
impl From<MeterProxy> for Meter {
    fn from(other: MeterProxy) -> Self {
        Self {
            bands: other.bands.into(),
            external_ids: other.external_ids.into(),
            fair: other.fair.into(),
            name: other.name,
            unit: other.unit.into(),
        }
    }
}
impl From<Meter> for MeterProxy {
    fn from(other: Meter) -> Self {
        Self {
            bands: other.bands.into(),
            external_ids: other.external_ids.into(),
            fair: other.fair.into(),
            name: other.name,
            unit: other.unit.into(),
        }
    }
}
