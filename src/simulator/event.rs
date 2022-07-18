///Simulation event
use serde::{Deserialize, Serialize};
use super::{NodeID, BlockID};

pub type Timestamp = u64;
pub enum Event{
    BlockReceived(BlockRecvEvent),
}

#[derive(Serialize, Deserialize)]
pub struct BlockRecvEvent{
    pub timestamp: Timestamp,
    pub block_id: BlockID,
    pub node_id: NodeID,
}

#[derive(Serialize, Deserialize)]
pub struct FringeFoundEvent{
    pub timestamp: Timestamp,
    pub node: NodeID,
    pub fringe: Vec<BlockID>,
}