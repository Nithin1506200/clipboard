use std::sync::{Arc, RwLock, Weak};
#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Option<Arc<RwLock<Node<T>>>>,
    prev: Option<Weak<RwLock<Node<T>>>>,
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
impl<T: Clone> Node<T> {
    pub fn set_value(&mut self, value: T) {
        self.val = value
    }
}
#[derive(Debug)]
pub struct DoubleLinkedList<T> {
    head: Option<Arc<RwLock<Node<T>>>>,
    tail: Option<Arc<RwLock<Node<T>>>>,
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
    pub fn peak_front(&self) -> Option<Arc<RwLock<Node<T>>>> {
        self.head.clone()
    }
    pub fn push_front_rc(&mut self, node: Arc<RwLock<Node<T>>>) {
        {
            let mut node = node.write().unwrap();
            node.prev = None;
            node.next = None
        }
        if self.len == 0 {
            self.head = Some(node.clone());
            self.tail = Some(node.clone());
        } else {
            match self.head.take() {
                Some(ptr) => {
                    let mut node_ = node.write().unwrap();
                    node_.next = Some(ptr.clone());
                    self.head = Some(node.clone());
                    ptr.write().unwrap().prev = Some(Arc::downgrade(&node));
                }
                None => self.head = Some(node.clone()),
            }
        }
        self.len += 1;
    }
    pub fn push_front(&mut self, value: T) {
        let mut node = Node::new(value);
        if self.len == 0 {
            let n = Arc::new(RwLock::new(node));
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        } else {
            match self.head.take() {
                Some(ptr) => {
                    node.next = Some(ptr.clone());
                    let n = Arc::new(RwLock::new(node));
                    self.head = Some(n.clone());
                    ptr.write().unwrap().prev = Some(Arc::downgrade(&n));
                }
                None => self.head = Some(Arc::new(RwLock::new(node))),
            }
        }
        self.len += 1;
    }
    #[allow(dead_code)]
    pub fn push_back(&mut self, value: T) {
        let mut node = Node::new(value);
        if self.len == 0 {
            let n = Arc::new(RwLock::new(node));
            self.head = Some(n.clone());
            self.tail = Some(n.clone());
        } else {
            match self.tail.take() {
                Some(ptr) => {
                    node.prev = Some(Arc::downgrade(&ptr));
                    let n = Arc::new(RwLock::new(node));
                    self.tail = Some(n.clone());
                    ptr.write().unwrap().next = self.tail.clone();
                }

                None => self.tail = Some(Arc::new(RwLock::new(node))),
            }
        }
        self.len += 1;
    }
    #[allow(dead_code)]
    pub fn pop_front(&mut self) -> Option<T> {
        let node = match self.head.take() {
            Some(node) => {
                let node = node.write().unwrap();
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
            Some(node) => Some(node.read().unwrap().val.clone()),
            None => None,
        };
        if self.len == 1 {
            self.head = None
        }
        self.len = (self.len.max(1) - 1).max(0);
        node
    }
    pub fn delete(&mut self, node: Arc<RwLock<Node<T>>>) {
        let node = node.write().unwrap();
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
                next.write().unwrap().prev = Some(prev.clone());
                match prev.upgrade() {
                    Some(prev) => prev.write().unwrap().next = Some(next.clone()),
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
    current: Option<Arc<RwLock<Node<T>>>>,
}

impl<T: Clone> Iterator for DoubleLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node_arc) = self.current.take() {
            let node = node_arc.read().unwrap();
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

    list.push_front(Arc::new(RwLock::new(Node::new(1))));
    list.push_front(Arc::new(RwLock::new(Node::new(2))));
    list.push_front(Arc::new(RwLock::new(Node::new(3))));

    assert_eq!(list.len(), 3);

    if let Some(node) = list.pop_front() {
        let val = node.read().unwrap().val.clone();
        assert_eq!(val, 3);
    } else {
        panic!("Expected Some(3) but got None");
    }
    println!("{:?}", &list);

    if let Some(node) = list.pop_front() {
        let val = node.read().unwrap().val.clone();
        assert_eq!(val, 2);
    } else {
        panic!("Expected Some(2) but got None");
    }

    if let Some(node) = list.pop_front() {
        let val = node.read().unwrap().val.clone();
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

    let list = Arc::new(RwLock::new(DoubleLinkedList::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let list_clone = Arc::clone(&list);
        handles.push(thread::spawn(move || {
            let node = Arc::new(RwLock::new(Node::new(i)));
            let mut list = list_clone.write().unwrap();
            list.push_front_rc(node);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(list.read().unwrap().len(), 10);

    let mut pop_handles = vec![];
    for _ in 0..10 {
        let list_clone = Arc::clone(&list);
        pop_handles.push(thread::spawn(move || {
            let mut list = list_clone.write().unwrap();
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
    assert_eq!(list.read().unwrap().len(), 0);
}
