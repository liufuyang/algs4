//! Just another impl of binary search tree

use crate::link_list::LinkedList;

pub struct Node<K, V> {
    key: K,
    value: V,
    // Box is needed to allow calculation of Node's size avoid recursion
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

pub struct BSTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Node<K, V> {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
    pub fn set_left(&mut self, n: Node<K, V>) {
        self.left = Some(Box::new(n))
    }
    pub fn set_right(&mut self, n: Node<K, V>) {
        self.right = Some(Box::new(n))
    }
    pub fn left(&self) -> &Option<Box<Node<K, V>>> {
        &self.left
    }
    pub fn right(&self) -> &Option<Box<Node<K, V>>> {
        &self.right
    }
}

/// Inorder recursive version
fn inorder_rec<'a, K, V>(n: &'a Option<Box<Node<K, V>>>, queue: &mut LinkedList<&'a K>)
{
    if n.is_none() {
        return;
    }
    let n = n.as_ref().unwrap();
    inorder_rec(n.left(), queue);     // left (recursive)
    queue.enqueue(&n.key);       // mid
    inorder_rec(n.right(), queue);    // right (recursive)
}


/// Inorder loop version
fn inorder_loop<'a, K, V>(n: &'a Option<Box<Node<K, V>>>, queue: &mut LinkedList<&'a K>)
{
    if n.is_none() {
        return;
    }
    let mut stack = LinkedList::new();
    let mut curr = n.as_ref();
    while curr.is_some() || !stack.is_empty() {
        while curr.is_some() {
            let left = curr.unwrap().left();
            stack.push(curr);
            curr = left.as_ref();
        }
        curr = stack.pop().unwrap();
        queue.enqueue(&curr.unwrap().key);
        curr = curr.unwrap().right().as_ref();
    }
}

/// Bottom-up recursive version
fn bottom_up_order_rec<'a, K, V>(n: &'a Option<Box<Node<K, V>>>, queue: &mut LinkedList<(&'a K, usize)>)
{
    if n.is_none() {
        return;
    }
    let mut stack = LinkedList::new();
    let curr = n.as_ref();
    stack.enqueue((curr, 0usize));
    while !stack.is_empty() {
        let (curr, level) = stack.dequeue().unwrap();
        let left = curr.unwrap().left();
        if left.is_some() {
            stack.enqueue((left.as_ref(), level + 1));
        }
        let right = curr.unwrap().right();
        if right.is_some() {
            stack.enqueue((right.as_ref(), level + 1));
        }
        queue.enqueue((&curr.unwrap().key, level));
    }
}

impl<K, V> BSTree<K, V> {
    pub fn new(n: Node<K, V>) -> BSTree<K, V> {
        BSTree {
            root: Some(Box::new(n))
        }
    }

    pub fn keys(&self) -> LinkedList<&K> {
        let mut queue = LinkedList::new();
        inorder_rec(&self.root, &mut queue);
        queue
    }

    pub fn keys_loop(&self) -> LinkedList<&K> {
        let mut queue = LinkedList::new();
        inorder_loop(&self.root, &mut queue);
        queue
    }

    pub fn keys_bottom_up(&self) -> LinkedList<(&K, usize)> {
        let mut queue = LinkedList::new();
        bottom_up_order_rec(&self.root, &mut queue);
        queue
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search_tree::*;

    #[test]
    fn inorder_works() {
        /* Constructed binary tree is
                  1
                /   \
              2      3
            /  \
          4     5
        */
        let mut root = Node::new(1, ());
        let n4 = Node::new(4, ());
        let n5 = Node::new(5, ());
        let mut n2 = Node::new(2, ());
        let n3 = Node::new(3, ());
        n2.set_left(n4);
        n2.set_right(n5);
        root.set_left(n2);
        root.set_right(n3);

        let bst = BSTree::new(root);
        for k in bst.keys().iter() {
            print!("{}, ", k);
        }
        println!();

        for k in bst.keys_loop().iter() {
            print!("{}, ", k);
        }
        println!();

        let key_bottom_up = bst.keys_bottom_up();
        let key_bottom_up = key_bottom_up.iter().collect::<Vec<&(&i32, usize)>>();
        let mut level = 0usize;
        let size = key_bottom_up.len() as f64;
        let size = size.log2().ceil() as usize;
        print!("{}", "   ".repeat( size));
        for (k, l) in key_bottom_up.iter() {
            if l > &level {
                level = *l;
                let n = 2usize.pow(level as u32 -1 );
                print!("\n{}", "   ".repeat(size - n));
                print!("{}", " / \\ ".repeat(n));
                print!("\n{}", "   ".repeat(size - n));
            }
            print!("{}  ", k);
        }
        println!();
    }
}
