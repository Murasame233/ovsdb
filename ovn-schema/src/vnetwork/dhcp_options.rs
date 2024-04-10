use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "DhcpOptionsProxy", into = "DhcpOptionsProxy")]
pub struct DhcpOptions {
    cidr: String,
    external_ids: std::collections::BTreeMap<String, String>,
    options: std::collections::BTreeMap<String, String>,
}
impl Entity for DhcpOptions {
    fn table_name() -> &'static str {
        "DHCP_Options"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DhcpOptionsProxy {
    cidr: String,
    external_ids: ovsdb::protocol::Map<String, String>,
    options: ovsdb::protocol::Map<String, String>,
}
impl From<DhcpOptionsProxy> for DhcpOptions {
    fn from(other: DhcpOptionsProxy) -> Self {
        Self {
            cidr: other.cidr,
            external_ids: other.external_ids.into(),
            options: other.options.into(),
        }
    }
}
impl From<DhcpOptions> for DhcpOptionsProxy {
    fn from(other: DhcpOptions) -> Self {
        Self {
            cidr: other.cidr,
            external_ids: other.external_ids.into(),
            options: other.options.into(),
        }
    }
}
