use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "NbGlobalProxy", into = "NbGlobalProxy")]
pub struct NbGlobal {
    connections: Vec<ovsdb::protocol::Uuid>,
    external_ids: std::collections::BTreeMap<String, String>,
    hv_cfg: i64,
    hv_cfg_timestamp: i64,
    ipsec: bool,
    name: String,
    nb_cfg: i64,
    nb_cfg_timestamp: i64,
    options: std::collections::BTreeMap<String, String>,
    sb_cfg: i64,
    sb_cfg_timestamp: i64,
    ssl: Option<ovsdb::protocol::Uuid>,
}
impl Entity for NbGlobal {
    fn table_name() -> &'static str {
        "NB_Global"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NbGlobalProxy {
    connections: ovsdb::protocol::UuidSet,
    external_ids: ovsdb::protocol::Map<String, String>,
    hv_cfg: i64,
    hv_cfg_timestamp: i64,
    ipsec: bool,
    name: String,
    nb_cfg: i64,
    nb_cfg_timestamp: i64,
    options: ovsdb::protocol::Map<String, String>,
    sb_cfg: i64,
    sb_cfg_timestamp: i64,
    ssl: ovsdb::protocol::Optional<ovsdb::protocol::Uuid>,
}
impl From<NbGlobalProxy> for NbGlobal {
    fn from(other: NbGlobalProxy) -> Self {
        Self {
            connections: other.connections.into(),
            external_ids: other.external_ids.into(),
            hv_cfg: other.hv_cfg,
            hv_cfg_timestamp: other.hv_cfg_timestamp,
            ipsec: other.ipsec,
            name: other.name,
            nb_cfg: other.nb_cfg,
            nb_cfg_timestamp: other.nb_cfg_timestamp,
            options: other.options.into(),
            sb_cfg: other.sb_cfg,
            sb_cfg_timestamp: other.sb_cfg_timestamp,
            ssl: other.ssl.into(),
        }
    }
}
impl From<NbGlobal> for NbGlobalProxy {
    fn from(other: NbGlobal) -> Self {
        Self {
            connections: other.connections.into(),
            external_ids: other.external_ids.into(),
            hv_cfg: other.hv_cfg,
            hv_cfg_timestamp: other.hv_cfg_timestamp,
            ipsec: other.ipsec,
            name: other.name,
            nb_cfg: other.nb_cfg,
            nb_cfg_timestamp: other.nb_cfg_timestamp,
            options: other.options.into(),
            sb_cfg: other.sb_cfg,
            sb_cfg_timestamp: other.sb_cfg_timestamp,
            ssl: other.ssl.into(),
        }
    }
}
