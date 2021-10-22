use crate::{irmaclient::SessionToken, util::TranslatedString};

use serde::{Deserialize, Serialize};

/// Status of an disclosed attribute
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AttributeStatus {
    #[serde(rename = "PRESENT")]
    Present,
    #[serde(rename = "EXTRA")]
    Extra,
    #[serde(rename = "NULL")]
    Null,
}

/// Status of an IRMA proof
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProofStatus {
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "INVALID_TIMESTAMP")]
    InvalidTimestamp,
    #[serde(rename = "UNMATCHED_REQUEST")]
    UnmatchedRequest,
    #[serde(rename = "MISSING_ATTRIBUTES")]
    MissingAttributes,
    #[serde(rename = "EXPIRED")]
    Expired,
}

/// Status of an IRMA session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SessionStatus {
    #[serde(rename = "INITIALIZED")]
    Initialized,
    #[serde(rename = "PAIRING")]
    Pairing,
    #[serde(rename = "CONNECTED")]
    Connected,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "TIMEOUT")]
    Timeout,
}

/// Type of an IRMA session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SessionType {
    #[serde(rename = "disclosing")]
    Disclosing,
    #[serde(rename = "signing")]
    Signing,
    #[serde(rename = "issuing")]
    Issuing,
}

/// A disclosed attribute
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DisclosedAttribute {
    /// The value of the attribute as encoded in the credential
    #[serde(rename = "rawvalue", skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<String>,
    /// A representation of the value that can be used for displaying in UI's of various languages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TranslatedString>,
    /// Identifier of the disclosed attribute
    pub identifier: String,
    /// Additional information on the role of the disclosed attribute in the complete session result
    pub status: AttributeStatus,
}

/// Results of a session
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionResult {
    /// Token of the session
    pub token: SessionToken,
    #[serde(rename = "type")]
    pub sessiontype: SessionType,
    pub status: SessionStatus,
    #[serde(rename = "proofStatus", skip_serializing_if = "Option::is_none")]
    pub proof_status: Option<ProofStatus>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disclosed: Vec<Vec<DisclosedAttribute>>,
}
