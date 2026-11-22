//! Stellar SDK Mock Fixtures for Backend Tests (#1751)
//!
//! Provides reusable mock data builders for Stellar RPC responses, transaction
//! records, and contract events used across backend integration tests.
//!
//! # Usage
//!
//! ```rust
//! use crate::fixtures::stellar_sdk_mock::{
//!     MockRpcEventBuilder, MockTransactionBuilder, MockAccountBuilder
//! };
//!
//! let event = MockRpcEventBuilder::default()
//!     .event_type("waste_registered")
//!     .ledger(12345)
//!     .build();
//!
//! let tx = MockTransactionBuilder::default()
//!     .hash("abc123...")
//!     .status("SUCCESS")
//!     .build();
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ─── Constants ───────────────────────────────────────────────────────────────

pub const DEFAULT_ADDRESS: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
pub const DEFAULT_CONTRACT_ID: &str = "CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD2KM";
pub const DEFAULT_TX_HASH: &str =
    "0000000000000000000000000000000000000000000000000000000000000001";
pub const DEFAULT_LEDGER: u64 = 12345;
pub const DEFAULT_TIMESTAMP: &str = "2024-01-15T12:00:00Z";

// ─── Mock RPC Account ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockRpcAccount {
    pub id: String,
    pub sequence: String,
    pub balances: Vec<MockBalance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockBalance {
    pub asset_type: String,
    pub balance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_issuer: Option<String>,
}

pub struct MockAccountBuilder {
    id: String,
    sequence: String,
    balances: Vec<MockBalance>,
}

impl Default for MockAccountBuilder {
    fn default() -> Self {
        Self {
            id: DEFAULT_ADDRESS.to_string(),
            sequence: "1234567890".to_string(),
            balances: vec![MockBalance {
                asset_type: "native".to_string(),
                balance: "10000.0000000".to_string(),
                asset_code: None,
                asset_issuer: None,
            }],
        }
    }
}

impl MockAccountBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn sequence(mut self, sequence: impl Into<String>) -> Self {
        self.sequence = sequence.into();
        self
    }

    pub fn add_balance(mut self, asset_type: impl Into<String>, balance: impl Into<String>) -> Self {
        self.balances.push(MockBalance {
            asset_type: asset_type.into(),
            balance: balance.into(),
            asset_code: None,
            asset_issuer: None,
        });
        self
    }

    pub fn build(self) -> MockRpcAccount {
        MockRpcAccount {
            id: self.id,
            sequence: self.sequence,
            balances: self.balances,
        }
    }
}

// ─── Mock RPC Transaction ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockRpcTransaction {
    pub id: String,
    pub hash: String,
    pub ledger: u64,
    pub status: String,
    pub envelope_xdr: String,
    pub result_xdr: String,
    pub result_meta_xdr: String,
    pub fee_charged: String,
    pub created_at: String,
}

pub struct MockTransactionBuilder {
    id: String,
    hash: String,
    ledger: u64,
    status: String,
    envelope_xdr: String,
    result_xdr: String,
    result_meta_xdr: String,
    fee_charged: String,
    created_at: String,
}

impl Default for MockTransactionBuilder {
    fn default() -> Self {
        Self {
            id: DEFAULT_TX_HASH.to_string(),
            hash: DEFAULT_TX_HASH.to_string(),
            ledger: DEFAULT_LEDGER,
            status: "SUCCESS".to_string(),
            envelope_xdr: "AAAA_ENVELOPE_AAAA".to_string(),
            result_xdr: "AAAA_RESULT_AAAA".to_string(),
            result_meta_xdr: "AAAA_META_AAAA".to_string(),
            fee_charged: "100".to_string(),
            created_at: DEFAULT_TIMESTAMP.to_string(),
        }
    }
}

impl MockTransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hash(mut self, hash: impl Into<String>) -> Self {
        self.hash = hash.into();
        self.id = self.hash.clone();
        self
    }

    pub fn ledger(mut self, ledger: u64) -> Self {
        self.ledger = ledger;
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = status.into();
        self
    }

    pub fn fee_charged(mut self, fee: impl Into<String>) -> Self {
        self.fee_charged = fee.into();
        self
    }

    pub fn created_at(mut self, timestamp: impl Into<String>) -> Self {
        self.created_at = timestamp.into();
        self
    }

    pub fn build(self) -> MockRpcTransaction {
        MockRpcTransaction {
            id: self.id,
            hash: self.hash,
            ledger: self.ledger,
            status: self.status,
            envelope_xdr: self.envelope_xdr,
            result_xdr: self.result_xdr,
            result_meta_xdr: self.result_meta_xdr,
            fee_charged: self.fee_charged,
            created_at: self.created_at,
        }
    }
}

