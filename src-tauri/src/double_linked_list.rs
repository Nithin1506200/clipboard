use std::{
    cell::RefCell,
    clone,
    fmt::Display,
    rc::{Rc, Weak},
};
#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}
impl<T: Clone> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
    pub fn val(&self) -> T {
        self.val.clone()
    }
}
#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}
impl<T> DoubleLinkedList<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn peak_front(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.head.clone()
    }
    pub fn push_front_rc(&mut self, node: Rc<RefCell<Node<T>>>) {
        {
            let mut node = node.borrow_mut();
            node.prev = None;
            node.next = None
        }
        if self.len == 0 {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            match self.head.take() {
                Some(ptr) => {
                    let mut node_ = node.borrow_mut();
                    node_.next = Some(ptr.clone());
                    self.head = Some(node.clone());
                    ptr.borrow_mut().prev = Some(Rc::downgrade(&node));
                }
                None => self.head = Some(node.clone()),
            }
        }
        self.len += 1;
    }
    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);
        if self.len == 0 {
            let n = Rc::new(RefCell::new(node));
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        } else {
            match self.head.take() {
                Some(ptr) => {
                    node.next = Some(ptr.clone());
                    let n = Rc::new(RefCell::new(node));
                    self.head = Some(n.clone());
                    ptr.borrow_mut().prev = Some(Rc::downgrade(&n));
                }
                None => self.head = Some(Rc::new(RefCell::new(node))),
            }
        }
        self.len += 1;
    }
    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);
        if self.len == 0 {
            let n = Rc::new(RefCell::new(node));
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        } else {
            match self.tail.take() {
                Some(ptr) => {
                    node.prev = Some(Rc::downgrade(&ptr));
                    let n = Rc::new(RefCell::new(node));
                    self.tail = Some(n.clone());
                    ptr.borrow_mut().next = self.tail.clone();
                }

                None => self.tail = Some(Rc::new(RefCell::new(node))),
            }
        }
        self.len += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let node = match self.head.take() {
            Some(node) => {
                self.head = node.borrow().next.clone();

                Some(node.borrow().val.clone())
            }
            None => None,
        };
        if self.len == 1 {
            self.tail = None
        }
        self.len = (self.len.max(1) - 1).max(0);
        node
    }
    pub fn pop_back(&mut self) -> Option<T> {
        let node = match self.tail.take() {
            Some(node) => Some(node.borrow().val.clone()),
            None => None,
        };
        if self.len == 1 {
            self.head = None
        }
        self.len = (self.len.max(1) - 1).max(0);
        node
    }
    pub fn delete(&mut self, node: Rc<RefCell<Node<T>>>) {
        let node = node.borrow_mut();
        let prev = node.prev.clone();
        let next = node.next.clone();
        self.len -= 1;
        match (prev, next) {
            (None, None) => {
                self.head = None;
                self.tail = None
            }
            (None, Some(node)) => {
                self.head = Some(node.clone());
            }
            (Some(prev), None) => match prev.upgrade() {
                Some(prev) => self.tail = Some(prev.clone()),
                None => self.tail = None,
            },
            (Some(prev), Some(next)) => {
                next.borrow_mut().prev = Some(prev.clone());
                match prev.upgrade() {
                    Some(prev) => prev.borrow_mut().next = Some(next.clone()),
                    None => self.head = Some(next.clone()),
                }
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}
