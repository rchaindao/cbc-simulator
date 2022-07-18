use std::rc::Rc;
use std::collections::HashMap;

use super::{Link, Event, Transaction, ProtocolMsg};

mod simple_honest_node;
use simple_honest_node::SimpleHonestNode;

pub type NodeID = String;

pub trait Node{
    fn get_id(&self)-> NodeID;

    fn get_links(&self)->Vec<Rc<Link>>;

    // Message relay may be implemented differently because we may want to simualte eclipse attack
    fn relay(&mut self, source_link: Rc<Link>, msg: &ProtocolMsg)->Vec<Event>;

    fn on_protocol_msg(&mut self, msg: &ProtocolMsg)->Vec<Event>;

    fn on_new_transaction(&mut self, tx: &Transaction)->Vec<Event>;
}

pub fn simple_relay<T : Node>(node: &T, source_link: Rc<Link>, msg: &ProtocolMsg) -> Vec<Event>{
    //Relay the procol message to all nodes connected to the nodes except for the source link 
    //Then generate a vector of BlockReceivedEvent
    todo!();
}

///Factory to make nodes given parameters
pub fn new_node(opt: HashMap<String, String>)->Rc<dyn Node>{
    todo!()
    //SimpleHonestNode::new()
}
