use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

// Add this after the existing use statements
#[derive(Debug, Clone)]
pub struct ShareStorage {
    // (protocol_id, step_id, party_id) -> (share_1half, share_2half)
    shares: HashMap<(u32, u32, u32), (u64, u64)>,
}

impl ShareStorage {
    pub fn new() -> Self {
        ShareStorage {
            shares: HashMap::new(),
        }
    }

    pub fn store_share(&mut self, protocol_id: u32, step_id: u32, party_id: u32, share: (u64, u64)) {
        self.shares.insert((protocol_id, step_id, party_id), share);
    }

    pub fn get_share(&self, protocol_id: u32, step_id: u32, party_id: u32) -> Option<(u64, u64)> {
        self.shares.get(&(protocol_id, step_id, party_id)).copied()
    }

    pub fn show_shares(&self) {
        for ((protocol_id, step_id, party_id), (share_1half, share_2half)) in &self.shares {
            println!("Protocol: {}, Step: {}, Party: {}, Share: ({}, {})", 
                     protocol_id, step_id, party_id, share_1half, share_2half);
        }
    }
}