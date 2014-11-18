#[test]
fn equality_over_degree_shifts () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], 3);

   let q = poly::new(vec![0i, 0i, 0i, 1i, 2i, 3i, 4i], 0);
   assert_eq!(p,q);
}

#[test]
fn poly_plus_zero () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], 4);
   let zero = poly::new(vec![0i], 0);
   assert_eq!(p, p + zero);
}

#[test]
fn twice_poly () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], 0);
   let q = poly::new(vec![2i, 4i, 6i, 8i], 0);
   assert_eq!(p + p, q);
}

#[test]
fn additive_inverse () {
    let p = poly::new(vec![1i, 2i, 3i, 4i], 0);
   let q = poly::new(vec![-1i, -2i, -3i, -4i], 0);
   let zero = poly::new(vec![0i], 4);
   assert_eq!(p + q, zero);
}

#[test]
fn subtract_from_self () {
    let p = poly::new(vec![1i, 2i, 3i, 4i], 0);
  
   let zero = poly::new(vec![0i], 4);
   assert_eq!(p - p, zero);
}

#[test]
fn subtract_zero () {
    let p = poly::new(vec![1i, 2i, 3i, 4i], 3);
   
   let zero = poly::new(vec![0i], 3);
   
   assert_eq!(p - zero, p);
}

#[test]
fn double_negative () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], 3);
   
   assert_eq!(- - p, p);
}

#[test]
fn addition_subtraction () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], 0);
   
   let q = poly::new(vec![2i, 4i, 6i, 8i, 7i, 1i], 0);
   
   assert_eq!(-p + q, q - p);
}

#[test]
fn mult_by_zero () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], 3);
   
   let zero = poly::new(vec![0i], 0);
   
   assert_eq!(p * zero, zero);
}


#[test]
fn mult_by_one () {
   let p = poly::new(vec![1i, 2i, 3i, 4i], -4);
   
   let one = poly::new(vec![1i], 0);
   
   assert_eq!(p * one, p);
}

#[test]
fn multiplication () {
   let p = poly::new(vec![3i, 0i, 3i, 7i], 0);
      
   let q = poly::new(vec![4i, 5i], 1);
   
   let pq = poly::new(vec![12i, 15i, 12i, 43i, 35i], 1);
   
   assert_eq!(p * q, pq);
}