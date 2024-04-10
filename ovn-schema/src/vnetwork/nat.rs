use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Type {
    #[serde(rename = "dnat")]
    Dnat,
    #[serde(rename = "dnat_and_snat")]
    DnatAndSnat,
    #[serde(rename = "snat")]
    Snat,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "NatProxy", into = "NatProxy")]
pub struct Nat {
    allowed_ext_ips: Option<ovsdb::protocol::Uuid>,
    exempted_ext_ips: Option<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    external_ip: String,
    external_mac: Option<String>,
    external_port_range: String,
    gateway_port: Option<ovsdb::protocol::Uuid>,
    logical_ip: String,
    logical_port: Option<String>,
    options: std::collections::BTreeMap<String, String>,
    #[serde(rename = "type")]
    kind: Type,
}
impl Entity for Nat {
    fn table_name() -> &'static str {
        "NAT"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NatProxy {
    allowed_ext_ips: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    exempted_ext_ips: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    external_ids: ovsdb::protocol::Map<String, String>,
    external_ip: String,
    external_mac: ovsdb::protocol::Optional<String>,
    external_port_range: String,
    gateway_port: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
    logical_ip: String,
    logical_port: ovsdb::protocol::Optional<String>,
    options: ovsdb::protocol::Map<String, String>,
    #[serde(rename = "type")]
    kind: Type,
}
impl From<NatProxy> for Nat {
    fn from(other: NatProxy) -> Self {
        Self {
            allowed_ext_ips: other.allowed_ext_ips.into(),
            exempted_ext_ips: other.exempted_ext_ips.into(),
            external_ids: other.external_ids.into(),
            external_ip: other.external_ip,
            external_mac: other.external_mac.into(),
            external_port_range: other.external_port_range,
            gateway_port: other.gateway_port.into(),
            logical_ip: other.logical_ip,
            logical_port: other.logical_port.into(),
            options: other.options.into(),
            kind: other.kind.into(),
        }
    }
}
impl From<Nat> for NatProxy {
    fn from(other: Nat) -> Self {
        Self {
            allowed_ext_ips: other.allowed_ext_ips.into(),
            exempted_ext_ips: other.exempted_ext_ips.into(),
            external_ids: other.external_ids.into(),
            external_ip: other.external_ip,
            external_mac: other.external_mac.into(),
            external_port_range: other.external_port_range,
            gateway_port: other.gateway_port.into(),
            logical_ip: other.logical_ip,
            logical_port: other.logical_port.into(),
            options: other.options.into(),
            kind: other.kind.into(),
        }
    }
}
