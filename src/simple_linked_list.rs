fn main() {
    let mut list: SimpleLinkedList<u32> = SimpleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut rev_list = list.rev();
    assert_eq!(rev_list.pop(), Some(1));
    assert_eq!(rev_list.pop(), Some(2));
    assert_eq!(rev_list.pop(), Some(3));
    assert_eq!(rev_list.pop(), None);
}

struct ListElement<T> {
    element: T,
    next: Option<Box<ListElement<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<ListElement<T>>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, length: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        let new_element = ListElement {
            element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_element));
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.length -= 1;
            node.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.element)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed_list = SimpleLinkedList::new();
        let mut current = self.head;

        while let Some(node) = current {
            reversed_list.push(node.element);
            current = node.next;
        }

        reversed_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for element in _iter {
            list.push(element);
        }

        list
    }
}

impl<T: std::fmt::Display,> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut result = Vec::new();

        let _linked_list = _linked_list.rev();

        let mut current = _linked_list.head;

        while let Some(node) = current {
            result.push(node.element);
            current = node.next;
        }

        result
    }
}
