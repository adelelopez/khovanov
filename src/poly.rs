use std::cmp;
use std::fmt;
use std::num;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub; 

pub struct Polynomial {
   // Polynomials are represented as a vector of i32egers
   // terms[i] is the coefficient corresponding to q^(shift + i)
   terms: Vec<i32>,
   shift: i32, 
}

fn get_index_or_zero(a: i32, t: &Vec<i32>) -> i32 {
   if a >= 0 && a < t.len() as i32 { t[a as usize] } else { 0 }
}

pub fn new(terms: Vec<i32>, shift: i32) -> Polynomial {
   Polynomial {
      terms: terms,
      shift: shift,
   }
}

impl Polynomial {

   pub fn degree(&self) -> i32 {
      self.shift + self.terms.len() as i32
   }

   pub fn shift(&self, shift: i32) -> Polynomial {
      Polynomial {
         terms: self.terms.clone(),
         shift: self.shift.clone() + shift,
      }
   }

   pub fn pow(&self, exp: usize) -> Polynomial {
      
      let mut ret = new(vec![1],0);
      for _ in range(0,exp) {
         let p = new(self.terms.clone(),self.shift.clone());
         ret = ret * p;
      }
      ret
   }
}

impl fmt::String for Polynomial {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let shift = self.shift;
      let mut i = shift;
      for t in self.terms.iter() {
         // only write if coefficient is non-zero
         if *t != 0 {
            // write + or - between the terms depending on the sign
            // and only for terms after the first one
            if i > shift && *t < 0 { try!(write!(f," - ")) }
            else if i > shift && *t > 0 { try!(write!(f," + ")) }
            else if i == shift && *t < 0 { try!(write!(f,"-")) }

            // don't write the coefficient if it is +/- 1
            // unless it is the constant term
            if num::SignedInt::abs(*t) != 1 || i == 0 { 
               try!(write!(f,"{}", num::SignedInt::abs(*t)))
            }
            // use 'q' as the variable
            if i == 1 { try!(write!(f,"q")) }
            else if i != 0 { try!(write!(f,"q^{}",i)) }      
         }
         i += 1;
      }
      // if there were no terms to write, write 0
      if i == shift { try!(write!(f,"0")) }
      Ok(())
   }
}

// fn foo<'r, T>(f: fn(i32) -> T) -> iter::Map<'r, i32, T, iter::Range<i32>> {
//     range(0, 1).map(|t| {
//         f(t)
//     })
// }

// macro_rules! linear_align(
//    ($lhs:ident, $rhs:ident, $f:expr) => (
//       let shift = cmp::min(lhs.shift, rhs.shift); 
//       let degree = cmp::max::<i32>(lhs.degree(), rhs.degree());
//       range(0i, degree - shift).map(|t| { 
//          f(get_index_or_zero(t + shift - lhs.shift, &lhs.terms),
//            get_index_or_zero(t + shift - rhs.shift, &rhs.terms))
//          } )
//    )     
// )

// fn linear_align<'r,T>(lhs: &'r Polynomial, rhs: &'r Polynomial, f: |i32, i32| -> T) -> iter::Map<'r, i32, T, iter::Range<i32>> {
//    let shift = cmp::min(lhs.shift, rhs.shift); 
//    let degree = cmp::max::<i32>(lhs.degree(), rhs.degree());
//    range(0i, degree - shift).map(|t| { 
//          f(get_index_or_zero(t + shift - lhs.shift, &lhs.terms),
//            get_index_or_zero(t + shift - rhs.shift, &rhs.terms))
//          } )
// }

impl PartialEq for Polynomial {
   fn eq(&self, rhs: &Polynomial) -> bool {
      // shift is the shift of the sum
      let shift = cmp::min(self.shift, rhs.shift);
      // degree is the degree of the sum
      let degree = cmp::max::<i32>(self.degree(), rhs.degree());
      // so the difference degree - shift is the total 32 of the sum
      let value = range(0, degree - shift).map(|t| {
            get_index_or_zero(t + shift - self.shift, &self.terms) ==
            get_index_or_zero(t + shift - rhs.shift, &rhs.terms)
         } ).fold(true, |value, x| value && x);
      value
   }
}

impl Neg for Polynomial {
   type Output = Polynomial;
   fn neg(self) -> Polynomial { 
      let vec = self.terms.iter().map(|&x| -x).collect::<Vec<i32>>();
      new(vec, self.shift) 
   }
}

impl Add for Polynomial {
   type Output = Polynomial;
   fn add(self, rhs: Polynomial) -> Polynomial {  
      let shift = cmp::min(self.shift, rhs.shift); 
      let degree = cmp::max::<i32>(self.degree(), rhs.degree());
      let vec = range(0, degree - shift).map(|t| {
            get_index_or_zero(t + shift - self.shift, &self.terms) +
            get_index_or_zero(t + shift - rhs.shift, &rhs.terms)
         } ).collect::<Vec<i32>>();
      new(vec, shift)
   }
}

impl Sub<Polynomial> for Polynomial {
   type Output = Polynomial;
   fn sub(self, rhs: Polynomial) -> Polynomial {  
      let shift = cmp::min(self.shift, rhs.shift); 
      let degree = cmp::max::<i32>(self.degree(), rhs.degree());
      let vec = range(0, degree - shift).map(|t| {
            get_index_or_zero(t + shift - self.shift, &self.terms) -
            get_index_or_zero(t + shift - rhs.shift, &rhs.terms)
         } ).collect::<Vec<i32>>();
      new(vec, shift)
   }
}

impl Mul<Polynomial> for Polynomial {
   type Output = Polynomial;
   fn mul(self, rhs: Polynomial) -> Polynomial {
      let mut shift = self.shift + rhs.shift;
      let mut product = new(Vec::new(), shift);
      for s in self.terms.iter() {
         let vec = rhs.terms.iter().map(|&x| x * *s).collect::<Vec<i32>>(); 
         product = product + new(vec, shift);
         shift += 1;
      }
      product
   }
}
