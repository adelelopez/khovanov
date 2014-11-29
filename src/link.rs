use set_union;

mod cross {
   pub struct X {
      i: uint,
      j: uint, 
      k: uint,
      l: uint,
      sign: int,
   }

   pub fn new(i: uint, j: uint, k: uint, l: uint) -> X {
      X {
         i: i,
         j: j,
         k: k,
         l: l,
         sign: 0,
      }
   }

   impl X {
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

struct Link {
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
         if x.get_sign() == 1 {
            np = np + 1 
         } else {
            nm = nm + 1
         };
      }
      Link {
         np: np,
         nm: nm,
         edges: v.len(),
         crosses: v,
      }
   }

   // pub fn count_loops(&self, smoothing: uint) -> uint {
   //    let s = set_union::new(self.edges);
   //    for x in self.crosses.iter() {

   //    }
   //    0u
   // }
}