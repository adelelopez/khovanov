mod poly;


#[cfg(not(test))]
fn main() {
   let p = poly::Polynomial {
      degree_shift: -1,
      terms: vec![1i, 2, 0, -3],
   };

   let q = poly::Polynomial {
      degree_shift: 4,
      terms: vec![2i, 4, 0, -6],
   };

   println!("{}", p + q);
} 


   