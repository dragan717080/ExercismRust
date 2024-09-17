use std::collections::{HashMap, HashSet};

mod utils;

use utils::{find_hashmap_by_value, get_unique_combinations};

fn main() {
    let input = &["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"];
    let output = winning_hands(input).into_iter().collect::<HashSet<_>>();
    let expected = ["5H 5S 5D 9S 9D"].into_iter().collect::<HashSet<_>>();
    assert_eq!(output, expected);
}

#[derive(Eq, Debug, Hash, PartialEq)]
enum StrongHand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    Straight,
    Flush,
    FullHouse,
    FourOfKind,
    StraightFlush,
}

#[derive(Debug)]
struct PokerHand<'a> {
    hand: &'a str,
    all_card_strengths: HashMap<StrongHand, u32>,
    numbers: Vec<u32>,
}

impl<'a> PokerHand<'a> {
    fn new(hand: &'a str) -> Self {
        Self {
            hand,
            all_card_strengths: Self::all_card_strengths(),
            numbers: Vec::new(),
        }
    }

    fn get_strength(&mut self) -> u32 {
        let cards = self.hand.split_whitespace().collect::<Vec<&str>>();

        // Split at last char to get number and sign (e.g. 'K' and 'H')
        let numbers_strings = cards.iter().map(|s| &s[0..s.len() - 1]).collect::<Vec<&str>>();

        let signs = cards.iter().map(|s| s.chars().last().unwrap()).collect::<HashSet<char>>();
        let is_flush = signs.len() == 1;

        for number_str in &numbers_strings {
            let d;

            if *number_str == "A" {
                d = 11;
            } else if *number_str == "J" {
                d = 12;
            } else if *number_str == "Q" {
                d = 13;
            } else if *number_str == "K" {
                d = 14;
            } else {
                d = number_str.parse().unwrap();
            }

            self.numbers.push(d);
        }

        let different_elements_count: HashSet<u32> = HashSet::from_iter(self.numbers.clone());

        let mut is_straight = false;

        // If all cards have different numbers, check for straight
        if different_elements_count.len() == numbers_strings.len() {
            if self.is_straight(self.numbers.clone()) {
                is_straight = true;
                if is_flush {
                    return self.all_card_strengths[&StrongHand::StraightFlush];
                }
            }
        }

        let number_frequencies = frequency_counts(self.numbers.clone());

        let largest_sequence = *number_frequencies.iter().max().unwrap();

        if largest_sequence == 4 {
            return self.all_card_strengths[&StrongHand::FourOfKind];
        }

        let mut is_three_of_kind = false;

        if largest_sequence == 3 {
            is_three_of_kind = true;

            if number_frequencies.len() == 2 {
                return self.all_card_strengths[&StrongHand::FullHouse];
            }
        }

        if is_flush {
            return self.all_card_strengths[&StrongHand::Flush];
        }

        if is_straight {
            return self.all_card_strengths[&StrongHand::Straight];
        }

        if is_three_of_kind {
            return self.all_card_strengths[&StrongHand::ThreeOfKind];
        }

        let is_two_pair = largest_sequence == 2 && number_frequencies.len() == 3;

        if is_two_pair {
            return self.all_card_strengths[&StrongHand::TwoPair];
        }

        let is_one_pair = largest_sequence == 2 && number_frequencies.len() == 4;

        if is_one_pair {
            return self.all_card_strengths[&StrongHand::OnePair];
        }

        return self.all_card_strengths[&StrongHand::HighCard];
    }

    fn is_straight(&self, numbers: Vec<u32>) -> bool {
        let mut sorted_numbers = numbers;
        sorted_numbers.sort();

        // Ace can start a straight of 1-5
        if sorted_numbers[3] == 5 && sorted_numbers[4] == 11 {
            return true;
        }

        sorted_numbers[4] - sorted_numbers[0] == 4
    }

    fn all_card_strengths() -> HashMap<StrongHand, u32> {
        HashMap::from([
            (StrongHand::HighCard, 1),
            (StrongHand::OnePair, 2),
            (StrongHand::TwoPair, 3),
            (StrongHand::ThreeOfKind, 4),
            (StrongHand::Straight, 5),
            (StrongHand::Flush, 6),
            (StrongHand::FullHouse, 7),
            (StrongHand::FourOfKind, 8),
            (StrongHand::StraightFlush, 9),
        ])
    }
}

