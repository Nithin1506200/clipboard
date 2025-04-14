use crate::data::*;
use crate::double_linked_list::{DoubleLinkedList, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
#[derive(Debug)]
pub struct Lru {
    list: DoubleLinkedList<Data>,
    hash: HashMap<String, Weak<RefCell<Node<Data>>>>,
    size: usize,
}
impl Lru {
    pub fn new(size: usize) -> Self {
        Lru {
            list: DoubleLinkedList::new(),
            hash: HashMap::new(),
            size,
        }
    }
    pub fn pop(&mut self) -> Option<Data> {
        if let Some(data) = self.list.pop_back() {
            self.hash.remove(&data.hash());
            Some(data)
        } else {
            None
        }
    }
    pub fn insert(&mut self, data: String) {
        let data: Data = Data::from(data);
        let hash = data.hash();
        match self.hash.get(&hash).map_or(None, |e| e.upgrade()) {
            Some(node) => {
                self.list.delete(node.clone());
                self.list.push_front_rc(node.clone());
            }
            None => {
                self.list.push_front(data);
                self.hash
                    .insert(hash, Rc::downgrade(&self.list.peak_front().unwrap()));
            }
        }
        while self.len() > self.size {
            self.pop();
        }
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut lru = Lru::new(3);
        lru.insert("data".into());
        assert_eq!(lru.len(), 1);
        lru.insert("data".into());
        assert_eq!(lru.len(), 1);
        lru.insert("nithin@gmail.com".into());
        assert_eq!(lru.len(), 2);
        assert_eq!(
            lru.list.peak_front().unwrap().borrow().val().val(),
            String::from("nithin@gmail.com")
        );
        lru.insert("9449352583".into());
        assert_eq!(lru.len(), 3);
        assert_eq!(
            lru.list.peak_front().unwrap().borrow().val().val(),
            String::from("9449352583")
        );
        lru.insert("94493525832".into());
        assert_eq!(lru.len(), 3);
        assert_eq!(
            lru.list.peak_front().unwrap().borrow().val().val(),
            String::from("94493525832")
        );
        // println!("{:#?}", lru)
    }
}
