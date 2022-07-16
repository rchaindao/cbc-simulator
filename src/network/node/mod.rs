use std::rc::Rc;
use std::collections::HashMap;

use super::{Link, Event, Transaction, ProtocolMsg};

mod simple_honest_node;
use simple_honest_node::SimpleHonestNode;

pub type NodeID = String;

pub trait Node{
    fn get_id(&self)-> NodeID;

    fn get_links(&self)->Vec<Rc<Link>>;

    fn broad_cast(&mut self, msg: &ProtocolMsg)->Vec<Event>;

    fn on_protocol_msg(&mut self, msg: &ProtocolMsg)->Vec<Event>;

    fn on_new_transaction(&mut self, tx: &Transaction)->Vec<Event>;
}

///Factory to make nodes given parameters
pub fn new_node(opt: HashMap<String, String>)->Rc<dyn Node>{
    todo!()
}