// ─── Mock RPC Event ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockRpcEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub ledger: u64,
    #[serde(rename = "ledgerClosedAt")]
    pub ledger_closed_at: String,
    #[serde(rename = "contractId")]
    pub contract_id: String,
    #[serde(rename = "txHash")]
    pub tx_hash: String,
    pub topic: Vec<JsonValue>,
    pub value: JsonValue,
    #[serde(rename = "inSuccessfulContractCall")]
    pub in_successful_contract_call: bool,
}

pub struct MockRpcEventBuilder {
    id: String,
    event_type: String,
    ledger: u64,
    ledger_closed_at: String,
    contract_id: String,
    tx_hash: String,
    topic: Vec<JsonValue>,
    value: JsonValue,
    in_successful_contract_call: bool,
}

impl Default for MockRpcEventBuilder {
    fn default() -> Self {
        Self {
            id: format!("{}-0", DEFAULT_LEDGER),
            event_type: "contract".to_string(),
            ledger: DEFAULT_LEDGER,
            ledger_closed_at: DEFAULT_TIMESTAMP.to_string(),
            contract_id: DEFAULT_CONTRACT_ID.to_string(),
            tx_hash: DEFAULT_TX_HASH.to_string(),
            topic: vec![
                JsonValue::String("event_name".to_string()),
                JsonValue::String("param1".to_string()),
            ],
            value: JsonValue::Array(vec![
                JsonValue::Number(0.into()),
                JsonValue::String("1000".to_string()),
                JsonValue::String(DEFAULT_ADDRESS.to_string()),
            ]),
            in_successful_contract_call: true,
        }
    }
}

impl MockRpcEventBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = event_type.into();
        self
    }

    pub fn ledger(mut self, ledger: u64) -> Self {
        self.ledger = ledger;
        self.id = format!("{}-0", ledger);
        self
    }

    pub fn contract_id(mut self, contract_id: impl Into<String>) -> Self {
        self.contract_id = contract_id.into();
        self
    }

    pub fn tx_hash(mut self, tx_hash: impl Into<String>) -> Self {
        self.tx_hash = tx_hash.into();
        self
    }

    pub fn topic(mut self, topic: Vec<JsonValue>) -> Self {
        self.topic = topic;
        self
    }

    pub fn value(mut self, value: JsonValue) -> Self {
        self.value = value;
        self
    }

    pub fn ledger_closed_at(mut self, timestamp: impl Into<String>) -> Self {
        self.ledger_closed_at = timestamp.into();
        self
    }

    pub fn build(self) -> MockRpcEvent {
        MockRpcEvent {
            id: self.id,
            event_type: self.event_type,
            ledger: self.ledger,
            ledger_closed_at: self.ledger_closed_at,
            contract_id: self.contract_id,
            tx_hash: self.tx_hash,
            topic: self.topic,
            value: self.value,
            in_successful_contract_call: self.in_successful_contract_call,
        }
    }
}

// ─── Mock RPC Events Response ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockGetEventsResponse {
    pub events: Vec<MockRpcEvent>,
    #[serde(rename = "latestLedger")]
    pub latest_ledger: u64,
}

pub struct MockGetEventsResponseBuilder {
    events: Vec<MockRpcEvent>,
    latest_ledger: u64,
}

impl Default for MockGetEventsResponseBuilder {
    fn default() -> Self {
        Self {
            events: vec![MockRpcEventBuilder::default().build()],
            latest_ledger: DEFAULT_LEDGER,
        }
    }
}

impl MockGetEventsResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_event(mut self, event: MockRpcEvent) -> Self {
        self.events.push(event);
        self
    }

    pub fn events(mut self, events: Vec<MockRpcEvent>) -> Self {
        self.events = events;
        self
    }

    pub fn latest_ledger(mut self, ledger: u64) -> Self {
        self.latest_ledger = ledger;
        self
    }

    pub fn build(self) -> MockGetEventsResponse {
        MockGetEventsResponse {
            events: self.events,
            latest_ledger: self.latest_ledger,
        }
    }
}

// ─── Mock Latest Ledger ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockLatestLedger {
    pub id: String,
    pub sequence: u64,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: u32,
}

impl Default for MockLatestLedger {
    fn default() -> Self {
        Self {
            id: "latest_ledger_id".to_string(),
            sequence: DEFAULT_LEDGER,
            protocol_version: 21,
        }
    }
}

