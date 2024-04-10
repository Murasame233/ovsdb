use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "GatewayChassisProxy", into = "GatewayChassisProxy")]
pub struct GatewayChassis {
    chassis_name: String,
    external_ids: std::collections::BTreeMap<String, String>,
    name: String,
    options: std::collections::BTreeMap<String, String>,
    priority: i64,
}
impl Entity for GatewayChassis {
    fn table_name() -> &'static str {
        "Gateway_Chassis"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GatewayChassisProxy {
    chassis_name: String,
    external_ids: ovsdb::protocol::Map<String, String>,
    name: String,
    options: ovsdb::protocol::Map<String, String>,
    priority: i64,
}
impl From<GatewayChassisProxy> for GatewayChassis {
    fn from(other: GatewayChassisProxy) -> Self {
        Self {
            chassis_name: other.chassis_name,
            external_ids: other.external_ids.into(),
            name: other.name,
            options: other.options.into(),
            priority: other.priority,
        }
    }
}
impl From<GatewayChassis> for GatewayChassisProxy {
    fn from(other: GatewayChassis) -> Self {
        Self {
            chassis_name: other.chassis_name,
            external_ids: other.external_ids.into(),
            name: other.name,
            options: other.options.into(),
            priority: other.priority,
        }
    }
}
