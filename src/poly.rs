use std::cmp;
use std::fmt;
use std::num;
pub struct Polynomial {
   // Polynomials are represented as a vector of integers
   // terms[i] is the coefficient corresponding to q^(degree_shift + i)
   degree_shift: int, 
   terms: Vec<int>,
}

pub fn new(terms: Vec<int>, shift: int) -> Polynomial {
   Polynomial {
      degree_shift: shift,
      terms: terms,
   }
}

fn bounded(a: int, bound: uint) -> Option<uint> {
   if a >= 0 && a < bound as int { Some(a as uint) } else { None }
}

impl PartialEq for Polynomial {
   fn eq(&self, other: &Polynomial) -> bool {
      for t in range(0, self.terms.len()) {
         let i = t as int + self.degree_shift - other.degree_shift;
         let a = match bounded(i, other.terms.len()) {
            Some(num)   => other.terms[num],
            None        => 0,
         };
         if a != self.terms[t] { return false; }
      }
      true
   }
}

impl fmt::Show for Polynomial {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let shift = self.degree_shift;
      let mut i = shift;
      for t in self.terms.iter() {
         // only write if coefficient is non-zero
         if *t != 0i {
            // write + or - between the terms depending on the sign
            // and only for terms after the first one
            if i > shift && *t < 0i { try!(write!(f," - ")) }
            else if i > shift && *t > 0i { try!(write!(f," + ")) }
            else if i == shift && *t < 0i { try!(write!(f,"-")) }

            // don't write the coefficient if it is +/- 1
            // unless it is the constant term
            if num::SignedInt::abs(*t) != 1 || i == 0i { 
               try!(write!(f,"{}", num::SignedInt::abs(*t)))
            }

            if i == 1 { try!(write!(f,"q")) }
            else if i != 0 { try!(write!(f,"q^{}",i)) }      
         }
         i+=1;
      }
      // if there were no terms to write, write 0
      if i == shift { try!(write!(f,"0")) }
      Ok(())
   }
}

impl Neg<Polynomial> for Polynomial {
   fn neg(&self) -> Polynomial { 
      let mut vec = Vec::new();
      for t in self.terms.iter() {
         vec.push(- *t);
      }
      new(vec, self.degree_shift) 
   }
}


impl Add<Polynomial, Polynomial> for Polynomial {
   fn add(&self, rhs: &Polynomial) -> Polynomial {
      // shift is the degree_shift of the sum
      let shift = cmp::min(self.degree_shift, rhs.degree_shift);
      // degree is the degree of the sum
      let degree = cmp::max::<int>(self.degree_shift + self.terms.len() as int, 
                            rhs.degree_shift + rhs.terms.len() as int);
      // so the difference degree - shift is the total size of the sum
      let mut vec: Vec<int> = Vec::new();
      vec.reserve((degree - shift) as uint);
  
      for t in range(0, degree - shift) {
         let ai = t + shift - self.degree_shift;
         let bi = t + shift - rhs.degree_shift;
         let a = match bounded(ai, self.terms.len()) {
            Some(num)   => self.terms[num],
            None        => 0,
         };
         let b = match bounded(bi, rhs.terms.len()) {
            Some(num)   => rhs.terms[num],
            None        => 0,
         };       
         vec.push(a+b);
      }
      new(vec, shift)
   }
}

impl Sub<Polynomial, Polynomial> for Polynomial {
   fn sub(&self, rhs: &Polynomial) -> Polynomial {
      - *rhs + *self
   }
}


impl Mul<Polynomial, Polynomial> for Polynomial {
   fn mul(&self, rhs: &Polynomial) -> Polynomial {
      let mut shift = self.degree_shift + rhs.degree_shift;
      let mut p = new(Vec::new(), shift);
       
      for s in self.terms.iter() {
         let vec = rhs.terms.iter().map(|&x| x * *s).collect::<Vec<int>>(); 
         p = p + new(vec, shift);
         shift += 1;
      }
      p
   }
}