// ─── Mock Simulate Transaction Response ──────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSimulateResponse {
    #[serde(rename = "transactionData")]
    pub transaction_data: String,
    #[serde(rename = "minResourceFee")]
    pub min_resource_fee: String,
    pub events: Vec<JsonValue>,
    pub results: Vec<MockSimulateResult>,
    pub cost: MockSimulateCost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSimulateResult {
    pub xdr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockSimulateCost {
    #[serde(rename = "cpuInsn")]
    pub cpu_insn: String,
    #[serde(rename = "memByte")]
    pub mem_byte: String,
}

impl Default for MockSimulateResponse {
    fn default() -> Self {
        Self {
            transaction_data: "AAAA_SIM_DATA_AAAA".to_string(),
            min_resource_fee: "1000".to_string(),
            events: vec![],
            results: vec![MockSimulateResult {
                xdr: "AAAA_RESULT_XDR_AAAA".to_string(),
            }],
            cost: MockSimulateCost {
                cpu_insn: "1000000".to_string(),
                mem_byte: "2000000".to_string(),
            },
        }
    }
}

// ─── Domain-Specific Event Factories ─────────────────────────────────────────

/// Create a waste_registered event
pub fn mock_waste_registered_event(
    waste_id: u64,
    recycler: &str,
    weight: u64,
) -> MockRpcEvent {
    MockRpcEventBuilder::default()
        .event_type("contract")
        .topic(vec![
            JsonValue::String("waste_registered".to_string()),
            JsonValue::Number(waste_id.into()),
        ])
        .value(JsonValue::Array(vec![
            JsonValue::Number(0.into()), // waste_type
            JsonValue::String(weight.to_string()),
            JsonValue::String(recycler.to_string()),
            JsonValue::String("40000000".to_string()), // latitude
            JsonValue::String("-74000000".to_string()), // longitude
        ]))
        .build()
}

/// Create a participant_registered event
pub fn mock_participant_registered_event(
    address: &str,
    name: &str,
    role: u32,
) -> MockRpcEvent {
    MockRpcEventBuilder::default()
        .event_type("contract")
        .topic(vec![
            JsonValue::String("participant_registered".to_string()),
            JsonValue::String(address.to_string()),
        ])
        .value(JsonValue::Array(vec![
            JsonValue::String(address.to_string()),
            JsonValue::String(name.to_string()),
            JsonValue::Number(role.into()),
            JsonValue::String("0".to_string()), // latitude
            JsonValue::String("0".to_string()), // longitude
        ]))
        .build()
}

/// Create a waste_transferred event
pub fn mock_waste_transferred_event(
    waste_id: u64,
    from: &str,
    to: &str,
) -> MockRpcEvent {
    MockRpcEventBuilder::default()
        .event_type("contract")
        .topic(vec![
            JsonValue::String("waste_transferred".to_string()),
            JsonValue::Number(waste_id.into()),
        ])
        .value(JsonValue::Array(vec![
            JsonValue::String(from.to_string()),
            JsonValue::String(to.to_string()),
            JsonValue::Number(waste_id.into()),
        ]))
        .build()
}

/// Create a tokens_rewarded event
pub fn mock_tokens_rewarded_event(
    recipient: &str,
    amount: u64,
) -> MockRpcEvent {
    MockRpcEventBuilder::default()
        .event_type("contract")
        .topic(vec![
            JsonValue::String("tokens_rewarded".to_string()),
            JsonValue::String(recipient.to_string()),
        ])
        .value(JsonValue::Array(vec![
            JsonValue::String(recipient.to_string()),
            JsonValue::String(amount.to_string()),
        ]))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_account_builder() {
        let account = MockAccountBuilder::new()
            .id("GTEST")
            .sequence("999")
            .build();

        assert_eq!(account.id, "GTEST");
        assert_eq!(account.sequence, "999");
        assert_eq!(account.balances.len(), 1);
    }

    #[test]
    fn test_mock_transaction_builder() {
        let tx = MockTransactionBuilder::new()
            .hash("abc123")
            .status("FAILED")
            .ledger(54321)
            .build();

        assert_eq!(tx.hash, "abc123");
        assert_eq!(tx.status, "FAILED");
        assert_eq!(tx.ledger, 54321);
    }

    #[test]
    fn test_mock_event_builder() {
        let event = MockRpcEventBuilder::new()
            .event_type("system")
            .ledger(99999)
            .build();

        assert_eq!(event.event_type, "system");
        assert_eq!(event.ledger, 99999);
        assert!(event.in_successful_contract_call);
    }

    #[test]
    fn test_domain_specific_events() {
        let waste_event = mock_waste_registered_event(42, DEFAULT_ADDRESS, 1500);
        assert_eq!(waste_event.topic[0], JsonValue::String("waste_registered".to_string()));

        let participant_event = mock_participant_registered_event(DEFAULT_ADDRESS, "Test User", 0);
        assert_eq!(participant_event.topic[0], JsonValue::String("participant_registered".to_string()));

        let transfer_event = mock_waste_transferred_event(42, DEFAULT_ADDRESS, "GOTHER");
        assert_eq!(transfer_event.topic[0], JsonValue::String("waste_transferred".to_string()));

        let reward_event = mock_tokens_rewarded_event(DEFAULT_ADDRESS, 500);
        assert_eq!(reward_event.topic[0], JsonValue::String("tokens_rewarded".to_string()));
    }
}
