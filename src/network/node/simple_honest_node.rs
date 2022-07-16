use std::rc::Rc;
use crate::network::{NodeID, Node, Link, Event, Transaction, ProtocolMsg};

pub struct SimpleHonestNode{
    id : NodeID,
    links : Vec<Rc<Link>>,
}

impl Node for SimpleHonestNode{
    fn get_id(&self)-> NodeID{
        todo!()
    }

    fn get_links(&self)->Vec<Rc<Link>>{
        todo!()
    }

    fn broad_cast(&mut self, msg: &ProtocolMsg)->Vec<Event>{
        todo!()
    }

    fn on_protocol_msg(&mut self, msg: &ProtocolMsg)->Vec<Event>{
        todo!()
    }

    fn on_new_transaction(&mut self, tx: &Transaction)->Vec<Event>{
        todo!()
    }
}