/**
 * Frequency counts but with cards as keys and frequency as values,
 e.g. card k has appeared v times.
 */
fn frequency_with_cards(arr: Vec<u32>) -> HashMap<u32, u32> {
    let mut counts: HashMap<u32, u32> = HashMap::new();

    for item in arr {
        *counts.entry(item).or_insert(0) += 1;
    }

    counts
}

fn frequency_counts(arr: Vec<u32>) -> Vec<u32> {
    let counts = frequency_with_cards(arr);

    let mut counts = counts.values().into_iter().map(|d| *d).collect::<Vec<u32>>();

    counts.sort();

    counts
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return [hands[0]].to_vec();
    }

    let indexes = (0..hands.len()).collect::<Vec<usize>>();
    let unique_combinations = get_unique_combinations(&indexes, 2);

    let mut strongest_hands: Vec<&'a str> = vec![];

    for unique_combination in unique_combinations {
        let [hand1_index, hand2_index] = [unique_combination[0], unique_combination[1]];

        let [hand1, hand2] = [hands.get(hand1_index).unwrap(), hands.get(hand2_index).unwrap()];

        #[allow(suspicious_double_ref_op)]
        let stronger_hand = compare_hands(hand1, hand2);

        // If current stronger hand is strongest so far, delete all from array of strongest hands and push to it
        if let Some(strongest_hand_so_far) = strongest_hands.get(0) {
            let compared_hands = compare_hands(stronger_hand[0], strongest_hand_so_far);
            if compared_hands == stronger_hand {
                strongest_hands = stronger_hand;
            } else {
                strongest_hands = stronger_hand;
            }
        } else {
            // Strongest hands was empty before
            strongest_hands = stronger_hand;
        }
    }

    strongest_hands
}

