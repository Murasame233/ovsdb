use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "PortGroupProxy", into = "PortGroupProxy")]
pub struct PortGroup {
    acls: Vec<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    name: String,
    ports: Vec<ovsdb::protocol::Uuid>,
}
impl Entity for PortGroup {
    fn table_name() -> &'static str {
        "Port_Group"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PortGroupProxy {
    acls: ovsdb::protocol::UuidSet,
    external_ids: ovsdb::protocol::Map<String, String>,
    name: String,
    ports: ovsdb::protocol::UuidSet,
}
impl From<PortGroupProxy> for PortGroup {
    fn from(other: PortGroupProxy) -> Self {
        Self {
            acls: other.acls.into(),
            external_ids: other.external_ids.into(),
            name: other.name,
            ports: other.ports.into(),
        }
    }
}
impl From<PortGroup> for PortGroupProxy {
    fn from(other: PortGroup) -> Self {
        Self {
            acls: other.acls.into(),
            external_ids: other.external_ids.into(),
            name: other.name,
            ports: other.ports.into(),
        }
    }
}
