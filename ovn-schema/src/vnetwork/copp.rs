use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "CoppProxy", into = "CoppProxy")]
pub struct Copp {
    external_ids: std::collections::BTreeMap<String, String>,
    meters: std::collections::BTreeMap<String, String>,
    name: String,
}
impl Entity for Copp {
    fn table_name() -> &'static str {
        "Copp"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CoppProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    meters: ovsdb::protocol::Map<String, String>,
    name: String,
}
impl From<CoppProxy> for Copp {
    fn from(other: CoppProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            meters: other.meters.into(),
            name: other.name,
        }
    }
}
impl From<Copp> for CoppProxy {
    fn from(other: Copp) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            meters: other.meters.into(),
            name: other.name,
        }
    }
}
