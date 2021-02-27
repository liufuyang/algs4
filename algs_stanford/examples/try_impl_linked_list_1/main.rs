#![feature(option_insert)]
mod link {
    pub struct Node {
        s: String,
        next: Option<Box<Node>>,
    }

    impl Node {
        pub fn init() -> Node {
            Node {
                s: "dummy_head".to_string(),
                next: None,
            }
        }

        pub fn new(s: String) -> Node {
            Node { s: s, next: None }
        }

        pub fn get_value_ref(&self) -> &String {
            &self.s
        }

        // pub fn link(&mut self, n: Node) -> &mut Node {
        //     // or use option.insert()
        //     self.next = Some(Box::new(n));
        //     self.next.as_deref_mut().unwrap()
        // }
    }

    pub struct LinkedList {
        head: Node,
    }

    impl LinkedList {
        pub fn new() -> LinkedList {
            let head = Node::init();
            LinkedList { head }
        }

        /// add element to list at the end
        pub fn append(&mut self, s: String) -> &mut LinkedList {
            let mut tail = &mut self.head;
            while let Some(ref mut next) = tail.next {
                tail = next;
            }
            tail.next = Some(Box::new(Node::new(s)));
            self
        }

        // https://stackoverflow.com/questions/66382519/rust-pattern-match-on-a-reference-of-option-one-way-working-but-the-others-way-n
        // pub fn append(&mut self, s: String) -> &mut LinkedList {
        //     let mut tail = &mut self.head;
        //     while let Some(next) = tail.next.as_deref_mut() {
        //         // why this line won't work?  but above solution works?
        //         tail = next;
        //     }
        //     tail.next = Some(Box::new(Node::new(s)));
        //     self
        // }

        /// pop element from tail
        // pub fn pop_tail(&mut self) -> Option<String> {
        //     if self.head.next.is_none() {
        //         return None;
        //     }
        //
        //     let mut one_before_tail = &mut self.head;
        //     while let Some(ref mut next_before) = one_before_tail.next {
        //         if let Some(ref mut next) = next_before.next {
        //             one_before_tail = next;
        //         } else {
        //             break;
        //         }
        //     }
        //
        //     let last = one_before_tail.next.take();
        //     last.map(|n| n.s)
        // }

        /// push element from head
        pub fn push(&mut self, s: String) -> &mut LinkedList {
            let mut new_head = Node::new(s);
            new_head.next = self.head.next.take();
            self.head.next = Some(Box::new(new_head));
            self
        }

        pub fn iter(&self) -> Iter {
            Iter {
                next: self.head.next.as_deref(),
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
}

fn main() {
    use link::LinkedList;

    let mut list = LinkedList::new();
    list.append("1".to_string())
        .append("2".to_string())
        .append("3".to_string());

    list.push("a".to_string())
        .push("b".to_string())
        .push("c".to_string());

    for s in list.iter() {
        print!("{},", s)
    }
    println!();

    // println!("pop: {}", list.pop_tail().unwrap_or("?".to_string()));
    // for s in list.iter() {
    //     print!("{},", s)
    // }
    // println!();
}

// If normal borrow checker cannot work, then the borrow checker bug still exist, use
// rustc -Zpolonius main.rs
// See https://github.com/rust-lang/rust/issues/62013
// See https://github.com/rust-lang/rust/issues/67957
