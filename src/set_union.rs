pub struct SetUnion {
   parents: Vec<uint>,
   sizes: Vec<uint>,
   components: uint,
}

pub fn new(size: uint) -> SetUnion {
   SetUnion {
      parents: range(0,size).collect::<Vec<uint>>(),
      sizes: Vec::from_fn(size, |_| 1u),
      components: if size == 0 { 1 } else { size },
   }
}

impl SetUnion {
   pub fn find(&mut self, y: uint) -> uint {
      let mut x = y;
      while x != self.parents[x] {
         // compress path
         self.parents[x] = self.parents[self.parents[x]];
         x = self.parents[x];
      }
      return x
   }

   pub fn merge(&mut self, a: uint, b: uint) {
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

   pub fn num_components(&self) -> uint {
      self.components
   }
}