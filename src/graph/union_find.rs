#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            x
        } else {
            let parent = self.parent[x];
            self.parent[x] = self.find(parent);
            self.parent[x]
        }
    }
    
    fn unite(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);
        
        if px == py { return; }
        
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
        } else {
            self.parent[py] = px;
            if self.rank[px] == self.rank[py] {
                self.rank[px] += 1;
            }
        }
    }
    
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn main() {
    let mut uf = UnionFind::new(5);
    
    uf.unite(0, 2);
    uf.unite(3, 4);
    uf.unite(4, 2);
    
    println!("0 and 3 are in the same set: {}", uf.same(0, 3));
    println!("1 and 3 are in the same set: {}", uf.same(1, 3));
}
