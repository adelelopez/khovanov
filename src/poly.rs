use std::cmp;
use std::fmt;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Debug)]
pub struct Polynomial {
    // Polynomials are represented as a vector of integers
    // terms with negative exponents are allowed, via the shift variable
    // terms[i] is the coefficient corresponding to q^(shift + i)
    terms: Vec<i32>,
    shift: i32,
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
        let mut ret = new(vec![1], 0);
        for _ in 0..exp {
            ret = ret * self.clone();
        }
        ret
    }

    pub fn coeff(&self, term: i32) -> i32 {
        let index = term - self.shift;
        match index >= 0 && index < self.terms.len() as i32 {
            true => self.terms[index as usize],
            false => 0,
        }
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let shift = self.shift;
        let mut exp = shift;
        let mut first = true;

        for coeff in &self.terms {
            // only write if coefficient is non-zero
            if *coeff != 0 {
                // write + or - between the terms depending on the sign
                // and only for terms after the first one
                match (first, *coeff > 0) {
                    (true, true) => {} // no initial sign for a positive first term
                    (true, false) => try!(write!(f, "-")),
                    (false, true) => try!(write!(f, " + ")),
                    (false, false) => try!(write!(f, " - ")),
                }
                first = false;

                // don't write the coefficient if it is +/- 1
                // unless it is the constant term
                if (*coeff).abs() != 1 || exp == 0 {
                    try!(write!(f, "{}", (*coeff).abs()))
                }
                // use 'q' as the variable
                if exp == 1 {
                    try!(write!(f, "q"))
                } else if exp != 0 {
                    try!(write!(f, "q^{}", exp))
                }
            }
            exp += 1;
        }
        // if there were no terms to write, write 0
        if exp == shift {
            try!(write!(f, "0"))
        }
        Ok(())
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, rhs: &Polynomial) -> bool {
        // shift is the shift of the sum
        let shift = cmp::min(self.shift, rhs.shift);
        // degree is the degree of the sum
        let degree = cmp::max::<i32>(self.degree(), rhs.degree());
        // so the difference is the total range of possibly non-zero terms
        let value = (0..degree - shift)
            .map(|t| self.coeff(t + shift) == rhs.coeff(t + shift))
            .fold(true, |value, x| value && x);
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
        let vec = (0..degree - shift)
            .map(|t| self.coeff(t + shift) + rhs.coeff(t + shift))
            .collect::<Vec<i32>>();
        new(vec, shift)
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Polynomial) -> Polynomial {
        let shift = cmp::min(self.shift, rhs.shift);
        let degree = cmp::max::<i32>(self.degree(), rhs.degree());
        let vec = (0..degree - shift)
            .map(|t| self.coeff(t + shift) - rhs.coeff(t + shift))
            .collect::<Vec<i32>>();
        new(vec, shift)
    }
}

impl Mul for Polynomial {
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
