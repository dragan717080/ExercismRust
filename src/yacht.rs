use std::collections::HashMap;

fn main() {
    let expected = 30;
    assert_eq!(score([4, 6, 2, 5, 3], Category::BigStraight), expected);

    let expected = 19;
    assert_eq!(score([5, 3, 3, 5, 3], Category::FullHouse), expected);
}

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut frequencies: HashMap<u8, u8> = HashMap::new();

    for num in dice {
        *frequencies.entry(num).or_insert(0) += 1;
    }

    println!("{:?}", frequencies);

    match category {
        Category::Ones => score_single(1, frequencies),
        Category::Twos => score_single(2, frequencies),
        Category::Threes => score_single(3, frequencies),
        Category::Fours => score_single(4, frequencies),
        Category::Fives => score_single(5, frequencies),
        Category::Sixes => score_single(6, frequencies),
        Category::FullHouse => score_full_house(frequencies),
        Category::FourOfAKind => score_four_of_kind(frequencies),
        Category::LittleStraight => score_straight(false, frequencies),
        Category::BigStraight => score_straight(true, frequencies),
        Category::Choice => score_choice(dice),
        Category::Yacht => score_yacht(frequencies)
    }
}

fn score_single(num: u8, frequencies: HashMap<u8, u8>) -> u8 {
    num * *frequencies.get(&num).unwrap_or(&0)
}

fn score_full_house(frequencies: HashMap<u8, u8>) -> u8 {
    let values = frequencies.values().map(|d| *d).collect::<Vec<u8>>();

    if !values.contains(&3) || !values.contains(&2) {
        return 0;
    }

    frequencies.iter().fold(0, |acc, (k, v)| { acc + k*v })
}

fn score_four_of_kind(frequencies: HashMap<u8, u8>) -> u8 {
    let k = frequencies.iter().find(|&(_, v)| *v >= 4);

    match k {
        Some(k) => k.0 * &4,
        None => 0
    }
}

fn score_straight(is_big: bool, frequencies: HashMap<u8, u8>) -> u8 {
    let mut keys = frequencies.keys().map(|d| *d).collect::<Vec<u8>>();

    keys.sort();

    if !(keys.len() == 5) {
        return 0;
    }

    let is_straight = is_big && keys[0] == 2 && keys[4] == 6 || !is_big && keys[0] == 1 && keys[4] == 5;

    if !is_straight { 0 } else { 30 }
}

fn score_choice(dice: Dice) -> u8 {
    dice.iter().sum()
}

fn score_yacht(frequencies: HashMap<u8, u8>) -> u8 {
    if frequencies.keys().len() == 1 { 50 } else { 0 }
}
