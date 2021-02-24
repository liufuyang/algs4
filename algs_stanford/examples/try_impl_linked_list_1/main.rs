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

        pub fn next(&mut self) -> Option<&mut Node> {
            self.next.as_deref_mut()
        }

        pub fn insert(&mut self, n: Node) -> &mut Node {
            self.next.insert(Box::new(n))
        }

        pub fn iter(&self) -> Iter {
            Iter {
                next: self.next.as_deref(),
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
    use link::Node;

    let mut node = Node::init();
    node.insert(Node::new("1".to_string()))
        .insert(Node::new("2".to_string()))
        .insert(Node::new("3".to_string()));

    let mut p = &mut node;
    while let Some(n) = p.next() {
        println!("{}", n.get_value_ref());
        p = n;
    }

    for s in node.iter() {
        println!("{}", s)
    }
}
