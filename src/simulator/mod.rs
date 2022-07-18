use std::sync::mpsc::{Receiver};

mod link;
mod node;
mod network_builder;
mod event;
mod message;
mod network;

use link::Link;
use node::{Node, NodeID};
use message::{ProtocolMsg, GenesisMsg, BlockID};
use event::Event;
use network::Network;

use self::event::Timestamp;

struct Simulator{
    network: Network,
    event_queue: Vec<Event>,
}

impl Simulator{

    // In future this loop can iterate over an async stream
    fn event_loop(&self, rx : &Receiver<Transaction> ){        
        for tx in rx.iter(){
            //New TX submitted to system
            let node = self.network.get_node(tx.node_id).unwrap();
            
        }
    }
    
}

type TransactionID = u128;
pub struct Transaction{
    timestamp: Timestamp,
    tx_id : TransactionID,
    node_id: NodeID,
}