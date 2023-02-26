use std::iter::FromIterator;

pub struct Node<T>(T, Option<Box<Node<T>>>);

impl <T: Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self(data, None)
    }

    pub fn push(&mut self, data: T) {
        match &mut self.1 {
            Some(node) => node.push(data),
            None => self.1 = Some(Box::new(Node::new(data)))
        }
    }

    pub fn pop(&mut self) -> (T, bool) {
        match &mut self.1 {
            None => (self.0.clone(), true),
            Some(node) => {
                let mut returned = node.pop();

                if returned.1 {
                    self.1 = None;
                    returned.1 = false;
                }

                returned
            },
        }
    }

    pub fn peek(&self) -> &T {
        match &self.1 {
            None => &self.0,
            Some(node) => node.peek()
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut list = match self.1 {
            Some(node)  => node.rev(),
            None => SimpleLinkedList::new()
        };

        list.push(self.0);
        list
    }
}

pub struct SimpleLinkedList<T: Clone> {
    head: Option<Node<T>>,
    len: usize
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            len: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, data: T) {
        match &mut self.head {
            Some(node) => node.push(data),
            None => self.head = Some(Node::new(data))
        }

        self.len += 1;
    }

    pub fn chain(mut self, data: T) -> Self {
        self.push(data);
        self
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
        }

        match &mut self.head {
            Some(node) => match node.pop() {
                (data, false) => Some(data),
                (data, true) => {
                    self.head = None;
                    Some(data)
                }
            },
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(node.peek()),
            None => None
        }
    }

    #[must_use]
    pub fn rev(self) -> Self {
        match self.head {
            Some(node) => node.rev(),
            None => Self::new()
        }
    }
}

impl<T: Clone> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> SimpleLinkedList<T> {
        iter
            .into_iter()
            .fold(SimpleLinkedList::new(), SimpleLinkedList::chain)
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Clone> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = Vec::new();
        let mut node = list.head;

        while let Some(current) = node {
            v.push(current.0);

            node = match current.1 {
                Some(n) => Some(*n),
                None => None
            };
        }

        v
    }
}
