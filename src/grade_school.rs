use std::collections::HashMap;

fn main() {

}

#[allow(clippy::new_without_default)]
pub struct School<'a> {
    all_grades: HashMap<u32, Vec<&'a str>>,
}

impl<'a> School<'a> {
    pub fn new() -> Self {
        Self { all_grades: HashMap::new() }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.all_grades.entry(grade).or_insert(Vec::new()).push(&student);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.all_grades.keys().map(|d| *d).collect::<Vec<u32>>();

        grades.sort();

        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self.all_grades.get(&grade).unwrap_or(&Vec::new()).iter().map(|s: &&str| s.to_string()).collect();

        students.sort();

        students
    }
}
