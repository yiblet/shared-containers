use std::iter::{IntoIterator, Iterator};
use std::rc::Rc;

#[derive(Clone)]
pub struct Stack<A> {
    stack: Option<Rc<Node<A>>>,
}

impl<A> Stack<A> {
    pub fn new() -> Self {
        Stack { stack: None }
    }

    pub fn push(&mut self, element: A) {
        self.stack = match std::mem::replace(&mut self.stack, None) {
            None => Option::Some(Rc::new(Node::new(element))),
            Some(ref mut prev) => Some(Rc::new(Node::push(element, prev))),
        }
    }

    pub fn peek_stack(&self) -> Stack<A> {
        Self {
            stack: self
                .stack
                .as_ref()
                .and_then(|node: &Rc<Node<A>>| node.as_ref().next.clone()),
        }
    }

    pub fn peek(&self) -> Option<Rc<A>> {
        self.stack.as_ref().map(|node| node.element.clone())
    }

    pub fn pop(&mut self) -> Option<Rc<A>> {
        if let Some(node) = std::mem::replace(&mut self.stack, None) {
            let Node { element, next } = node.as_ref();
            self.stack = next.clone();
            Some(element.clone())
        } else {
            None
        }
    }
}

impl<A> Iterator for Stack<A> {
    type Item = Rc<A>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}

#[derive(Clone)]
struct Node<A> {
    element: Rc<A>,
    next: Option<Rc<Node<A>>>,
}

impl<A> Node<A> {
    fn new(element: A) -> Self {
        Node {
            element: Rc::new(element),
            next: None,
        }
    }

    fn push(element: A, prev: &Rc<Node<A>>) -> Self {
        Node {
            element: Rc::new(element),
            next: Some(prev.clone()),
        }
    }
}
