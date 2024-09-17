fn main() {
    let input = [4, 3, 4];
    let output = Triangle::build(input).unwrap();
    assert!(output.is_isosceles());
}

pub struct Triangle {
    a: u64,
    b: u64,
    c: u64
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if !sides.iter().all(|d| *d > 0) {
            return None;
        }

        let [a, b, c] = sides;

        if !Triangle::sides_are_valid(a, b, c) {
            return None;
        }

        Some(Triangle { a, b, c })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        let mut equal_sides = 1;

        if self.a == self.b {
            equal_sides += 1;
        }

        if self.a == self.c {
            equal_sides += 1;
        }

        if self.b == self.c {
            equal_sides += 1;
        }

        if equal_sides > 3 {
            equal_sides = 3;
        }

        equal_sides >= 2
    }

    fn sides_are_valid(a: u64, b: u64, c: u64) -> bool {
        (a < b + c) && (b < a + c) && (c < a + b)    
    }
}
