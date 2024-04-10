use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "AddressSetProxy", into = "AddressSetProxy")]
pub struct AddressSet {
    addresses: Vec<String>,
    external_ids: std::collections::BTreeMap<String, String>,
    name: String,
}
impl Entity for AddressSet {
    fn table_name() -> &'static str {
        "Address_Set"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AddressSetProxy {
    addresses: ovsdb::protocol::Set<String>,
    external_ids: ovsdb::protocol::Map<String, String>,
    name: String,
}
impl From<AddressSetProxy> for AddressSet {
    fn from(other: AddressSetProxy) -> Self {
        Self {
            addresses: other.addresses.into(),
            external_ids: other.external_ids.into(),
            name: other.name,
        }
    }
}
impl From<AddressSet> for AddressSetProxy {
    fn from(other: AddressSet) -> Self {
        Self {
            addresses: other.addresses.into(),
            external_ids: other.external_ids.into(),
            name: other.name,
        }
    }
}
