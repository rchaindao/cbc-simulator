use super::{Node, NodeID, Link, ProtocolMsg, Event};
use std::rc::Rc;

pub struct Network{
    pub(crate) nodes: Vec<Rc<dyn Node>>,
    pub(crate) links: Vec<Rc<Link>>,
}

impl Network{     
    
    pub fn get_node(&self, id: NodeID) -> Option<Rc<dyn Node>>{
        todo!()
    }


    pub fn broad_cast(&mut self, node_id: NodeID, msg: &ProtocolMsg)->Vec<Event>{
        todo!()
        //Simulate node broadcasting a protocl message to all its neighbours
    }

}