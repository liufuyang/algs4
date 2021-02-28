/// Unsafe but maybe okay stack queue
///
/// Ref: https://rust-unofficial.github.io/too-many-lists/fourth-final.html
use std::ptr;

pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(element: T) -> Node<T> {
        Node {
            element,
            next: None,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }

    pub fn get_value_ref(&self) -> &T {
        &self.element
    }
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.len
    }

    /// push element from head
    pub fn push(&mut self, element: T) -> &mut LinkedList<T> {
        let mut new_head = Box::new(Node::new(element));
        let raw_head: *mut _ = &mut *new_head;

        if !self.tail.is_null() {
            let old_head = self.head.take();
            new_head.next = old_head;
            self.head = Some(new_head);
        } else {
            self.head = Some(new_head);
            self.tail = raw_head;
        }
        self.len += 1;
        self
    }

    /// pop element from head
    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut old_head) = self.head.take() {
            match old_head.next.take() {
                Some(next) => {
                    self.head = Some(next);
                }
                None => self.tail = ptr::null_mut(),
            }
            self.len -= 1;
            Some(old_head.into_element())
        } else {
            None
        }
    }

    /// enqueue element to list at the end
    pub fn enqueue(&mut self, element: T) -> &mut LinkedList<T> {
        let mut new_tail = Box::new(Node::new(element));
        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
        self.len += 1;
        self
    }

    /// dequeue element from head, the same as pop
    pub fn dequeue(&mut self) -> Option<T> {
        self.pop()
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            n.get_value_ref()
        })
    }
}
