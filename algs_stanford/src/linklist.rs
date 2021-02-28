/// Unsafe but maybe okay stack queue
///
/// Ref: https://rust-unofficial.github.io/too-many-lists/fourth-final.html
use std::ptr;

pub struct Node {
    s: String,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(s: String) -> Node {
        Node { s, next: None }
    }

    fn into_element(self: Box<Self>) -> String {
        self.s
    }

    pub fn get_value_ref(&self) -> &String {
        &self.s
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
    tail: *mut Node,
    len: usize,
}

impl LinkedList {
    pub fn new() -> LinkedList {
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
    pub fn push(&mut self, s: String) -> &mut LinkedList {
        let mut new_head = Box::new(Node::new(s));
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
    pub fn pop(&mut self) -> Option<String> {
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
    pub fn enqueue(&mut self, s: String) -> &mut LinkedList {
        let mut new_tail = Box::new(Node::new(s));
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
    pub fn dequeue(&mut self) -> Option<String> {
        self.pop()
    }

    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a String;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|n| {
            self.next = n.next.as_deref();
            n.get_value_ref()
        })
    }
}
