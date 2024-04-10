use serde::{Deserialize, Serialize};
use ovsdb::Entity;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "SslProxy", into = "SslProxy")]
pub struct Ssl {
    bootstrap_ca_cert: bool,
    ca_cert: String,
    certificate: String,
    external_ids: std::collections::BTreeMap<String, String>,
    private_key: String,
    ssl_ciphers: String,
    ssl_protocols: String,
}
impl Entity for Ssl {
    fn table_name() -> &'static str {
        "SSL"
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SslProxy {
    bootstrap_ca_cert: bool,
    ca_cert: String,
    certificate: String,
    external_ids: ovsdb::protocol::Map<String, String>,
    private_key: String,
    ssl_ciphers: String,
    ssl_protocols: String,
}
impl From<SslProxy> for Ssl {
    fn from(other: SslProxy) -> Self {
        Self {
            bootstrap_ca_cert: other.bootstrap_ca_cert,
            ca_cert: other.ca_cert,
            certificate: other.certificate,
            external_ids: other.external_ids.into(),
            private_key: other.private_key,
            ssl_ciphers: other.ssl_ciphers,
            ssl_protocols: other.ssl_protocols,
        }
    }
}
impl From<Ssl> for SslProxy {
    fn from(other: Ssl) -> Self {
        Self {
            bootstrap_ca_cert: other.bootstrap_ca_cert,
            ca_cert: other.ca_cert,
            certificate: other.certificate,
            external_ids: other.external_ids.into(),
            private_key: other.private_key,
            ssl_ciphers: other.ssl_ciphers,
            ssl_protocols: other.ssl_protocols,
        }
    }
}
