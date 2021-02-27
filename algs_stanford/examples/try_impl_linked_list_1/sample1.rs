use std::borrow::BorrowMut;

struct Node {
    next: Option<Box<Node>>,
}

fn add1_working<'a>(mut node: &'a mut Node) {
    while let Some(ref mut next) = node.next {
        node = next;
    }
    node.next = Some(Box::new(Node { next: None }));
}

fn add1_broken<'a>(mut node: &'a mut Node) {
    while let Some(next) = node.next.as_deref_mut() {
        node = next;
    }
    node.next = Some(Box::new(Node { next: None }));
}

fn add1_borken_ergonomics(mut node: &mut Node) {
    while let Some(next) = &mut node.next {
        node = next;
    }
    node.next = Some(Box::new(Node { next: None }));
}

fn add1_borken_explicit<'a>(mut node: &'a mut Node) {
    while let Some(ref mut next) = *&mut node.next {
        node = next;
    }
    node.next = Some(Box::new(Node { next: None }));
}

fn add1_borken_ergonomics2(mut node: &mut Node) {
    let mut anchor = node.next.as_deref_mut();
    while let Some(next) = anchor {
        anchor = next.next.as_deref_mut();
    }
    anchor.unwrap().next = Some(Box::new(Node { next: None }));
}
