//! Raw-email conversion interfaces. Initial red-test checkpoint, not a converter.
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
pub use process_mining::core::event_data::object_centric::OCEL;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventView { Observed, HeaderAsserted, Both }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission { Synthetic, AuthorisedLocal, Unknown }

/// Caller-supplied bytes and explicit provenance; deliberately not Debug/Serialize.
pub struct RawEmail {
    pub bytes: Vec<u8>,
    pub source_ref: String,
    pub observed_at: DateTime<FixedOffset>,
    pub permission: Permission,
}

/// Secret remains private and is never serialized or formatted.
pub struct Config {
    pub scope: String,
    key: [u8; 32],
    pub view: EventView,
    pub max_message_bytes: usize,
    pub max_total_bytes: usize,
    pub max_messages: usize,
    pub max_parts: usize,
    pub max_depth: usize,
}
impl Config {
    pub fn new(scope: String, key: [u8; 32], view: EventView) -> Self {
        Self { scope, key, view, max_message_bytes: 8 * 1024 * 1024,
            max_total_bytes: 64 * 1024 * 1024, max_messages: 1024,
            max_parts: 1024, max_depth: 16 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Party {
    pub party_id: String,
    pub role: String,
    pub identity_state: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Attachment {
    pub part_id: String,
    pub mime_type: String,
    pub size_bytes: u64,
    pub filename_hash: Option<String>,
    pub content_sha256: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ThreadEvidence {
    pub related_message_id: String,
    pub basis: String,
    pub relationship_state: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ObservedMessage {
    pub schema_version: String,
    pub source_copy_id: String,
    pub message_candidate_id: String,
    pub source_sha256: String,
    pub source_kind: String,
    pub observed_at: DateTime<FixedOffset>,
    pub header_date: Option<DateTime<FixedOffset>>,
    pub source_received_at: Option<DateTime<FixedOffset>>,
    pub header_message_id_hash: Option<String>,
    pub source_ref: Option<String>,
    pub observation_state: String,
    pub addresses: Vec<Party>,
    pub attachments: Vec<Attachment>,
    pub thread_evidence: Vec<ThreadEvidence>,
    pub parse_warnings: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Receipt {
    pub schema_version: String,
    pub run_id: String,
    pub source_id: String,
    pub source_version: String,
    pub source_rights_status: String,
    pub input_count: usize,
    pub parsed_count: usize,
    pub rejected_count: usize,
    pub duplicate_copy_count: usize,
    pub output_count: usize,
    pub completeness: String,
    pub config_sha256: String,
    pub producer_version: String,
    pub result_hash: String,
    pub notes: Vec<String>,
}
#[derive(Debug, Serialize)]
pub struct Conversion {
    pub ocel: OCEL,
    pub messages: Vec<ObservedMessage>,
    pub receipt: Receipt,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(pub &'static str);
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str(self.0) }
}
impl std::error::Error for Error {}

/// Frozen interface for the first red/green vertical slice.
pub fn convert(_inputs: &[RawEmail], config: &Config) -> Result<Conversion, Error> {
    let _ = config.key;
    Err(Error("NOT_IMPLEMENTED"))
}
