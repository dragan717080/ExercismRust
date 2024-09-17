fn main() {
    let robot_start = Robot::new(0, 0, Direction::North);
    let robot_end = robot_start.instructions("LAAARALA");
    assert_eq!(robot_end.position(), (-4, 1));
    assert_eq!(robot_end.direction(), &Direction::West);
}


#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let mut robot = self;

        let d = match robot.d {
            Direction::East => Direction::South,
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };

        robot.d = d;

        robot
    }

    pub fn turn_left(self) -> Self {
        let mut robot = self;

        let d = match robot.d {
            Direction::East => Direction::North,
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South
        };

        robot.d = d;

        robot
    }

    pub fn advance(self) -> Self {
        let mut robot = self;

        match robot.d {
            Direction::East => robot.x += 1,
            Direction::North => robot.y += 1,
            Direction::South => robot.y -= 1,
            Direction::West => robot.x -= 1
        }

        robot
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;

        let instructions_chars = instructions.chars().collect::<Vec<char>>();

        for c in instructions_chars {
            match c {
                'A' => robot = robot.advance(),
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                _ => robot = robot.advance(),
            };
        }

        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
