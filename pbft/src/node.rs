use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub state: NodeState,
    pub peers: HashMap<u32, String>, // Node ID -> Address
}

#[derive(Debug, Clone)]
pub enum NodeState {
    Idle,
    Primary,
    Replica,
    Committing,
}

impl Node {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            state: NodeState::Replica,  // Default state
            peers: HashMap::new(),
        }
    }

    pub fn handle_message(&self, msg: PbftMessage) {
        match msg.phase {
            PbftPhase::Request => println!("Handling client request..."),
            PbftPhase::PrePrepare => println!("Pre-prepare phase..."),
            PbftPhase::Prepare => println!("Prepare phase..."),
            PbftPhase::Commit => println!("Commit phase..."),
            PbftPhase::Reply => println!("Reply to client..."),
        }
    }
}
