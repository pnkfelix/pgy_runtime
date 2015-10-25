extern crate typed_arena;

use std::fmt;

pub trait FromChar { fn from_char(c: char) -> Self; }
impl FromChar for char { fn from_char(c: char) -> Self { c } }
pub trait LabelZero { fn label_zero() -> Self; }

pub trait Context<'g, LABEL> {
    type Success: Default;
    type ParseError: Default;
    type Term: FromChar;

    fn i_in(&self, &[Self::Term]) -> bool;
    fn i_len(&self) -> usize;
    fn i_incr(&mut self);
    fn pop(&mut self);
    fn g_dummy(&self) -> &'g Node<'g, GData<LABEL>>;
    fn r_pop(&mut self) -> Option<Desc<'g, LABEL>>;
    fn r_seen_contains(&self, &Desc<'g, LABEL>) -> bool;
    fn set_s(&mut self, u: Stack<'g, LABEL>);
    fn set_i(&mut self, j: InputPos);
}

use graph::{Node};

pub mod arena;
pub mod graph;
pub mod demo;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct InputPos(pub usize);
impl InputPos {
    pub fn incr(&mut self) {
        self.0 += 1;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Desc<'g, LABEL:'g>(pub LABEL,
                              pub Stack<'g, LABEL>,
                              pub InputPos);
#[derive(Copy, Clone, PartialEq)]
pub struct GData<LABEL>(Option<(LABEL, InputPos)>);
impl<LABEL> GData<LABEL> {
    pub fn dummy() -> GData<LABEL> { GData(None) }
    pub fn new(l: LABEL, i: InputPos) -> Self {
        GData(Some((l, i)))
    }
    pub fn label(&self) -> LABEL where LABEL:Copy {
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
pub struct Stack<'g, LABEL:'g>(pub &'g Node<'g, GData<LABEL>>);
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
