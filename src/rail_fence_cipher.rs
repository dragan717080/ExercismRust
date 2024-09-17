fn main() {
    let input = "古池蛙飛び込む水の音";
    let rails = 3;
    let rail_fence = RailFence::new(rails);
    let output = rail_fence.encode(input);
    let expected = "古びの池飛込水音蛙む";
    assert_eq!(output, expected);
}

pub struct RailFence {
    rail_count: usize
}

impl RailFence {
    pub fn new(rail_count: u32) -> RailFence {
        Self { rail_count: rail_count as usize }
    }

    pub fn encode(&self, input: &str) -> String {
        let s_chars: Vec<char> = input.chars().collect();

        let mut rails: Vec<Vec<char>> = Vec::with_capacity(self.rail_count);

        for _ in 0..self.rail_count {
            rails.push(Vec::new());
        }

        let mut current_rail = 0;
        let mut direction = 1;
        for &c in &s_chars {
            rails[current_rail].push(c);
            if current_rail == 0 {
                direction = 1;
            } else if current_rail == self.rail_count - 1 {
                direction = -1;
            }
            current_rail = (current_rail as isize + direction) as usize;
        }

        let encoded: String = rails.iter().flatten().collect();
        encoded
    }

    pub fn decode(&self, cipher: &str) -> String {
        // Collect characters into a vector
        let c_chars: Vec<char> = cipher.chars().collect();

        // Create a 2D vector for rails
        let mut rails: Vec<Vec<char>> = vec![Vec::new(); self.rail_count];

        // Determine the pattern length
        let pattern_length = 2 * (self.rail_count - 1);

        // Initialize counters for each rail
        let mut counters = vec![0; self.rail_count];

        // Determine the number of characters in each rail
        for i in 0..c_chars.len() {
            let rail = if i % pattern_length < self.rail_count {
                i % pattern_length
            } else {
                pattern_length - i % pattern_length
            };
            counters[rail] += 1;
        }

        let mut pos = 0;
        for rail in 0..self.rail_count {
            for _ in 0..counters[rail] {
                rails[rail].push(c_chars[pos]);
                pos += 1;
            }
        }

        let mut decoded = String::with_capacity(cipher.len());
        let mut current_rail = 0;
        let mut direction = 1;
        let mut rail_positions = vec![0; self.rail_count];

        for _ in 0..cipher.len() {
            decoded.push(rails[current_rail][rail_positions[current_rail]]);
            rail_positions[current_rail] += 1;

            if current_rail == 0 {
                direction = 1;
            } else if current_rail == self.rail_count - 1 {
                direction = -1;
            }

            current_rail = (current_rail as isize + direction) as usize;
        }

        decoded
    }
}
