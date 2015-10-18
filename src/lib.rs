extern crate typed_arena;

use std::fmt;

pub trait Context {
    
}

use graph::{Node};

pub mod arena;
pub mod graph;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InputPos(usize);
impl InputPos {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Desc<'g, LABEL:'g>(LABEL, Stack<'g, LABEL>, InputPos);
#[derive(Copy, Clone, PartialEq)]
struct GData<LABEL>(Option<(LABEL, InputPos)>);
impl<LABEL> GData<LABEL> {
    fn dummy() -> GData<LABEL> { GData(None) }
    fn new(l: LABEL, i: InputPos) -> Self {
        GData(Some((l, i)))
    }
    fn label(&self) -> LABEL where LABEL:Copy {
        self.0.unwrap().0
    }
}
impl<LABEL: fmt::Debug> fmt::Debug for GData<LABEL> {
    fn fmt(&self, w: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.0 {
            None => {
                write!(w, "$")
            }
            Some(ref p) => {
                write!(w, "{:?}^{:?}", p.0, (p.1).0)
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Stack<'g, LABEL:'g>(&'g Node<'g, GData<LABEL>>);
impl<'g, LABEL> PartialEq for Stack<'g, LABEL> {
    fn eq(&self, rhs: &Stack<'g, LABEL>) -> bool {
        (self.0 as *const _) == (rhs.0 as *const _)
    }
}
impl<'g, LABEL> Stack<'g, LABEL> {
    fn empty(&self) -> bool {
        self.0.children().count() == 0
    }
}
impl<'g, LABEL: fmt::Debug> fmt::Debug for Stack<'g, LABEL> {
    fn fmt(&self, w: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(w, "Stack [ ");
        let mut n = self.0;
        loop {
            write!(w, "{:?} ", n.data);
            match n.children().count() {
                0 => break,
                1 => { n = n.children().next().unwrap(); }
                _ => { write!(w, ".."); break; }
            }
        }
        write!(w, "]")
    }
}
