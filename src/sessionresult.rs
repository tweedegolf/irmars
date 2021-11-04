use crate::{irmaclient::SessionToken, util::TranslatedString};

use serde::{Deserialize, Serialize};

/// Status of an disclosed attribute
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AttributeStatus {
    Present,
    Extra,
    Null,
}

/// Status of an IRMA proof
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProofStatus {
    Valid,
    Invalid,
    InvalidTimestamp,
    UnmatchedRequest,
    MissingAttributes,
    Expired,
}

/// Status of an IRMA session
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    Initialized,
    Pairing,
    Connected,
    Cancelled,
    Done,
    Timeout,
}

/// Type of an IRMA session
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "lowercase")]
pub enum SessionType {
    Disclosing,
    Signing,
    Issuing,
}

/// A disclosed attribute
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DisclosedAttribute {
    /// The value of the attribute as encoded in the credential
    #[serde(rename = "rawvalue", skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<String>,
    /// A representation of the value that can be used for displaying in UI's of various languages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<TranslatedString>,
    /// Identifier of the disclosed attribute
    #[serde(rename = "id")]
    pub identifier: String,
    /// Additional information on the role of the disclosed attribute in the complete session result
    pub status: AttributeStatus,
}

/// Results of a session
#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct SessionResult {
    /// Token of the session
    pub token: SessionToken,
    #[serde(rename = "type")]
    pub sessiontype: SessionType,
    pub status: SessionStatus,
    #[serde(rename = "proofStatus", skip_serializing_if = "Option::is_none")]
    pub proof_status: Option<ProofStatus>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub disclosed: Vec<Vec<DisclosedAttribute>>,
}

#[cfg(test)]
mod tests {
    use crate::{
        AttributeStatus, DisclosedAttribute, ProofStatus, SessionResult, SessionStatus,
        SessionToken, SessionType, TranslatedString,
    };

    #[test]
    fn test_decode_result() {
        let result = serde_json::from_str::<SessionResult>(
            r#"
        {
            "type" : "disclosing",
            "status" : "DONE",
            "disclosed" : [
              [{
                "status" : "PRESENT",
                "rawvalue" : "yes",
                "id" : "irma-demo.MijnOverheid.ageLower.over18",
                "value" : {
                  "en" : "yes",
                  "nl" : "yes",
                  "" : "yes"
                }
              }]
            ],
            "proofStatus" : "VALID",
            "token" : "ELMExi5iauWYHzbH7gwU"
        }
        "#,
        )
        .unwrap();

        let expected = SessionResult {
            sessiontype: SessionType::Disclosing,
            status: SessionStatus::Done,
            disclosed: vec![vec![DisclosedAttribute {
                status: AttributeStatus::Present,
                raw_value: Some("yes".into()),
                identifier: "irma-demo.MijnOverheid.ageLower.over18".into(),
                value: Some(TranslatedString {
                    en: "yes".into(),
                    nl: "yes".into(),
                }),
            }]],
            proof_status: Some(ProofStatus::Valid),
            token: SessionToken("ELMExi5iauWYHzbH7gwU".into()),
        };

        assert_eq!(result, expected);

        let result = serde_json::from_str::<SessionResult>(
            r#"
        {
            "type" : "disclosing",
            "status" : "CONNECTED",
            "token" : "ELMExi5iauWYHzbH7gwU"
        }
        "#,
        )
        .unwrap();

        let expected = SessionResult {
            sessiontype: SessionType::Disclosing,
            status: SessionStatus::Connected,
            disclosed: vec![],
            proof_status: None,
            token: SessionToken("ELMExi5iauWYHzbH7gwU".into()),
        };

        assert_eq!(result, expected);
    }
}
