use std::{
    borrow::BorrowMut as _,
    cell::RefCell,
    sync::{Arc, Mutex, Weak},
};
#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Arc<Mutex<Node<T>>>>,
    prev: Option<Weak<Mutex<Node<T>>>>,
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
    head: Option<Arc<Mutex<Node<T>>>>,
    tail: Option<Arc<Mutex<Node<T>>>>,
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
    pub fn peak_front(&self) -> Option<Arc<Mutex<Node<T>>>> {
        self.head.clone()
    }
    pub fn push_front_rc(&mut self, node: Arc<Mutex<Node<T>>>) {
        {
            let mut node = node.lock().unwrap();
            node.prev = None;
            node.next = None
        }
        if self.len == 0 {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            match self.head.take() {
                Some(ptr) => {
                    let mut node_ = node.lock().unwrap();
                    node_.next = Some(ptr.clone());
                    self.head = Some(node.clone());
                    ptr.lock().unwrap().prev = Some(Arc::downgrade(&node));
                }
                None => self.head = Some(node.clone()),
            }
        }
        self.len += 1;
    }
    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);
        if self.len == 0 {
            let n = Arc::new(Mutex::new(node));
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        } else {
            match self.head.take() {
                Some(ptr) => {
                    node.next = Some(ptr.clone());
                    let n = Arc::new(Mutex::new(node));
                    self.head = Some(n.clone());
                    ptr.lock().unwrap().prev = Some(Arc::downgrade(&n));
                }
                None => self.head = Some(Arc::new(Mutex::new(node))),
            }
        }
        self.len += 1;
    }
    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);
        if self.len == 0 {
            let n = Arc::new(Mutex::new(node));
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        } else {
            match self.tail.take() {
                Some(ptr) => {
                    node.prev = Some(Arc::downgrade(&ptr));
                    let n = Arc::new(Mutex::new(node));
                    self.tail = Some(n.clone());
                    ptr.lock().unwrap().next = self.tail.clone();
                }

                None => self.tail = Some(Arc::new(Mutex::new(node))),
            }
        }
        self.len += 1;
    }
    pub fn pop_front(&mut self) -> Option<T> {
        let node = match self.head.take() {
            Some(node) => {
                let node = node.lock().unwrap();
                self.head = node.next.clone();
                Some(node.val.clone())
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
            Some(node) => Some(node.lock().unwrap().val.clone()),
            None => None,
        };
        if self.len == 1 {
            self.head = None
        }
        self.len = (self.len.max(1) - 1).max(0);
        node
    }
    pub fn delete(&mut self, node: Arc<Mutex<Node<T>>>) {
        let node = node.lock().unwrap();
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
                next.lock().unwrap().prev = Some(prev.clone());
                match prev.upgrade() {
                    Some(prev) => prev.lock().unwrap().next = Some(next.clone()),
                    None => self.head = Some(next.clone()),
                }
            }
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

pub struct DoubleLinkedListIter<T> {
    current: Option<Arc<Mutex<Node<T>>>>,
}

impl<T: Clone> Iterator for DoubleLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_arc) = self.current.take() {
            let node = node_arc.lock().unwrap();
            self.current = node.next.clone();
            Some(node.val.clone())
        } else {
            None
        }
    }
}

impl<T: Clone> DoubleLinkedList<T> {
    pub fn iter(&self) -> DoubleLinkedListIter<T> {
        DoubleLinkedListIter {
            current: self.head.clone(),
        }
    }
}

#[test]
fn test_push_and_pop_front() {
    let mut list = DoubleLinkedList::new();

    list.push_front(Arc::new(Mutex::new(Node::new(1))));
    list.push_front(Arc::new(Mutex::new(Node::new(2))));
    list.push_front(Arc::new(Mutex::new(Node::new(3))));

    assert_eq!(list.len(), 3);

    if let Some(node) = list.pop_front() {
        let val = node.lock().unwrap().val.clone();
        assert_eq!(val, 3);
    } else {
        panic!("Expected Some(3) but got None");
    }
    println!("{:?}", &list);

    if let Some(node) = list.pop_front() {
        let val = node.lock().unwrap().val.clone();
        assert_eq!(val, 2);
    } else {
        panic!("Expected Some(2) but got None");
    }

    if let Some(node) = list.pop_front() {
        let val = node.lock().unwrap().val.clone();
        assert_eq!(val, 1);
    } else {
        panic!("Expected Some(1) but got None");
    }

    assert!(list.pop_front().is_none());
    assert_eq!(list.len(), 0);
}

#[test]
fn test_multithreaded_push_and_pop() {
    use std::thread;

    let list = Arc::new(Mutex::new(DoubleLinkedList::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let list_clone = Arc::clone(&list);
        handles.push(thread::spawn(move || {
            let node = Arc::new(Mutex::new(Node::new(i)));
            let mut list = list_clone.lock().unwrap();
            list.push_front_rc(node);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(list.lock().unwrap().len(), 10);

    let mut pop_handles = vec![];
    for _ in 0..10 {
        let list_clone = Arc::clone(&list);
        pop_handles.push(thread::spawn(move || {
            let mut list = list_clone.lock().unwrap();
            list.pop_front()
        }));
    }

    let mut results = vec![];
    for handle in pop_handles {
        if let Ok(result) = handle.join() {
            if result.is_some() {
                results.push(result.unwrap());
            }
        }
    }

    assert_eq!(results.len(), 10);
    assert_eq!(list.lock().unwrap().len(), 0);
}
