use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "ChassisTemplateVarProxy", into = "ChassisTemplateVarProxy")]
pub struct ChassisTemplateVar {
    chassis: String,
    external_ids: std::collections::BTreeMap<String, String>,
    variables: std::collections::BTreeMap<String, String>,
}
impl Entity for ChassisTemplateVar {
    fn table_name() -> &'static str {
        "Chassis_Template_Var"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ChassisTemplateVarProxy {
    chassis: String,
    external_ids: ovsdb::protocol::Map<String, String>,
    variables: ovsdb::protocol::Map<String, String>,
}
impl From<ChassisTemplateVarProxy> for ChassisTemplateVar {
    fn from(other: ChassisTemplateVarProxy) -> Self {
        Self {
            chassis: other.chassis,
            external_ids: other.external_ids.into(),
            variables: other.variables.into(),
        }
    }
}
impl From<ChassisTemplateVar> for ChassisTemplateVarProxy {
    fn from(other: ChassisTemplateVar) -> Self {
        Self {
            chassis: other.chassis,
            external_ids: other.external_ids.into(),
            variables: other.variables.into(),
        }
    }
}
