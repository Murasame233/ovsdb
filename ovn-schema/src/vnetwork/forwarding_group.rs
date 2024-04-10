use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "ForwardingGroupProxy", into = "ForwardingGroupProxy")]
pub struct ForwardingGroup {
    child_port: Vec<String>,
    external_ids: std::collections::BTreeMap<String, String>,
    liveness: bool,
    name: String,
    vip: String,
    vmac: String,
}
impl Entity for ForwardingGroup {
    fn table_name() -> &'static str {
        "Forwarding_Group"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ForwardingGroupProxy {
    child_port: ovsdb::protocol::Set<String>,
    external_ids: ovsdb::protocol::Map<String, String>,
    liveness: bool,
    name: String,
    vip: String,
    vmac: String,
}
impl From<ForwardingGroupProxy> for ForwardingGroup {
    fn from(other: ForwardingGroupProxy) -> Self {
        Self {
            child_port: other.child_port.into(),
            external_ids: other.external_ids.into(),
            liveness: other.liveness,
            name: other.name,
            vip: other.vip,
            vmac: other.vmac,
        }
    }
}
impl From<ForwardingGroup> for ForwardingGroupProxy {
    fn from(other: ForwardingGroup) -> Self {
        Self {
            child_port: other.child_port.into(),
            external_ids: other.external_ids.into(),
            liveness: other.liveness,
            name: other.name,
            vip: other.vip,
            vmac: other.vmac,
        }
    }
}
