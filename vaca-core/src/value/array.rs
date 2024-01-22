use std::{collections::LinkedList, rc::Rc};
use crate::Value;

pub type Array = LinkedList<Rc<Value>>;