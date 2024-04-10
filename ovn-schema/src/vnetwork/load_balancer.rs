use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Protocol {
    #[serde(rename = "sctp")]
    Sctp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SelectionFields {
    #[serde(rename = "eth_dst")]
    EthDst,
    #[serde(rename = "eth_src")]
    EthSrc,
    #[serde(rename = "ip_dst")]
    IpDst,
    #[serde(rename = "ip_src")]
    IpSrc,
    #[serde(rename = "tp_dst")]
    TpDst,
    #[serde(rename = "tp_src")]
    TpSrc,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "LoadBalancerProxy", into = "LoadBalancerProxy")]
pub struct LoadBalancer {
    external_ids: std::collections::BTreeMap<String, String>,
    health_check: Vec<ovsdb::protocol::Uuid>,
    ip_port_mappings: std::collections::BTreeMap<String, String>,
    name: String,
    options: std::collections::BTreeMap<String, String>,
    protocol: Option<Protocol>,
    selection_fields: Vec<SelectionFields>,
    vips: std::collections::BTreeMap<String, String>,
}
impl Entity for LoadBalancer {
    fn table_name() -> &'static str {
        "Load_Balancer"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LoadBalancerProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    health_check: ovsdb::protocol::UuidSet,
    ip_port_mappings: ovsdb::protocol::Map<String, String>,
    name: String,
    options: ovsdb::protocol::Map<String, String>,
    protocol: ovsdb::protocol::Optional<Protocol>,
    selection_fields: ovsdb::protocol::Set<SelectionFields>,
    vips: ovsdb::protocol::Map<String, String>,
}
impl From<LoadBalancerProxy> for LoadBalancer {
    fn from(other: LoadBalancerProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            health_check: other.health_check.into(),
            ip_port_mappings: other.ip_port_mappings.into(),
            name: other.name,
            options: other.options.into(),
            protocol: other.protocol.into(),
            selection_fields: other.selection_fields.into(),
            vips: other.vips.into(),
        }
    }
}
impl From<LoadBalancer> for LoadBalancerProxy {
    fn from(other: LoadBalancer) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            health_check: other.health_check.into(),
            ip_port_mappings: other.ip_port_mappings.into(),
            name: other.name,
            options: other.options.into(),
            protocol: other.protocol.into(),
            selection_fields: other.selection_fields.into(),
            vips: other.vips.into(),
        }
    }
}
