mod poly;

#[cfg(not(test))]
fn main() {
   let p = poly::Polynomial {
      degree_shift: 0,
      terms: vec![1i, 0, 1],
   };

   let q = poly::Polynomial {
      degree_shift: 0,
      terms: vec![3i],
   };

   println!("{}", p * q);
} 


   