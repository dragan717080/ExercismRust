// IMPORTANT: Uncomment next line before posting
// mod pre_implemented;

use std::ptr::NonNull;

fn main() {

}

type NodePtr<T> = NonNull<Node<T>>;
type OptNodePtr<T> = Option<NodePtr<T>>;

pub struct LinkedList<T> {
    back: OptNodePtr<T>,
    front: OptNodePtr<T>,
    len: usize,
    marker: std::marker::PhantomData<Box<T>>,
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    node: OptNodePtr<T>,
}

struct Node<T> {
    element: T,
    next: OptNodePtr<T>,
    prev: OptNodePtr<T>,
}

impl<T> Node<T> {
    fn new_linkless(element: T) -> NodePtr<T> {
        unsafe {
            NonNull::new_unchecked(Box::into_raw(Box::new(Self {
                element,
                prev: None,
                next: None,
            })))
        }
    }

    unsafe fn link(mut left: NodePtr<T>, mut right: NodePtr<T>) {
        left.as_mut().next = Some(right);
        right.as_mut().prev = Some(left);
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            back: None,
            front: None,
            len: 0,
            marker: std::marker::PhantomData,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.front,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            node: self.back,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next_node: self.front,
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut cursor = self.cursor_front();
        while cursor.take().is_some() {}
    }
}

pub struct Iter<'a, T> {
    next_node: OptNodePtr<T>,
    marker: std::marker::PhantomData<&'a LinkedList<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node_ptr = self.next_node?;
        unsafe {
            let current_node = &*node_ptr.as_ptr();
            self.next_node = current_node.next;
            Some(&current_node.element)
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.node.map(|node| &mut (*node.as_ptr()).element)
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe { self._step(|node| node.next) }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe { self._step(|node| node.prev) }
    }

    unsafe fn _step(&mut self, get_next: impl Fn(&Node<T>) -> OptNodePtr<T>) -> Option<&mut T> {
        let new_pos = get_next(self.node?.as_ref())?;
        self.node = Some(new_pos);
        Some(&mut (*new_pos.as_ptr()).element)
    }

    pub fn take(&mut self) -> Option<T> {
        unsafe {
            let mut node = self.node?;
            let &mut Node { prev, next, .. } = node.as_mut();

            self.node = next.or(prev);
            match next {
                Some(mut next) => next.as_mut().prev = prev,
                None => self.list.back = prev,
            };
            match prev {
                Some(mut prev) => prev.as_mut().next = next,
                None => self.list.front = next,
            }

            self.list.len -= 1;
            Some(Box::from_raw(node.as_ptr()).element)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            self._insert(
                element,
                |list| &mut list.back,
                |mut cursor_node, new_node| {
                    if let Some(next) = cursor_node.as_mut().next {
                        Node::link(new_node, next);
                    }
                    Node::link(cursor_node, new_node);
                },
            );
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            self._insert(
                element,
                |list| &mut list.front,
                |mut cursor_node, new_node| {
                    if let Some(prev) = cursor_node.as_mut().prev {
                        Node::link(prev, new_node);
                    }
                    Node::link(new_node, cursor_node);
                },
            );
        }
    }

    unsafe fn _insert(
        &mut self,
        element: T,
        end_node: impl Fn(&mut LinkedList<T>) -> &mut OptNodePtr<T>,
        link_new_node: impl Fn(NodePtr<T>, NodePtr<T>),
    ) {
        let new_node = Node::new_linkless(element);

        let cursor_node = match self.node {
            Some(node) => node,
            None => {
                self.node = Some(new_node);
                self.list.back = self.node;
                self.list.front = self.node;
                self.list.len += 1;
                return;
            }
        };
        link_new_node(cursor_node, new_node);
        let end_node = end_node(self.list);
        if *end_node == Some(cursor_node) {
            *end_node = Some(new_node);
        }
        self.list.len += 1;
    }
}
