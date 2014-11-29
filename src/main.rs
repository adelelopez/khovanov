#![feature(macro_rules)]
mod poly;
#[allow(dead_code)]
mod set_union;
#[allow(dead_code)]
mod link;
#[cfg(test)]
mod tests;


#[cfg(not(test))]
fn main() {
   let p = poly::new(vec![1,0,1],-1);

   println!("{}", p * p);
} 


