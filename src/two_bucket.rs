use std::collections::{HashSet, VecDeque};

fn main() {
    let output = solve(3, 5, 1, &Bucket::One);
    let expected = Some(BucketStats {
        moves: 4,
        goal_bucket: Bucket::One,
        other_bucket: 5,
    });
    assert_eq!(output, expected);
}

#[derive(Debug, Eq, PartialEq)]
pub enum Bucket {
    One,
    Two,
}

#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    pub moves: u8,
    pub goal_bucket: Bucket,
    pub other_bucket: u8,
}

pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let state = match *start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };

    let mut next_search = VecDeque::new();
    let mut visited = HashSet::new();
    let mut moves = 1;

    next_search.push_front(state);

    visited.insert((capacity_1, 0));
    visited.insert((0, capacity_2));

    while !next_search.is_empty() {
        let mut current_search = next_search;
        next_search = VecDeque::new();

        for state in current_search.drain(0..) {
            let (bucket_1, bucket_2) = state;

            if bucket_1 == goal {
                return Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::One,
                    other_bucket: bucket_2,
                });
            } else if bucket_2 == goal {
                return Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::Two,
                    other_bucket: bucket_1,
                });
            }

            let empty_1 = (0, bucket_2);
            if !visited.contains(&empty_1) {
                next_search.push_front(empty_1);
                visited.insert(empty_1);
            }

            let empty_2 = (bucket_1, 0);
            if !visited.contains(&empty_2) {
                next_search.push_front(empty_2);
                visited.insert(empty_2);
            }

            let fill_1 = (capacity_1, bucket_2);
            if !visited.contains(&fill_1) {
                next_search.push_front(fill_1);
                visited.insert(fill_1);
            }

            let fill_2 = (bucket_1, capacity_2);
            if !visited.contains(&fill_2) {
                next_search.push_front(fill_2);
                visited.insert(fill_2);
            }

            let pour_1_into_2 = if bucket_1 + bucket_2 <= capacity_1 {
                (bucket_1 + bucket_2, 0)
            } else {
                (capacity_1, bucket_1 + bucket_2 - capacity_1)
            };
            if !visited.contains(&pour_1_into_2) {
                next_search.push_front(pour_1_into_2);
                visited.insert(pour_1_into_2);
            }

            let pour_2_into_1 = if bucket_1 + bucket_2 <= capacity_2 {
                (0, bucket_1 + bucket_2)
            } else {
                (bucket_1 + bucket_2 - capacity_2, capacity_2)
            };
            if !visited.contains(&pour_2_into_1) {
                next_search.push_front(pour_2_into_1);
                visited.insert(pour_2_into_1);
            }
        }

        moves += 1;
    }

    None
}
