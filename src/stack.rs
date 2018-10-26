use std::iter::{FromIterator, Iterator};
use std::rc::Rc;
use std::vec::Vec;

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

    pub fn push_rc(&mut self, element: Rc<A>) {
        self.stack = match std::mem::replace(&mut self.stack, None) {
            None => Option::Some(Rc::new(Node::new_rc(element))),
            Some(ref mut prev) => Some(Rc::new(Node::push_rc(element, prev))),
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

impl<A> FromIterator<A> for Stack<A> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut new = Stack::new();
        let next: Vec<A> = iter.into_iter().collect();
        for item in next.into_iter().rev() {
            new.push(item);
        }
        new
    }
}

impl<A> FromIterator<Rc<A>> for Stack<A> {
    fn from_iter<I: IntoIterator<Item = Rc<A>>>(iter: I) -> Self {
        let mut new = Stack::new();
        let next: Vec<Rc<A>> = iter.into_iter().collect();
        for item in next.into_iter().rev() {
            new.push_rc(item);
        }
        new
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

    fn new_rc(element: Rc<A>) -> Self {
        Node {
            element: element,
            next: None,
        }
    }

    fn push_rc(element: Rc<A>, prev: &Rc<Node<A>>) -> Self {
        Node {
            element: element,
            next: Some(prev.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use stack::*;
}
