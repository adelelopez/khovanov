mod poly;
#[allow(dead_code)]
mod set_union;
#[allow(dead_code)]
mod cobordism;
#[allow(dead_code)]
mod link;
#[cfg(test)]
mod tests;

use link::cross::X;

fn main() {
    let trefoil = link::Link::new(vec![X::new(1, 5, 2, 4), X::new(5, 3, 6, 2), X::new(3, 1, 4, 6)]);

    let figure_eight = link::Link::new(vec![X::new(4, 2, 5, 1),
                                            X::new(8, 6, 1, 5),
                                            X::new(6, 3, 7, 4),
                                            X::new(2, 7, 3, 8)]);

    let millett_unknot = link::Link::new(vec![X::new(1, 10, 2, 11),
                                              X::new(9, 2, 10, 3),
                                              X::new(3, 7, 4, 6),
                                              X::new(15, 5, 16, 4),
                                              X::new(5, 17, 6, 16),
                                              X::new(7, 14, 8, 15),
                                              X::new(8, 18, 9, 17),
                                              X::new(11, 18, 12, 19),
                                              X::new(19, 12, 20, 13),
                                              X::new(13, 20, 14, 1)]);

    let twist = link::Link::new(vec![X::new(1, 1, 2, 3), X::new(3, 2, 4, 4)]);

    println!("Trefoil: {}", trefoil.jones());
    println!("Figure eight: {}", figure_eight.jones());
    println!("Millet unknot: {}", millett_unknot.jones());
    println!("Twist: {}", twist.jones());

    let a = cobordism::Smoothing::new(vec![0, 1], vec![], vec![(0, 1)]);
    let b = cobordism::Smoothing::new(vec![0, 1], vec![2], vec![(0, 1)]);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", a.curves.clone() * b.curves.clone());
    println!("{:?}", cobordism::Cobordism::new(&a, &a, vec![], vec![]));
    let x = cobordism::Cobordism::new(&a, &b, vec![(0, 2)], vec![]);
    let y = cobordism::Cobordism::new(&b, &a, vec![(2, 1)], vec![]);
    println!("{:?}", x.clone() * y.clone());
    println!("{:?}", y * x);
}
