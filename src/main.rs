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
   let trefoil = link::Link::new(vec![
                 link::cross::X::new(1,5,2,4), 
                 link::cross::X::new(5,3,6,2),
                 link::cross::X::new(3,1,4,6)]);

   println!("{}", trefoil.jones());
} 


