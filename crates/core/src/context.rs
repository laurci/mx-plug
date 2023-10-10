use base64::{engine::general_purpose, Engine as _};
use mx_plug_shared::serialization::__serde as serde;
use serde_json;

#[derive(Debug, serde::Deserialize)]
#[serde(crate = "mx_plug_shared::serialization::__serde")]
struct RawCallContext {
    #[serde(rename = "Address")]
    pub address: String,
}

#[derive(Debug)]
pub struct PluginContext {
    pub address: Vec<u8>,
}

pub fn decode_plugin_context_from_str(data: &str) -> PluginContext {
    let raw: RawCallContext = serde_json::from_str(data).unwrap();
    let address = general_purpose::STANDARD.decode(raw.address).unwrap();

    PluginContext { address }
}
