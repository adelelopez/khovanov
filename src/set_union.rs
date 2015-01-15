use std::iter;

// a basic disjoint set data structure, with fast lookup and fast union 
pub struct SetUnion {
   parents: Vec<usize>,
   sizes: Vec<usize>,
   components: usize,
}

pub fn new(size: usize) -> SetUnion {
   SetUnion {
      parents: range(0,size).collect::<Vec<usize>>(),
      sizes: iter::FromIterator::from_iter(iter::repeat(1).take(size)),
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
      return x
   }

   pub fn union(&mut self, a: usize, b: usize) {
      let r1 = self.find(a);
      let r2 = self.find(b);
      if r1 == r2 { return };

      // balance the sets
      let total_size = self.sizes[r1] + self.sizes[r2];
      if self.sizes[r1] >= self.sizes[r2] {
         self.sizes[r1] = total_size;
         self.parents[r2] = r1
      } else {
         self.sizes[r2] = total_size;
         self.parents[r1] = r2
      };
      self.components = self.components - 1;
   }

   pub fn num_components(&self) -> usize {
      self.components
   }
}