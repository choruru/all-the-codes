use std::{cmp::Ord, fmt::Debug};

pub struct PriorityQueue<T: Ord + Debug> {
    data: Vec<T>,
}

impl<T: Ord + Debug> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { data: vec![] }
    }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
        if self.data.len() == 1 {
            return;
        }

        let mut curr = self.data.len() - 1;
        while curr > 0 {
            let parent = (curr - 1) / 2;
            if self.data[curr] > self.data[parent] {
                break;
            }
            self.data.swap(curr, parent);
            curr = parent;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.len() {
            0 => None,
            1 => self.data.pop(),
            len => {
                self.data.swap(0, len - 1);
                let ret = self.data.pop();

                let mut parent = 0;
                let len = self.data.len();
                while 2 * parent + 1 < len {
                    let left = 2 * parent + 1;
                    let right = left + 1;
                    let smaller = if right >= len || self.data[left] < self.data[right] {
                        left
                    } else {
                        right
                    };
                    if self.data[parent] < self.data[smaller] {
                        break;
                    }
                    self.data.swap(parent, smaller);
                    parent = smaller;
                }

                ret
            }
        }
    }
}

mod tests {
    #[test]
    fn it_works() {
        let mut pque = super::PriorityQueue::new();
        pque.push(3);
        pque.push(5);
        pque.push(1);
        pque.push(2);
        pque.push(4);
        pque.push(3);

        assert_eq!(pque.pop(), Some(1));
        assert_eq!(pque.pop(), Some(2));
        assert_eq!(pque.pop(), Some(3));
        assert_eq!(pque.pop(), Some(3));
        assert_eq!(pque.pop(), Some(4));
        assert_eq!(pque.pop(), Some(5));
        assert_eq!(pque.pop(), None);
        assert_eq!(pque.pop(), None);
    }
}
