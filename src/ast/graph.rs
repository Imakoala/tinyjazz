pub use crate::ast::BiOp;
use global_counter::global_counter;
use std::{cell::RefCell, hash::Hash, ops::Deref, rc::Rc};
//this is just used to get a unique value.
global_counter!(COUNTER, u32, 0);
fn get_value() -> u32 {
    COUNTER.inc();
    COUNTER.get_cloned()
}
//Without going into too much details, the RCell type is a wrapper (some kind of pointer),
//with reference counting included (so it can be freed).
//It allows for mutable access to the value inside,
//and has a unique id which is used for hashing.
//this impl is way better than the one in graph_automaton.rs, which is supposed to disseapear at
//some point anyway.
#[derive(Debug, Clone, Eq)]
pub struct RCell<T>(Rc<(RefCell<T>, u32)>);
impl<T> RCell<T> {
    pub fn new(value: T) -> Self {
        RCell(Rc::new((RefCell::new(value), get_value())))
    }
    pub fn id(&self) -> u32 {
        (self.0).1
    }
}
impl<T> Deref for RCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &(self.0).0
    }
}

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
//the main program. Only needs the name and value of the outputs, and the size of the inputs.
#[derive(Debug, Clone)]
pub struct FlatProgramGraph {
    pub outputs: Vec<(String, RCell<Node>)>,
    pub inputs: Vec<usize>,
}

//A "Node" of the dataflow graph is an operation
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Input(usize), //Input for the whole program
    Const(Vec<bool>),
    Not(RCell<Node>),
    Slice(RCell<Node>, usize, usize),
    BiOp(BiOp, RCell<Node>, RCell<Node>),
    Mux(RCell<Node>, RCell<Node>, RCell<Node>),
    Reg(usize, RCell<Node>), //The size is still specified
    Ram(RCell<Node>, RCell<Node>, RCell<Node>, RCell<Node>),
    Rom(usize, RCell<Node>), //Size specified here as well
    TmpValueHolder(usize), //This is used while building the graph. All instance of this are removed.
}
//As this struct can be recursive, I needed to make my own pretty-printer
//(the default one just overflows the stack when it is applied on a cyclic struct...)
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", print_node(self, false, "".into()))
    }
}
fn print_node(node: &Node, in_reg: bool, indent: String) -> String {
    match node {
        Node::Input(i) => format!("Input {}", i),
        Node::Const(v) => format!("Const {:?}", v),
        Node::Not(e) => format!(
            "Not {}",
            print_node(&*e.borrow(), in_reg, indent.clone() + " ")
        ),
        Node::Slice(e, c1, c2) => {
            format!(
                "Slice {} {} {}",
                c1,
                c2,
                print_node(&*e.borrow(), in_reg, indent.clone() + " ")
            )
        }
        Node::BiOp(op, e1, e2) => format!(
            "{:?} {} \n{}{}",
            op,
            print_node(&*e1.borrow(), in_reg, indent.clone() + " "),
            indent,
            print_node(&*e2.borrow(), in_reg, indent.clone() + " ")
        ),
        Node::Mux(e1, e2, e3) => format!(
            "Mux {} \n{}{} \n{}{}",
            print_node(&*e1.borrow(), in_reg, indent.clone() + " "),
            indent,
            print_node(&*e2.borrow(), in_reg, indent.clone() + " "),
            indent,
            print_node(&*e3.borrow(), in_reg, indent.clone() + " ")
        ),
        Node::Reg(_, e) => {
            if in_reg {
                "Reg loop".to_string()
            } else {
                format!(
                    "Reg {}",
                    print_node(&*e.borrow(), true, indent.clone() + " ")
                )
            }
        }
        Node::Ram(e1, e2, e3, e4) => format!(
            "Ram \n{}{} \n{}{} \n{}{} \n{}{}",
            indent,
            print_node(&*e1.borrow(), in_reg, indent.clone() + " "),
            indent,
            print_node(&*e2.borrow(), in_reg, indent.clone() + " "),
            indent,
            print_node(&*e3.borrow(), in_reg, indent.clone() + " "),
            indent,
            print_node(&*e4.borrow(), in_reg, indent.clone() + " ")
        ),
        Node::Rom(_, e) => format!(
            "Rom {}",
            print_node(&*e.borrow(), in_reg, indent.clone() + " ")
        ),
        Node::TmpValueHolder(i) => format!("Temp value {}", i),
    }
}
