
use std::collections::HashMap;

/// Protocol message
/// 

use serde::{Deserialize, Serialize};
use super::{NodeID,Transaction};

pub type BlockID = String;

pub enum ProtocolMsg{
    Genesis(GenesisMsg),
    BlockProposed(BlockMsg),
}

#[derive(Serialize,Deserialize)]
pub struct GenesisMsg{
    pub bond_map: HashMap<NodeID, u128>,
}

#[derive(Serialize,Deserialize)]
pub struct BlockMsg{
    
    pub block_id: BlockID,
    
    pub validator_id: NodeID,
    
    pub justifications: Vec<BlockID>,
    
    #[serde(skip)]
    transactions: Vec<Transaction>,  

}