pub mod media_session;
pub mod bandwidth;
pub mod origin;
pub mod media_attribute;
pub mod media_description;
pub mod data_transfer_mode;
pub mod payload_type;
pub mod network_type;
pub mod address_type;
pub mod time;
mod sdp_port;
mod transport_protocol;

const RAW_SEPARATOR: &[u8] = b"\n";
const KEY_VALUE_SEPARATOR: &[u8] = b"=";
const COLON_SEPARATOR: &[u8] = b":";
const TRIM: &[u8] = b" ";