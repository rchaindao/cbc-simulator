/// Network builder
/// 

use std::collections::HashMap;
use super::{Network, GenesisMsg};
pub trait NetworkBuilder{
    fn make_network(genesis: GenesisMsg, opt: HashMap<String, String>) -> Network;
}

///Build a simple Kademila network with all honest nodes
struct AllHonestKademilaBuilder{   

}

impl NetworkBuilder for AllHonestKademilaBuilder{
    fn make_network(genesis: GenesisMsg, opt: HashMap<String, String>) -> Network{
        todo!()
    }
}