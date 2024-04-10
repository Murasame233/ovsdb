use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "StaticMacBindingProxy", into = "StaticMacBindingProxy")]
pub struct StaticMacBinding {
    ip: String,
    logical_port: String,
    mac: String,
    override_dynamic_mac: bool,
}
impl Entity for StaticMacBinding {
    fn table_name() -> &'static str {
        "Static_MAC_Binding"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct StaticMacBindingProxy {
    ip: String,
    logical_port: String,
    mac: String,
    override_dynamic_mac: bool,
}
impl From<StaticMacBindingProxy> for StaticMacBinding {
    fn from(other: StaticMacBindingProxy) -> Self {
        Self {
            ip: other.ip,
            logical_port: other.logical_port,
            mac: other.mac,
            override_dynamic_mac: other.override_dynamic_mac,
        }
    }
}
impl From<StaticMacBinding> for StaticMacBindingProxy {
    fn from(other: StaticMacBinding) -> Self {
        Self {
            ip: other.ip,
            logical_port: other.logical_port,
            mac: other.mac,
            override_dynamic_mac: other.override_dynamic_mac,
        }
    }
}
