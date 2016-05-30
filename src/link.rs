use set_union;
use poly;
use std::cmp;

pub mod cross {
   pub struct X {
      pub i: usize,
      pub j: usize, 
      pub k: usize,
      pub l: usize,
      sign: i32,
   }

   impl X {
      pub fn new(i: usize, j: usize, k: usize, l: usize) -> X {
         X {
            i: i,
            j: j,
            k: k,
            l: l,
            sign: 0,
         }
      }

      pub fn get_sign(&self) -> i32 {

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
   np: i32,
   nm: i32,
   edges: usize,
   crosses: Vec<cross::X>,
}

impl Link {
   pub fn new(v: Vec<cross::X>) -> Link {
      let mut nm = 0;
      let mut np = 0;
      let mut edges = 0;
      for x in v.iter() {
         edges = [x.i,x.j,x.k,x.l,edges].iter().fold(0, |max, i| cmp::max(max,*i));
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

   pub fn count_loops(&self, smoothing: usize) -> usize {
      let mut s = set_union::new(self.edges);
      let mut i = 0;
      for x in self.crosses.iter() {
         if (smoothing & 1 << i) != 0 {
            s.union(x.i - 1, x.l - 1);
            s.union(x.j - 1, x.k - 1);
         } else {
            s.union(x.i - 1, x.j - 1);
            s.union(x.k - 1, x.l - 1);
         }
         i = i + 1;
      }
      s.num_components()
   }

   pub fn jones(&self) -> poly::Polynomial {
      let v = poly::new(vec![1,0,1],-1);
      let n: usize = 1 << self.crosses.len();
      let mut ret: poly::Polynomial = poly::new(vec![0],0);
      let mut height: usize;
      let mut dim: usize;
      for i in 0..n {
         dim = self.count_loops(i);
         height = i.count_ones() as usize;
         ret = ret + if (height % 2) == 0 {
            v.pow(dim).shift(height as i32) 
         } else
         {
            -v.pow(dim).shift(height as i32) 
         };
      }
      if (self.nm % 2) == 0 {
         ret
      } else {
         -ret
      }.shift(self.np  - 2 * self.nm)
   }
}