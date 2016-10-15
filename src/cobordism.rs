use std::ops::Mul;
use set_union::SetUnion;
use set_union;

fn shift(v: &Vec<usize>, shift: usize) -> Vec<usize> {
    v.into_iter().map(|t| t + shift).collect()
}

#[derive(Clone, Debug)]
pub struct Smoothing {
    boundary: Vec<usize>,
    loops: Vec<usize>,
    pub curves: SetUnion,
}

impl Smoothing {
    pub fn new(boundary: Vec<usize>,
               loops: Vec<usize>,
               connections: Vec<(usize, usize)>)
               -> Smoothing {
        let mut curves: SetUnion = set_union::new(boundary.len() + loops.len());
        for i in connections {
            curves.connect(i.0, i.1);
        }
        Smoothing {
            boundary: boundary,
            loops: loops,
            curves: curves,
        }
    }

    pub fn size(&self) -> usize {
        self.boundary.len() + self.loops.len()
    }
}

impl Mul for Smoothing {
    type Output = Smoothing;
    fn mul(self, rhs: Smoothing) -> Smoothing {
        let mut boundary = self.boundary.clone();
        boundary.extend(rhs.boundary.into_iter().map(|b| b + self.curves.size()));
        let mut loops = self.loops.clone();
        loops.extend(rhs.loops.into_iter().map(|l| l + self.curves.size()));
        Smoothing {
            boundary: boundary,
            loops: loops,
            curves: self.curves * rhs.curves,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cobordism {
    boundary: Vec<usize>,
    top_loops: Vec<usize>,
    bottom_loops: Vec<usize>,
    components: SetUnion,
}

impl Cobordism {
    pub fn new(a: &Smoothing,
               b: &Smoothing,
               connections: Vec<(usize, usize)>,
               bubbles: Vec<usize>)
               -> Cobordism {
        if a.boundary.len() != b.boundary.len() {
            panic!()
        }
        let s = a.size();
        let mut components: SetUnion = a.curves.clone() * b.curves.clone();

        for b in bubbles {
            components.insert(b);
        }

        for i in 0..a.boundary.len() {
            if !components.connected(a.boundary[i], b.boundary[i] + s) {
                components.connect(a.boundary[i], b.boundary[i] + s)
            }
        }

        for (x, y) in connections {
            components.connect(x, y + s);
        }

        Cobordism {
            boundary: a.boundary.clone(),
            top_loops: a.loops.clone(),
            bottom_loops: shift(&b.loops, s),
            components: components,
        }
    }
}

impl Mul for Cobordism {
    type Output = Cobordism;
    fn mul(self, rhs: Cobordism) -> Cobordism {
        let s = self.components.size();
        let boundary = self.boundary.clone();
        let mut components: SetUnion = self.components.clone() * rhs.components.clone();

        // connect boundaries again
        for (x, y) in self.boundary.into_iter().zip(rhs.boundary.into_iter()) {
            if !components.connected(x, y + s) {
                components.connect(x, y + s)
            };
        }

        for (x, y) in self.bottom_loops.into_iter().zip(rhs.top_loops.into_iter()) {
            components.connect(x, y + s);
        }

        Cobordism {
            boundary: boundary,
            top_loops: self.top_loops.clone(),
            bottom_loops: shift(&rhs.bottom_loops, s),
            components: components,
        }
    }
}
