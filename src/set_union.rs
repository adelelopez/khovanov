use std::ops::Mul;

// a basic disjoint set data structure, with fast lookup and fast union
#[derive(Clone, Debug)]
pub struct SetUnion {
    parents: Vec<usize>,
    structure: Vec<(usize, usize)>, // (verticies, edges)
    components: usize,
}

pub fn new(size: usize) -> SetUnion {
    SetUnion {
        parents: (0..size).collect(),
        structure: (0..size).map(|_| (1, 0)).collect(),
        components: size,
    }
}

impl SetUnion {
    pub fn find(&mut self, y: usize) -> usize {
        let mut x = y;
        while x != self.parents[x] {
            // compress path
            self.parents[x] = self.parents[self.parents[x]];
            x = self.parents[x];
        }
        return x;
    }

    pub fn connect(&mut self, a: usize, b: usize) {
        let r1 = self.find(a);
        let r2 = self.find(b);
        if r1 == r2 {
            self.structure[r1] = (self.structure[r1].0, self.structure[r1].1 + 1);
            return;
        };

        // balance the sets
        let combined_structure = (self.structure[r1].0 + self.structure[r2].0,
                                  self.structure[r1].1 + self.structure[r2].1 + 1);
        if self.structure[r1] >= self.structure[r2] {
            self.structure[r1] = combined_structure;
            self.parents[r2] = r1
        } else {
            self.structure[r2] = combined_structure;
            self.parents[r1] = r2
        };
        self.components = self.components - 1;
    }

    pub fn insert(&mut self, genus: usize) {
        let idx = self.parents.len();
        self.parents.push(idx);
        self.structure.push((1, genus))
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    pub fn num_components(&self) -> usize {
        self.components
    }

    pub fn size(&self) -> usize {
        self.parents.len()
    }
}

impl Mul for SetUnion {
    type Output = SetUnion;
    fn mul(self, rhs: SetUnion) -> SetUnion {
        let mut parents = self.parents.clone();
        parents.extend(rhs.parents.into_iter().map(|i| i + self.parents.len()));
        let mut structure = self.structure.clone();
        structure.append(&mut rhs.structure.clone());
        SetUnion {
            parents: parents,
            structure: structure,
            components: self.components + rhs.components,
        }
    }
}
