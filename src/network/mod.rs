use std::rc::Rc;

mod link;
mod node;
mod builder;
mod event;
mod message;

use link::Link;
use node::{Node, NodeID};
use message::{ProtocolMsg, GenesisMsg, BlockID};
use event::Event;

pub struct Network{
    nodes: Vec<Rc<dyn Node>>,
    links: Vec<Rc<Link>>,
}
pub struct Transaction{
}