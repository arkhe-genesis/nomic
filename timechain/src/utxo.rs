use crate::mhd::EvoField;
use crate::timechain::ShadowHash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtxoRef {
    pub tx_id: [u8; 32],
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utxo {
    pub id: [u8; 32],
    pub owner: [u8; 32],
    pub value: u64,
    pub field_signature: ShadowHash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub inputs: Vec<UtxoRef>,
    pub outputs: Vec<Utxo>,
    pub witness: Vec<u8>,
    pub reconnection_phase: f64,
}

impl Transaction {
    pub fn verify(&self, _current_field: &EvoField) -> bool {
        // In a full implementation, this checks topological consistency
        true
    }
}
