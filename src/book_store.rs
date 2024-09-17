use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BTreeSet, HashSet};
use std::hash::{Hash, Hasher};

fn main() {
    let input = &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5];
    let output = lowest_price(input);
    let expected = 10000;
    assert_eq!(output, expected);
}

type Book = u32;
type Price = u32;
const BOOK_PRICE: Price = 800;

#[derive(Debug, Clone)]
struct Group(RefCell<BTreeSet<Book>>);

impl Group {
    fn new() -> Group {
        Group(RefCell::new(BTreeSet::new()))
    }

    fn new_containing(book: Book) -> Group {
        let g = Group::new();
        g.0.borrow_mut().insert(book);
        g
    }

    fn price(&self) -> Price {
        (self.0.borrow().len() as Price)
            * BOOK_PRICE
            * match self.0.borrow().len() {
                2 => 95,
                3 => 90,
                4 => 80,
                5 => 75,
                _ => 100,
            }
            / 100
    }
}

impl Ord for Group {
    // we want to order groups first by qty contained DESC, then by lowest value ASC
    fn cmp(&self, other: &Group) -> Ordering {
        match other.0.borrow().len().cmp(&self.0.borrow().len()) {
            Ordering::Equal => {
                if self.0.borrow().is_empty() {
                    Ordering::Equal
                } else {
                    self.0
                        .borrow()
                        .iter()
                        .next()
                        .unwrap()
                        .cmp(other.0.borrow().iter().next().unwrap())
                }
            }
            otherwise => otherwise,
        }
    }
}

impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Group) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.0.borrow().eq(&other.0.borrow())
    }
}

impl Eq for Group {}

impl Hash for Group {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.borrow().hash(hasher);
    }
}

fn basket_price(basket: &[Group]) -> Price {
    basket.iter().map(|g| g.price()).sum()
}

// Get the hash of a Vec<Group>
fn hash_of(basket: &[Group]) -> u64 {
    let lengths = basket
        .iter()
        .map(|g| g.0.borrow().len())
        .collect::<Vec<_>>();
    let mut hasher = DefaultHasher::new();
    lengths.hash(&mut hasher);
    hasher.finish()
}

pub fn lowest_price(books: &[Book]) -> Price {
    DecomposeGroups::new(books)
        .map(|gb| basket_price(&gb))
        .min()
        .unwrap_or(0)
}

struct DecomposeGroups {
    prev_states: HashSet<u64>,
    next: Option<Vec<Group>>,
}

impl Iterator for DecomposeGroups {
    type Item = Vec<Group>;
    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.next.clone();
        if let Some(groups) = self.next.take() {
            if !(groups.is_empty() || groups.iter().all(|g| g.0.borrow().len() == 1)) {
                let mut hypothetical;
                for mpg_book in groups[0].0.borrow().iter() {
                    for (idx, other_group) in groups[1..].iter().enumerate() {
                        if !other_group.0.borrow().contains(mpg_book) {
                            hypothetical = groups.clone();
                            hypothetical[0].0.borrow_mut().remove(mpg_book);
                            hypothetical[1 + idx].0.borrow_mut().insert(*mpg_book);
                            hypothetical.sort();
                            let hypothetical_hash = hash_of(&hypothetical);
                            if !self.prev_states.contains(&hypothetical_hash) {
                                self.prev_states.insert(hypothetical_hash);
                                self.next = Some(hypothetical);
                                return return_value;
                            }
                        }
                    }
                }

                let book = {
                    let backing_bt = groups[0].0.borrow();
                    let mut book_iter = backing_bt.iter();
                    *book_iter.next().unwrap()
                };
                hypothetical = groups;
                hypothetical[0].0.borrow_mut().remove(&book);
                hypothetical.push(Group::new_containing(book));
                hypothetical.sort();
                self.prev_states.insert(hash_of(&hypothetical));
                self.next = Some(hypothetical);
            }
        }
        return_value
    }
}

impl DecomposeGroups {
    fn new(books: &[Book]) -> DecomposeGroups {
        let mut book_groups = Vec::new();
        'nextbook: for book in books {
            for Group(book_group) in book_groups.iter() {
                if !book_group.borrow().contains(book) {
                    book_group.borrow_mut().insert(*book);
                    continue 'nextbook;
                }
            }

            book_groups.push(Group::new_containing(*book));
        }
        book_groups.sort();

        DecomposeGroups {
            next: Some(book_groups),
            prev_states: HashSet::new(),
        }
    }
}
