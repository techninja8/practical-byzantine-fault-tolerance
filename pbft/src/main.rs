mod node;
mod pbft;
mod network;
mod utils;

use node::Node;
use std::sync::{Arc, Mutex};

fn main() {
    println!("Starting PBFT node...");

    let node = Arc::new(Mutex::new(Node::new(1)));  // Node ID = 1
    // TODO: Start networking, receive messages, and trigger PBFT state transitions.

    println!("PBFT skeleton initialized.");
}
