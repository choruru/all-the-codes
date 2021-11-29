use std::cmp::Ordering;

pub struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        let mut parent = vec![0; size];
        for i in 0..size {
            parent[i] = i;
        }
        DisjointSet {
            parent: parent,
            rank: vec![0; size],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            self.parent[x] = self.root(self.parent[x]);
            self.parent[x]
        }
    }

    pub fn merge(&mut self, mut x: usize, mut y: usize) {
        x = self.root(x);
        y = self.root(y);
        if x == y {
            return;
        }
        match self.rank[x].cmp(&self.rank[y]) {
            Ordering::Less => self.parent[x] = y,
            Ordering::Greater => self.parent[x] = y,
            Ordering::Equal => {
                self.parent[x] = y;
                self.rank[x] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
}

mod tests {
    #[test]
    fn it_works() {
        let mut set = super::DisjointSet::new(5);
        set.merge(1, 4);
        set.merge(2, 3);
        assert_eq!(set.same(1, 2), false);
        assert_eq!(set.same(3, 4), false);
        assert_eq!(set.same(1, 4), true);
        assert_eq!(set.same(3, 2), true);
        set.merge(1, 3);
        assert_eq!(set.same(2, 4), true);
        assert_eq!(set.same(3, 0), false);
        set.merge(0, 4);
        assert_eq!(set.same(0, 2), true);
        assert_eq!(set.same(3, 0), true);
    }
}
