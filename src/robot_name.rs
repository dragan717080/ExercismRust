use std::collections::HashSet;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::sync::Mutex;

lazy_static! {
    static ref ALL_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn main() {
    let vec: Vec<_> = (0..3529).map(|_| Robot::new()).collect();
    let set: HashSet<_> = vec.iter().map(|robot| robot.name()).collect();
    let number_of_collisions = vec.len() - set.len();
    assert_eq!(number_of_collisions, 0);
}

pub struct Robot {
    name: String
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: Robot::generate_random_name()
        }
    }

    fn generate_random_name() -> String {
        let mut name = (0..2).map(|_| char::from_u32(thread_rng().gen_range(65..90)).unwrap()).collect::<String>();
        name.extend((0..3).map(|_| char::from_u32(thread_rng().gen_range(48..57)).unwrap()));

        if ALL_NAMES.lock().unwrap().insert(name.clone()) {
            name
        } else {
            Robot::generate_random_name()
        }
    }

    pub fn name(&self) -> &str {
        &self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_random_name()
    }
}
