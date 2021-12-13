use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

struct Node<T: Clone> {
    val: Box<T>,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Clone> Node<T> {
    fn new(val: T) -> Rc<RefCell<Node<T>>> {
        let node = Rc::new(RefCell::new(Node {
            val: Box::new(val),
            prev: None,
            next: None,
        }));
        node.borrow_mut().prev = Some(node.clone());
        node.borrow_mut().next = Some(node.clone());
        node
    }
}

struct DoublyLinkedList<T: Clone> {
    head: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            head: None,
            size: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        let node = Node::new(val);
        match self.head.as_ref() {
            None => {
                self.head = Some(node.clone());
                node.borrow_mut().prev = Some(node.clone());
                node.borrow_mut().next = Some(node.clone());
            }
            Some(h) => {
                let last = h.borrow().prev.as_ref().unwrap().clone();
                last.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(last.clone());
                node.borrow_mut().next = Some(h.clone());
                h.borrow_mut().prev = Some(node.clone());
            }
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        if self.head.is_none() {
            return None
        }

        let head = self.head.clone().unwrap();
        let new_head = head.borrow().next.as_ref().unwrap().clone();
        let last = head.borrow().prev.as_ref().unwrap().clone();
        new_head.borrow_mut().prev = Some(last.clone());
        last.borrow_mut().next = Some(new_head.clone());
        head.borrow_mut().next = None;
        head.borrow_mut().prev = None;
        self.head = Some(new_head);
        self.size -= 1;
        Some(head)
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut vals = vec![];
        let mut curr = self.head.as_ref().unwrap().clone();
        for _ in 0..self.size {
            vals.push(curr.borrow().val.clone());
            let new = curr.borrow().next.as_ref().unwrap().clone();
            curr = new;
        }
        write!(f, "{:?}", vals)
    }
}

mod tests {
    #[test]
    fn it_works() {
        let mut list = super::DoublyLinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.pop();
        list.push(4);
        list.pop();
        assert_eq!(format!("{:?}", list), "[3, 4]");
    }
}
