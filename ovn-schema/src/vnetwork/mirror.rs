use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Filter {
    #[serde(rename = "both")]
    Both,
    #[serde(rename = "from-lport")]
    FromLport,
    #[serde(rename = "to-lport")]
    ToLport,
}
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Type {
    #[serde(rename = "erspan")]
    Erspan,
    #[serde(rename = "gre")]
    Gre,
    #[serde(rename = "local")]
    Local,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "MirrorProxy", into = "MirrorProxy")]
pub struct Mirror {
    external_ids: std::collections::BTreeMap<String, String>,
    filter: Filter,
    index: i64,
    name: String,
    sink: String,
    #[serde(rename = "type")]
    kind: Type,
}
impl Entity for Mirror {
    fn table_name() -> &'static str {
        "Mirror"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MirrorProxy {
    external_ids: ovsdb::protocol::Map<String, String>,
    filter: Filter,
    index: i64,
    name: String,
    sink: String,
    #[serde(rename = "type")]
    kind: Type,
}
impl From<MirrorProxy> for Mirror {
    fn from(other: MirrorProxy) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            filter: other.filter.into(),
            index: other.index,
            name: other.name,
            sink: other.sink,
            kind: other.kind.into(),
        }
    }
}
impl From<Mirror> for MirrorProxy {
    fn from(other: Mirror) -> Self {
        Self {
            external_ids: other.external_ids.into(),
            filter: other.filter.into(),
            index: other.index,
            name: other.name,
            sink: other.sink,
            kind: other.kind.into(),
        }
    }
}
