use poly;

#[test]
fn equality_over_degree_shifts () {
   let p = poly::new(vec![1, 2, 3, 4], 3);
   let q = poly::new(vec![0, 0, 0, 1, 2, 3, 4], 0);
   assert_eq!(p,q);
}

#[test]
fn poly_plus_zero () {
   let p = poly::new(vec![1, 2, 3, 4], 4);
   let zero = poly::new(vec![0], 0);
   assert_eq!(p.clone(), p.clone() + zero);
}

#[test]
fn twice_poly () {
   let p = poly::new(vec![1, 2, 3, 4], 0);
   let q = poly::new(vec![2, 4, 6, 8], 0);
   assert_eq!(p.clone() + p.clone(), q);
}

#[test]
fn additive_inverse () {
   let p = poly::new(vec![1, 2, 3, 4], 0);
   let q = poly::new(vec![-1, -2, -3, -4], 0);
   let zero = poly::new(vec![0], 4);
   assert_eq!(p + q, zero);
}

#[test]
fn subtract_from_self () {
   let p = poly::new(vec![1, 2, 3, 4], 0);
   let zero = poly::new(vec![0], 4);
   assert_eq!(p.clone() - p.clone(), zero);
}

#[test]
fn subtract_zero () {
   let p = poly::new(vec![1, 2, 3, 4], 3);
   let zero = poly::new(vec![0], 3);
   assert_eq!(p.clone() - zero, p.clone());
}

#[test]
fn double_negative () {
   let p = poly::new(vec![1, 2, 3, 4], 3);
   assert_eq!(- - p.clone(), p.clone());
}

#[test]
fn addition_subtraction () {
   let p = poly::new(vec![1, 2, 3, 4], 0);
   let q = poly::new(vec![2, 4, 6, 8, 7, 1], 0);
   assert_eq!(-p.clone() + q.clone(), q.clone() - p.clone());
}

#[test]
fn mult_by_zero () {
   let p = poly::new(vec![1, 2, 3, 4], 3);
   let zero = poly::new(vec![0], 0);
   assert_eq!(p * zero.clone(), zero.clone());
}

#[test]
fn mult_by_one () {
   let p = poly::new(vec![1, 2, 3, 4], -4);
   let one = poly::new(vec![1], 0);
   assert_eq!(p.clone() * one, p.clone());
}

#[test]
fn multiplication () {
   let p = poly::new(vec![3, 0, 3, 7], 0);
   let q = poly::new(vec![4, 5], 1);
   let pq = poly::new(vec![12, 15, 12, 43, 35], 1);
   assert_eq!(p * q, pq);
}

#[test]
fn test_shift () {
   let p = poly::new(vec![0, 1, 3, 7], 0);
   let q = poly::new(vec![1, 3, 7], 0);
   assert_eq!(p, q.shift(1));
}

#[test]
fn test_pow () {
   let p = poly::new(vec![0, 1, 3, 7], 0);
   let one = poly::new(vec![1], 0);

   assert_eq!(p.pow(0), one);
   assert_eq!(p.pow(1), p);
   assert_eq!(p.pow(2), p.clone() * p.clone());
   assert_eq!(p.pow(3), p.clone() * p.clone() * p.clone());
}
// link module tests

