/// Safe but not good stack queue
///
/// Ref: https://rust-unofficial.github.io/too-many-lists/fourth-final.html
mod link {
    use std::cell::{Ref, RefCell};
    use std::ops::Deref;
    use std::rc::Rc;

    #[derive(Default)]
    pub struct Node {
        s: String,
        prev: Option<Link>,
        next: Option<Link>,
    }

    impl Node {
        pub fn new(s: String) -> Link {
            Link::new(Rc::new(RefCell::new(Node {
                s,
                prev: None,
                next: None,
            })))
        }
    }

    #[derive(Clone)]
    pub struct Link {
        inner: Rc<RefCell<Node>>,
    }

    impl Link {
        pub fn new(inner: Rc<RefCell<Node>>) -> Link {
            Link { inner }
        }

        pub fn deref(&self) -> &RefCell<Node> {
            self.inner.deref()
        }

        pub fn prev(&self) -> Option<Link> {
            self.inner.deref().borrow().prev.clone()
        }

        pub fn next(&self) -> Option<Link> {
            self.inner.deref().borrow().next.clone()
        }

        pub fn link(&self, next: Link) -> Link {
            self.deref().borrow_mut().next = Some(next.clone());
            next.deref().borrow_mut().prev = Some(self.clone());
            next
        }

        pub fn take(&self) -> String {
            self.inner.take().s
        }

        pub fn get_value_ref(&self) -> Ref<String> {
            Ref::map(self.inner.deref().borrow(), |n| &n.s)
        }
    }

    pub struct LinkedList {
        head: Option<Link>,
        tail: Option<Link>,
    }

    impl LinkedList {
        pub fn new() -> LinkedList {
            LinkedList {
                head: None,
                tail: None,
            }
        }

        /// add element to list at the end
        pub fn append(&mut self, s: String) -> &mut LinkedList {
            let tail = &self.tail;
            if let Some(ref last) = tail {
                let new_tail = last.link(Node::new(s));
                self.tail = Some(new_tail);
            } else {
                let new_node = Node::new(s);
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            self
        }

        /// pop element from tail
        pub fn dequeue(&mut self) -> Option<String> {
            if let Some(tail) = self.tail.take() {
                match tail.prev() {
                    Some(prev) => {
                        prev.inner.deref().borrow_mut().next = None;
                        self.tail = Some(prev);
                    }
                    None => self.head = None,
                }
                Some(tail.take())
            } else {
                None
            }
        }

        /// push element from head
        pub fn push(&mut self, s: String) -> &mut LinkedList {
            if let Some(head) = self.head.take() {
                let new_head = Node::new(s);
                new_head.link(head);
                self.head = Some(new_head);
            } else {
                let new_node = Node::new(s);
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            self
        }

        /// enqueue element from head
        pub fn enqueue(&mut self, s: String) -> &mut LinkedList {
            self.push(s)
        }

        /// pop element from head
        pub fn pop(&mut self) -> Option<String> {
            if let Some(head) = self.head.take() {
                match head.next() {
                    Some(next) => {
                        next.inner.deref().borrow_mut().prev = None;
                        self.head = Some(next);
                    }
                    None => self.tail = None,
                }
                Some(head.take())
            } else {
                None
            }
        }

        pub fn iter(&self) -> Iter {
            Iter {
                next: self.head.clone(),
            }
        }
    }

    /// Not working ... can not let Iterator return a reference of inner value
    /// https://rust-unofficial.github.io/too-many-lists/fourth-iteration.html
    // pub struct Iter<'a> {
    //     next: Option<Ref<'a, Node>>,
    // }
    // impl<'a> Iterator for Iter<'a> {
    //     type Item = Ref<'a, String>;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         self.next.map(|link: Rc<Node>| {
    //             self.next = link.next.as_ref().map(|l| l.deref().borrow());
    //             Ref::map(link, |n: &Node| &n.s)
    //         })
    //     }
    // }
    pub struct Iter {
        next: Option<Link>,
    }
    impl Iterator for Iter {
        type Item = Link;
        fn next(&mut self) -> Option<Self::Item> {
            self.next.clone().map(|link| {
                self.next = link.next();
                link
            })
        }
    }
}

fn main() {
    use link::LinkedList;

    let mut list = LinkedList::new();
    // insert from tail
    list.append("1".to_string())
        .append("2".to_string())
        .append("3".to_string());

    // insert from head
    list.push("a".to_string())
        .push("b".to_string())
        .enqueue("c".to_string());

    for s in list.iter() {
        print!("{},", s.get_value_ref())
    }
    println!();

    println!("pop head: {}", list.pop().unwrap_or("?".to_string()));
    println!("pop head: {}", list.pop().unwrap_or("?".to_string()));

    println!("pop tail: {}", list.dequeue().unwrap_or("?".to_string()));
    println!("pop tail: {}", list.dequeue().unwrap_or("?".to_string()));

    for s in list.iter() {
        print!("{},", s.get_value_ref())
    }
    println!();

    println!("pop head: {}", list.pop().unwrap_or("?".to_string()));
    println!("pop tail: {}", list.dequeue().unwrap_or("?".to_string()));
    println!("pop head: {}", list.pop().unwrap_or("?".to_string()));
    println!("pop tail: {}", list.dequeue().unwrap_or("?".to_string()));
}

// If normal borrow checker cannot work, then the borrow checker bug still exist, use
// rustc -Zpolonius main.rs
// See https://github.com/rust-lang/rust/issues/62013
// See https://github.com/rust-lang/rust/issues/67957
