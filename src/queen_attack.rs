use std::collections::HashSet;

fn main() {
    let white_queen = Queen::new(ChessPosition::new(3, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(6, 5).unwrap());
    assert!(white_queen.can_attack(&black_queen));

    let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
    let black_queen = Queen::new(ChessPosition::new(1, 1).unwrap());
    assert!(white_queen.can_attack(&black_queen));
}

#[derive(Debug)]
pub struct ChessPosition {
    row: u32,
    col: u32,
}

#[derive(Debug)]
pub struct Queen {
    row: u32,
    col: u32,
    attack_fields: HashSet<[u8; 2]>,
}

impl ChessPosition {
    pub fn new(row: i32, col: i32) -> Option<Self> {
        if row < 0 || row > 7 || col < 0 || col > 7 {
            return None;
        }

        Some(Self {
            row: row as u32,
            col: col as u32,
        })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        let mut queen = Self {
            row: position.row,
            col: position.col,
            attack_fields: HashSet::new(),
        };

        queen.get_attack_fields();

        queen
    }

    fn get_attack_fields(&mut self) {
        let row_fields = (0..8)
            .into_iter()
            .map(|d| [self.row as u8, d as u8])
            .collect::<Vec<[u8; 2]>>();

        self.attack_fields.extend(row_fields);

        let col_fields = (0..8)
            .into_iter()
            .map(|d| [d as u8, self.col as u8])
            .collect::<Vec<[u8; 2]>>();

        self.attack_fields.extend(col_fields);


        let top_left_coordinates: [u8; 2] = [self.row.checked_sub(self.col).unwrap_or(0) as u8, self.col.checked_sub(self.row).unwrap_or(0) as u8];

        let mut bottom_right_coordinates = top_left_coordinates;
        while bottom_right_coordinates[0] < 7 && bottom_right_coordinates[1] < 7 {
            bottom_right_coordinates[0] += 1;
            bottom_right_coordinates[1] += 1;
        }

        let mut top_right_coordinates = [self.row as u8, self.col as u8];

        while top_right_coordinates[0] < 7 && top_right_coordinates[1] > 0 {
            top_right_coordinates[0] += 1;
            top_right_coordinates[1] -= 1;
        }

        let mut j = top_left_coordinates[1];

        for i in top_left_coordinates[0]..=bottom_right_coordinates[0] {
            self.attack_fields.insert([i, j]);
            j += 1;
        }

        let mut bottom_left_coordinates = top_right_coordinates;

        while bottom_left_coordinates[0] > 0 && bottom_left_coordinates[1] < 7 {
            bottom_left_coordinates[0] -= 1;
            bottom_left_coordinates[1] += 1;
        }

        let mut j = bottom_left_coordinates[1];

        for i in bottom_left_coordinates[0]..=top_right_coordinates[0] {
            self.attack_fields.insert([i, j]);
            j = j.checked_sub(1).unwrap_or(0);
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.attack_fields.contains(&[other.row as u8, other.col as u8])
    }
}
