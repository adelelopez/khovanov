use std::cmp;
use std::fmt;
use std::num;
pub struct Polynomial {
   // Polynomials are represented as a vector of integers
   // terms[i] is the coefficient corresponding to q^(shift + i)
   terms: Vec<int>,
   shift: int, 
}

pub fn new(terms: Vec<int>, shift: int) -> Polynomial {
   Polynomial {
      terms: terms,
      shift: shift,
   }
}

fn get_index_or_zero(a: int, t: &Vec<int>) -> int {
   if a >= 0 && a < t.len() as int { t[a as uint] } else { 0 }
}


impl fmt::Show for Polynomial {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let shift = self.shift;
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
            // use 'q' as the variable
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

impl PartialEq for Polynomial {
   fn eq(&self, rhs: &Polynomial) -> bool {
      // shift is the shift of the sum
      let shift = cmp::min(self.shift, rhs.shift);
      // degree is the degree of the sum
      let degree = cmp::max::<int>(self.shift + self.terms.len() as int, 
                            rhs.shift + rhs.terms.len() as int);
      // so the difference degree - shift is the total size of the sum
   
      let value = range(0i, degree - shift).map(|t| {
            get_index_or_zero(t + shift - self.shift, &self.terms) ==
            get_index_or_zero(t + shift - rhs.shift, &rhs.terms)
         } ).fold(true, |value, x| value && x);
      value
   }
}

impl Neg<Polynomial> for Polynomial {
   fn neg(&self) -> Polynomial { 
      let vec = self.terms.iter().map(|&x| -x).collect::<Vec<int>>();
      new(vec, self.shift) 
   }
}


impl Add<Polynomial, Polynomial> for Polynomial {
   fn add(&self, rhs: &Polynomial) -> Polynomial {  
      let shift = cmp::min(self.shift, rhs.shift); 
      let degree = cmp::max::<int>(self.shift + self.terms.len() as int, 
                            rhs.shift + rhs.terms.len() as int);

      let vec = range(0i, degree - shift).map(|t| {
            get_index_or_zero(t + shift - self.shift, &self.terms) +
            get_index_or_zero(t + shift - rhs.shift, &rhs.terms)
         } ).collect::<Vec<int>>();
      new(vec, shift)
   }
}

impl Sub<Polynomial, Polynomial> for Polynomial {
   fn sub(&self, rhs: &Polynomial) -> Polynomial {  
      let shift = cmp::min(self.shift, rhs.shift); 
      let degree = cmp::max::<int>(self.shift + self.terms.len() as int, 
                            rhs.shift + rhs.terms.len() as int);
      
      let vec = range(0i, degree - shift).map(|t| {
            get_index_or_zero(t + shift - self.shift, &self.terms) -
            get_index_or_zero(t + shift - rhs.shift, &rhs.terms)
         } ).collect::<Vec<int>>();
      new(vec, shift)
   }
}

impl Mul<Polynomial, Polynomial> for Polynomial {
   fn mul(&self, rhs: &Polynomial) -> Polynomial {
      let mut shift = self.shift + rhs.shift;
      let mut p = new(Vec::new(), shift);
      for s in self.terms.iter() {
         let vec = rhs.terms.iter().map(|&x| x * *s).collect::<Vec<int>>(); 
         p = p + new(vec, shift);
         shift += 1;
      }
      p
   }
}


