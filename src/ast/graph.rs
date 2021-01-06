pub use crate::ast::BiOp;
use global_counter::global_counter;
use std::{cell::RefCell, rc::Rc};
use std::{hash::Hash, ops::Deref};
global_counter!(COUNTER, u32, 0);
fn get_value() -> u32 {
    COUNTER.inc();
    COUNTER.get_cloned()
}
#[derive(Debug, Clone, Eq)]
pub struct RCell<T>(Rc<(RefCell<T>, u32)>);
impl<T> RCell<T> {
    pub fn new(value: T) -> Self {
        RCell(Rc::new((RefCell::new(value), get_value())))
    }
}
impl<T> Deref for RCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &(self.0).0
    }
}
//WARNING ! the hash and eq impl comapre pointers !
impl<T: PartialEq> PartialEq for RCell<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.0).1 == (other.0).1
    }
}
impl<T: Hash> Hash for RCell<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.0).1.hash(state);
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Input(usize),
    Const(Vec<bool>),
    Not(RCell<Node>),
    Slice(RCell<Node>, usize, usize),
    BiOp(BiOp, RCell<Node>, RCell<Node>),
    Mux(RCell<Node>, RCell<Node>, RCell<Node>),
    Reg(usize, RCell<Node>), //size, node.
    Ram(RCell<Node>, RCell<Node>, RCell<Node>, RCell<Node>),
    Rom(RCell<Node>),
    TmpValueHolder(usize),
}
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_node(self, false))
    }
}
fn print_node(node: &Node, in_reg: bool) -> String {
    match node {
        Node::Input(i) => format!("Input {}", i),
        Node::Const(v) => format!("Const {:?}", v),
        Node::Not(e) => format!("Not {}", print_node(&*e.borrow(), in_reg)),
        Node::Slice(e, c1, c2) => {
            format!("Slice {} {} {}", c1, c2, print_node(&*e.borrow(), in_reg))
        }
        Node::BiOp(op, e1, e2) => format!(
            "{:?} {} \n {}",
            op,
            print_node(&*e1.borrow(), in_reg),
            print_node(&*e2.borrow(), in_reg)
        ),
        Node::Mux(e1, e2, e3) => format!(
            "Mux {} \n {} \n {}",
            print_node(&*e1.borrow(), in_reg),
            print_node(&*e2.borrow(), in_reg),
            print_node(&*e3.borrow(), in_reg)
        ),
        Node::Reg(_, e) => {
            if in_reg {
                "Reg loop".to_string()
            } else {
                format!("Reg {}", print_node(&*e.borrow(), true))
            }
        }
        Node::Ram(e1, e2, e3, e4) => format!(
            "Ram {} \n {} \n {} \n {}",
            print_node(&*e1.borrow(), in_reg),
            print_node(&*e2.borrow(), in_reg),
            print_node(&*e3.borrow(), in_reg),
            print_node(&*e4.borrow(), in_reg)
        ),
        Node::Rom(e) => format!("Rom {}", print_node(&*e.borrow(), in_reg)),
        Node::TmpValueHolder(i) => format!("Temp value {}", i),
    }
}
#[derive(Debug, Clone)]
pub struct FlatProgramGraph {
    pub outputs: Vec<(String, RCell<Node>)>,
    pub inputs: Vec<usize>,
}
