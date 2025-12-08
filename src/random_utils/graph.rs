pub struct Dsu {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
    pub components: usize,
}

impl Dsu {
    #[inline]
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            components: n,
        }
    }

    #[inline]
    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;

        while self.parent[root] != root {
            root = self.parent[root];
        }

        while self.parent[x] != x {
            let next = self.parent[x];
            self.parent[x] = root;
            x = next;
        }

        root
    }

    #[inline]
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find(x);
        let ry = self.find(y);

        if rx == ry {
            return false;
        }

        if self.size[rx] < self.size[ry] {
            self.parent[rx] = ry;
            self.size[ry] += self.size[rx];
        } else {
            self.parent[ry] = rx;
            self.size[rx] += self.size[ry];
        }

        self.components -= 1;

        true
    }
}
