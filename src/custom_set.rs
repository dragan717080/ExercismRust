fn main() {
    let mut set = CustomSet::<i32>::new(&[1, 2, 4]);
    set.add(3);
    let expected = CustomSet::<i32>::new(&[1, 2, 3, 4]);
    assert_eq!(set, expected);
}

#[derive(Debug)]
pub struct CustomSet<T> {
    elements: Vec<T>
}

impl<T> PartialEq for CustomSet<T>
where T: Clone + Copy + Ord {
    fn eq(&self, other: &Self) -> bool {
        self.elements.iter().all(|d| other.contains(d)) &&
            other.elements.iter().all(|d| self.contains(d))
    }
}

impl<T: Clone + Copy + Ord> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        Self { elements: _input.to_vec() }
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.elements.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        self.elements.push(_element);
        self.elements.sort();
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.elements.iter().all(|d| _other.contains(d))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.len() == 0
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        !self.elements.iter().any(|item| _other.contains(item))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let elements = self.elements.iter().filter(|item| _other.contains(item)).map(|item| *item).collect();

        Self { elements }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let elements = self.elements.iter().filter(|item| !_other.contains(item)).map(|item| *item).collect();

        Self { elements }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let elements = self.elements.iter().chain(_other.elements.iter()).map(|item| *item).collect();

        Self { elements }
    }
}
