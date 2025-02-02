use serde::{Deserialize, Serialize};

use casper_types::{Key, ProtocolVersion};

pub use crate::rpcs::common::GlobalStateIdentifier;
use crate::types::{BlockHeader, StoredValue};
#[cfg(doc)]
use crate::BlockIdentifier;

pub(crate) const QUERY_GLOBAL_STATE_METHOD: &str = "query_global_state";

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct QueryGlobalStateParams {
    /// The identifier used for the query.
    state_identifier: GlobalStateIdentifier,
    /// `casper_types::Key` as formatted string.
    key: String,
    /// The path components starting from the key as base.
    path: Vec<String>,
}

impl QueryGlobalStateParams {
    //This clippy lint should be re-enabled once the client is updated to handle multiple different node versions.
    #[allow(dead_code)]
    pub(crate) fn new(
        state_identifier: GlobalStateIdentifier,
        key: Key,
        path: Vec<String>,
    ) -> Self {
        QueryGlobalStateParams {
            state_identifier,
            key: key.to_formatted_string(),
            path,
        }
    }
}

/// The `result` field of a successful JSON-RPC response to a `query_global_state` request.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct QueryGlobalStateResult {
    /// The JSON-RPC server version.
    pub api_version: ProtocolVersion,
    /// The block header if the query was made using a block hash.
    pub block_header: Option<BlockHeader>,
    /// The stored value.
    pub stored_value: StoredValue,
    /// The merkle proof of the value.
    pub merkle_proof: String,
}
