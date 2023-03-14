use std::ptr;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

pub struct Node<T> {
    data: T,
    next: *mut Node<T>,
    prev: *mut Node<T>
}

pub struct LinkedList<T> {
    front: *mut Node<T>,
    back: *mut Node<T>,
    len: usize
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    current: *mut Node<T>,
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl <T> Node<T> {
    pub unsafe fn alloc(data: T) -> *mut Self {
        let mut node = Box::new(Self {
            data,
            next: ptr::null_mut(),
            prev: ptr::null_mut()
        });

        Box::into_raw(node)
    }

    pub unsafe fn free(node: *mut Self) -> T {
        let data = ptr::read(&(*node).data);
        drop(Box::from_raw(node));

        data
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: ptr::null_mut(),
            back: ptr::null_mut(),
            len: 0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.front,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.back,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        Iter(unsafe { self.front.as_ref() })
    }

    pub fn extend<I: Iterator<Item=T>>(&mut self, iter: I) {
        for element in iter {
            self.cursor_back().insert_after(element);
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.front;

        unsafe {
            while !current.is_null() {
                let next = (*current).next;
                drop( Box::from_raw(current));
                current = next;
            }
        }

    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            None
        } else {
            Some(unsafe { &mut (*self.current).data })
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() || (*self.current).next.is_null() {
                None
            } else {
                self.current = (*self.current).next;
                Some(&mut (*self.current).data)
            }
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() || (*self.current).prev.is_null() {
                None
            } else {
                self.current = (*self.current).prev;
                Some(&mut (*self.current).data)
            }
        }
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        let a = 1;

        unsafe {
            match true {
                _ if self.current.is_null() => None,
                _ => {
                    let mut current = self.current;

                    self.current = match true {
                        _ if (*self.current).next.is_null() && (*self.current).prev.is_null() => {
                            let null = ptr::null_mut();
                            self.list.back = null;
                            self.list.front = null;
                            null
                        },
                        _ if (*self.current).next.is_null() => {
                            self.list.back = (*self.current).prev;
                            (*self.list.back).next = ptr::null_mut();
                            self.list.back
                        },
                        _ if (*self.current).prev.is_null() => {
                            self.list.front = (*self.current).next;
                            (*self.list.front).prev = ptr::null_mut();
                            self.list.front
                        },
                        _ => (*self.current).next
                    };

                    self.list.len -= 1;
                    Some(Node::free(current))
                }
            }
        }
    }

    pub fn insert_after(&mut self, _element: T) {
        unsafe {
            let mut node = Node::alloc(_element);

            match true {
                _ if self.current.is_null() => {
                    self.list.front = node;
                    self.list.back = node;
                },
                _ if (*self.current).next.is_null() => {
                    (*self.current).next = node;
                    (*node).prev = self.current;
                    self.list.back = node;
                },
                _ => {
                    (*(*self.current).next).prev = node;
                    (*node).next = (*self.current).next;
                    (*node).prev = self.current;
                    (*self.current).next = node;
                }
            };

            self.list.len += 1;
        }
    }

    pub fn insert_before(&mut self, _element: T) {
        unsafe {
            let mut node = Node::alloc(_element);

            match true {
                _ if self.current.is_null() => {
                    self.list.front = node;
                    self.list.back = node;
                },
                _ if (*self.current).prev.is_null() => {
                    (*self.current).prev = node;
                    (*node).next = self.current;
                    self.list.front = node;
                },
                _ => {
                    (*(*self.current).prev).next = node;
                    (*node).prev = (*self.current).prev;
                    (*node).next = self.current;
                    (*self.current).prev = node;
                }
            };

            self.list.len += 1;
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.0.map(|node| unsafe {
            self.0 = node.next.as_ref();
            &node.data
        })
    }
}