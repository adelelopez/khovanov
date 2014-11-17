mod poly;

#[test]
fn equality_over_degree_shifts () {
   let p = poly::Polynomial {
      degree_shift: 3,
      terms: vec![1i, 2i, 3i, 4i],
   };
   let q = poly::Polynomial {
      degree_shift: 0,
      terms: vec![0i, 0i, 0i, 1i, 2i, 3i, 4i],
   };
   assert_eq!(p,q);
}

#[test]
fn poly_plus_zero () {
   let p = poly::Polynomial {
      degree_shift: 4,
      terms: vec![1i, 2i, 3i, 4i],
   };
   let zero = poly::Polynomial {
      degree_shift: 0,
      terms: vec![0i],
   };
   assert_eq!(p, p + zero);
}

#[test]
fn twice_poly () {
   let p = poly::Polynomial {
      degree_shift: 0,
      terms: vec![1i, 2i, 3i, 4i],
   };
   let q = poly::Polynomial {
      degree_shift: 0,
      terms: vec![2i, 4i, 6i, 8i],
   };
   assert_eq!(p + p, q);
}

#[test]
fn additive_inverse () {
    let p = poly::Polynomial {
      degree_shift: 0,
      terms: vec![1i, 2i, 3i, 4i],
   };
   let q = poly::Polynomial {
      degree_shift: 0,
      terms: vec![-1i, -2i, -3i, -4i],
   };
   let zero = poly::Polynomial {
      degree_shift: 4,
      terms: vec![0i],
   };
   assert_eq!(p + q, zero);
}

#[test]
fn subtract_from_self () {
    let p = poly::Polynomial {
      degree_shift: 0,
      terms: vec![1i, 2i, 3i, 4i],
   };
  
   let zero = poly::Polynomial {
      degree_shift: 4,
      terms: vec![0i],
   };
   assert_eq!(p - p, zero);
}

#[test]
fn subtract_zero () {
    let p = poly::Polynomial {
      degree_shift: 3,
      terms: vec![1i, 2i, 3i, 4i],
   };
   let zero = poly::Polynomial {
      degree_shift: 4,
      terms: vec![0i],
   };
   assert_eq!(p - zero, p);
}

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


   