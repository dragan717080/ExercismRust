use std::collections::HashMap;

fn main() {
    let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV";
    let student = "Larry";
    println!("{:?}", plants(diagram, student));
}

pub fn plants<'a>(diagram: &'a str, student: &str) -> Vec<&'a str> {
    let map = HashMap::from([
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]);

    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];

    let arr: Vec<&'a str> = diagram.split("\n").collect::<Vec<&str>>();

    let [group1, group2]: [&str; 2] = arr.as_slice().try_into().unwrap();

    let mut result = Vec::new();

    let index = students.iter().position(|name| *name == student).expect("No such student");

    let start_index = index * 2;
    let end_index = start_index + 2;

    for c in group1[start_index..end_index].chars() {
        result.push(*map.get(&c).unwrap());
    }

    for c in group2[start_index..end_index].chars() {
        result.push(*map.get(&c).unwrap());
    }

    result
}
