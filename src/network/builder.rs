/// Network builder
/// 
/// 
/// 
use super::{Network, GenesisMsg};
pub trait NetworkBuilder{
    fn make_network(genesis: GenesisMsg) -> Network;
}

///Build a simple Kademila network with simple all honest nodes
struct SimpleHonestBuilder{   

}

impl NetworkBuilder for SimpleHonestBuilder{
    fn make_network(genesis: GenesisMsg) -> Network{
        todo!()
    }
}