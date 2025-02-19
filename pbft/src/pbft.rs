use std::time::SystemTime;

#[derive(Debug, Clone)]
pub enum PbftPhase {
    Request,
    PrePrepare,
    Prepare,
    Commit,
    Reply,
}

#[derive(Debug, Clone)]
pub struct PbftMessage {
    pub sender: u32,       // Node ID
    pub phase: PbftPhase,  // PBFT phase
    pub timestamp: u64,    // Time of request
    pub data: Vec<u8>,     // Transaction data (simplified)
    pub signature: Vec<u8>, // Digital signature (TODO)
}

impl PbftMessage {
    pub fn new(sender: u32, phase: PbftPhase, data: Vec<u8>) -> Self {
        Self {
            sender,
            phase,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
            signature: vec![], // TODO: Add cryptographic signature
        }
    }
}
