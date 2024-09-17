use std::iter;

fn main() {
    let input = &[
        (1, 2),
        (5, 3),
        (3, 1),
        (1, 2),
        (2, 4),
        (1, 6),
        (2, 3),
        (3, 4),
        (5, 6),
    ];

    println!("{:?}", chain(input));
}

struct AvailabilityTable {
    m: Vec<u8>,
}

impl AvailabilityTable {
    fn new() -> AvailabilityTable {
        AvailabilityTable {
            m: iter::repeat(0).take(6 * 6).collect(),
        }
    }

    fn get(&self, x: u8, y: u8) -> u8 {
        self.m[((x - 1) * 6 + (y - 1)) as usize]
    }

    fn set(&mut self, x: u8, y: u8, v: u8) {
        let m = &mut self.m[..];
        m[((x - 1) * 6 + (y - 1)) as usize] = v;
    }

    fn add(&mut self, x: u8, y: u8) {
        if x == y {
            let n = self.get(x, y);
            self.set(x, y, n + 1) // Along the diagonal
        } else {
            let m = self.get(x, y);
            self.set(x, y, m + 1);
            let n = self.get(y, x);
            self.set(y, x, n + 1);
        }
    }

    fn remove(&mut self, x: u8, y: u8) {
        if self.get(x, y) > 0 {
            if x == y {
                let n = self.get(x, y);
                self.set(x, y, n - 1)
            } else {
                let m = self.get(x, y);
                self.set(x, y, m - 1);
                let n = self.get(y, x);
                self.set(y, x, n - 1);
            }
        } else {
            panic!("Remove for 0 dominoes: ({:?}, {:?})", x, y)
        }
    }

    fn pop_first(&mut self, x: u8) -> Option<u8> {
        if self.get(x, x) > 0 {
            self.remove(x, x);
            return Some(x);
        }

        for y in 1..7 {
            if self.get(x, y) > 0 {
                self.remove(x, y);
                return Some(y);
            }
        }
        None
    }
}

pub fn chain(dominoes: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    match dominoes.len() {
        0 => Some(vec![]),
        1 => {
            if dominoes[0].0 == dominoes[0].1 {
                Some(vec![dominoes[0]])
            } else {
                None
            }
        }
        _ => {
            // First check if the total number of each amount of dots is even, if not it's not
            // possible to complete a cycle. This follows from that it's an Eulerian path.
            let mut v: Vec<u8> = vec![0, 0, 0, 0, 0, 0];
            // Keep the mutable borrow in a small scope here to allow v.iter().
            {
                let vs = &mut v[..];
                for dom in dominoes.iter() {
                    vs[dom.0 as usize - 1] += 1;
                    vs[dom.1 as usize - 1] += 1;
                }
            }
            for n in v.iter() {
                if n % 2 != 0 {
                    return None;
                }
            }
            let chain = chain_worker(dominoes);
            if chain.len() == dominoes.len() {
                Some(chain)
            } else {
                None
            }
        }
    }
}

fn chain_worker(dominoes: &[(u8, u8)]) -> Vec<(u8, u8)> {
    let mut doms = dominoes.to_vec();
    let first = doms.pop().unwrap();
    let mut t = AvailabilityTable::new();
    for dom in doms.iter() {
        t.add(dom.0, dom.1)
    }
    let mut v: Vec<(u8, u8)> = Vec::new();
    v.push(first);
    let mut n = first.1; // Number to connect to
    while let Some(m) = t.pop_first(n) {
        v.push((n, m));
        n = m;
    }
    v
}
