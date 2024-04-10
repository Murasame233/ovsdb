use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "DnsProxy", into = "DnsProxy")]
pub struct Dns {
    external_ids: std::collections::BTreeMap<String, String>,
    options: std::collections::BTreeMap<String, String>,
    records: std::collections::BTreeMap<String, String>,
}
impl Entity for Dns {
    fn table_name() -> &'static str {
        "DNS"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DnsProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    options: ovsdb::protocol::Map<String, String>,
    records: ovsdb::protocol::Map<String, String>,
}
impl From<DnsProxy> for Dns {
    fn from(other: DnsProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            options: other.options.into(),
            records: other.records.into(),
        }
    }
}
impl From<Dns> for DnsProxy {
    fn from(other: Dns) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            options: other.options.into(),
            records: other.records.into(),
        }
    }
}
