use crate::pbft::PbftMessage;
use std::sync::mpsc::{Sender, Receiver, channel};

pub struct Network {
    pub sender: Sender<PbftMessage>,
    pub receiver: Receiver<PbftMessage>,
}

impl Network {
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self { sender: tx, receiver: rx }
    }

    pub fn send(&self, msg: PbftMessage) {
        println!("Sending message: {:?}", msg);
        self.sender.send(msg).unwrap();
    }

    pub fn receive(&self) -> Option<PbftMessage> {
        match self.receiver.recv() {
            Ok(msg) => Some(msg),
            Err(_) => None,
        }
    }
}
