use std::cmp::Eq;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::hash::{Hash, Hasher};
use std::mem;

use super::value::Value;
use super::environment::Environment;

pub trait Clip: Debug {
    fn get(&self, &Value) -> Value;
    fn set(&mut self, Value, Value) -> Result<(), String>;
    fn play(&mut self, Vec<Value>, &mut Environment) -> Result<Value, String>;
}

#[derive(Clone, Debug)]
pub struct ClipHolder {
    clip: Rc<RefCell<Box<Clip>>>
}

impl Eq for ClipHolder {}

impl Hash for ClipHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_ptr: usize = unsafe {mem::transmute(&self.clip.borrow())};
        self_ptr.hash(state);
    }
}

impl PartialEq for ClipHolder {
    fn eq(&self, other: &ClipHolder) -> bool {
        let self_ptr: usize = unsafe {mem::transmute(self.clip.as_unsafe_cell().get())};
        let other_ptr: usize = unsafe {mem::transmute(other.clip.as_unsafe_cell().get())};
        self_ptr == other_ptr
    }

    fn ne(&self, other: &ClipHolder) -> bool {
        !self.eq(other)
    }
}

impl ClipHolder {
    pub fn new(clip: Box<Clip>) -> ClipHolder {
        ClipHolder {
            clip: Rc::new(RefCell::new(clip))
        }
    }

    pub fn borrow_mut(&mut self) -> RefMut<Box<Clip>> {
        self.clip.borrow_mut()
    }
}