fn handle_tie<'a> (
    original_hand1: &'a str,
    original_hand2: &'a str,
    hand1: &mut PokerHand,
    hand2: &mut PokerHand,
    hand_strength: u32,
    ) -> Vec<&'a str> {
        // Highest card wins
        hand1.numbers = hand1.numbers.iter().rev().map(|d| *d).collect::<Vec<u32>>();
        hand2.numbers = hand2.numbers.iter().rev().map(|d| *d).collect::<Vec<u32>>();

        if [1, 6].contains(&hand_strength) {
            return compare_cards_in_hand(original_hand1, original_hand2, hand1, hand2);
        }

        let hand1_frequencies_with_cards = frequency_with_cards(hand1.numbers.clone());
        let hand2_frequencies_with_cards = frequency_with_cards(hand2.numbers.clone());

        let hand1_card_with_freq = hand1_frequencies_with_cards.iter().map(|(k, v)| (*k, *v)).filter(|(_, v)| *v > 1).collect::<HashMap<u32, u32>>();
        let hand2_card_with_freq = hand2_frequencies_with_cards.iter().map(|(k, v)| (*k, *v)).filter(|(_, v)| *v > 1).collect::<HashMap<u32, u32>>();

        // Highest pair wins
        if hand_strength == 2 {
            let hand1_highest_pair = hand1_card_with_freq.keys().map(|d| d.to_owned()).collect::<Vec<u32>>()[0];
            let hand2_highest_pair = hand2_card_with_freq.keys().map(|d| d.to_owned()).collect::<Vec<u32>>()[0];

            if hand1_highest_pair > hand2_highest_pair {
                return [original_hand1].to_vec();
            } else if hand2_highest_pair > hand1_highest_pair {
                return [original_hand2].to_vec();
            } else {
                // If both cards have the same pair, kicker wins
                return compare_cards_in_hand(original_hand1, original_hand2, hand1, hand2);
            }
        } else if hand_strength == 3 {
            let mut hand1_pairs = hand1_card_with_freq.keys().map(|d| d.to_owned()).collect::<Vec<u32>>();
            hand1_pairs.sort();
            hand1_pairs.reverse();

            let mut hand2_pairs = hand2_card_with_freq.keys().map(|d| d.to_owned()).collect::<Vec<u32>>();
            hand2_pairs.sort();
            hand2_pairs.reverse();

            // Can't be simplified
            if hand1_pairs[0] > hand2_pairs[0] {
                return [original_hand1].to_vec();
            } else if hand2_pairs[0] > hand1_pairs[0] {
                return [original_hand2].to_vec();
            } else if hand1_pairs[1] > hand2_pairs[1] {
                return [original_hand1].to_vec();
            } else if hand1_pairs[1] > hand2_pairs[1] {
                return [original_hand2].to_vec();
            }

            // If two pairs are the same, decide with kicker
            return compare_cards_in_hand(original_hand1, original_hand2, hand1, hand2);
        } else if hand_strength == 4 {
            let hand1_triplet = find_hashmap_by_value(&hand1_card_with_freq, 3);
            let hand2_triplet = find_hashmap_by_value(&hand2_card_with_freq, 3);

            if hand1_triplet > hand2_triplet {
                return [original_hand1].to_vec();
            } else {
                return [original_hand2].to_vec();
            }
        } if [5, 9].contains(&hand_strength) {
            // If both hands are straights, rank by highest non ace card
            let hand1_ace_index = hand1.numbers.iter().position(|d| *d == 11);
            if hand1_ace_index.is_some() {
                hand1.numbers.remove(hand1_ace_index.unwrap());
            }

            let hand2_ace_index = hand2.numbers.iter().position(|d| *d == 11);
            if hand2_ace_index.is_some() {
                hand2.numbers.remove(hand2_ace_index.unwrap());
            }

            let hand1_max = hand1.numbers.iter().max();
            let hand2_max = hand2.numbers.iter().max();

            if hand1_max > hand2_max {
                return [original_hand1].to_vec();
            } else {
                return [original_hand2].to_vec();
            }
        } else if hand_strength == 7 {
            // Card with triplet in hand
            let hand1_triplet = find_hashmap_by_value(&hand1_card_with_freq, 3);
            let hand2_triplet = find_hashmap_by_value(&hand2_card_with_freq, 3);

            if hand1_triplet > hand2_triplet {
                return [original_hand1].to_vec();
            } else if hand2_triplet > hand1_triplet {
                return [original_hand2].to_vec();
            }

            // Same triplets, order by pair
            let hand1_pair = find_hashmap_by_value(&hand1_card_with_freq, 2);
            let hand2_pair = find_hashmap_by_value(&hand2_card_with_freq, 2);

            if hand1_pair > hand2_pair {
                return [original_hand1].to_vec();
            } else {
                return [original_hand2].to_vec();
            }
        } else if hand_strength == 8 {
            // Card with quad in hand
            let hand1_quad = find_hashmap_by_value(&hand1_card_with_freq, 4);
            let hand2_quad = find_hashmap_by_value(&hand2_card_with_freq, 4);

            if hand1_quad > hand2_quad {
                return [original_hand1].to_vec();
            } else {
                return [original_hand2].to_vec();
            }
        }

        [original_hand1, original_hand2].to_vec()
    }

/**
 * When two hands are tied, compare hands by rank.
 */
fn compare_cards_in_hand<'a>(
    original_hand1: &'a str,
    original_hand2: &'a str,
    hand1: &mut PokerHand,
    hand2: &mut PokerHand
) -> Vec<&'a str> {
    hand1.numbers.sort();
    hand1.numbers.reverse();

    hand2.numbers.sort();
    hand2.numbers.reverse();

    for i in 0..5 {
        let card1 = hand1.numbers[i];
        let card2 = hand2.numbers[i];

        if card1 > card2 {
            return [original_hand1].to_vec();
        } else if card2 > card1 {
            return [original_hand2].to_vec();
        }
    }

    return [original_hand1, original_hand2].to_vec();
}

// If there is a tie, return both
fn compare_hands<'a>(original_hand1: &'a str, original_hand2: &'a str) -> Vec<&'a str> {
    if original_hand1 == original_hand2 {
        return [original_hand1].to_vec();
    }

    let mut hand1 = PokerHand::new(original_hand1);
    let mut hand2 = PokerHand::new(original_hand2);

    let hand1_strength = hand1.get_strength();
    let hand2_strength = hand2.get_strength();

    if hand1_strength > hand2_strength {
        [original_hand1].to_vec()
    } else if hand2_strength > hand1_strength {
        [original_hand2].to_vec()
    } else {
        handle_tie(original_hand1, original_hand2, &mut hand1, &mut hand2, hand1_strength)
    }
}
