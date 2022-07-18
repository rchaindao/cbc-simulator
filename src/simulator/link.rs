use super::NodeID;
/// Simulate a link between two nodes
pub struct Link{
    pub pairs : (NodeID, NodeID),  //Node pairs this link connects, can be directional or undirectional
    pub latency_qntl : Vec<u64>,   //Latency quantile in miliseconds
    pub fail_prob: f64,            //Hazard rate of this link getting dropped    
}


