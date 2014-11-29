use set_union;
use std::cmp;

pub mod cross {
   pub struct X {
      pub i: uint,
      pub j: uint, 
      pub k: uint,
      pub l: uint,
      sign: int,
   }

   impl X {
      pub fn new(i: uint, j: uint, k: uint, l: uint) -> X {
         X {
            i: i,
            j: j,
            k: k,
            l: l,
            sign: 0,
         }
      }

      pub fn get_sign(&self) -> int {

         if self.sign == 0 {
            match (self.j, self.l) {
               (1,_)                 =>  1,
               (_,1)                 => -1,
               (j, l) if j + 1 == l  => -1,
               _                     =>  1,
            }
         } else {
            self.sign
         }
      }
   }
}

pub struct Link {
   np: uint,
   nm: uint,
   edges: uint,
   crosses: Vec<cross::X>,
}

impl Link {
   pub fn new(v: Vec<cross::X>) -> Link {
      let mut nm = 0u;
      let mut np = 0u;
      let mut edges = 0u;
      for x in v.iter() {
         edges = [x.i,x.j,x.k,x.l,edges].iter().fold(0u, |max, i| cmp::max(max,*i));
         if x.get_sign() == 1 {
            np = np + 1 
         } else {
            nm = nm + 1
         };
      }
      Link {
         np: np,
         nm: nm,
         edges: edges,
         crosses: v,
      }
   }

   pub fn count_loops(&self, smoothing: uint) -> uint {
      let mut s = set_union::new(self.edges);
      let mut i = 0;
      for x in self.crosses.iter() {
         if (smoothing & 1 << i) != 0 {
            s.merge(x.i - 1, x.l - 1);
            s.merge(x.j - 1, x.k - 1);
         } else {
            s.merge(x.i - 1, x.j - 1);
            s.merge(x.k - 1, x.l - 1);
         }
         i = i + 1;
      }
      s.num_components()
   }
}