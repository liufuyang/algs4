/// see src/linklist.rs
fn main() {
    use algs_stanford::linklist::LinkedList;

    let mut list = LinkedList::new();

    // insert from head
    list.push("a".to_string())
        .push("b".to_string())
        .push("c".to_string());

    for s in list.iter() {
        print!("{},", s)
    }
    println!();
    println!("size: {}", list.size());
    println!("pop: {}", list.pop().unwrap_or("?".to_string()));
    println!("pop: {}", list.pop().unwrap_or("?".to_string()));
    println!("pop: {}", list.pop().unwrap_or("?".to_string()));
    println!("pop: {}", list.pop().unwrap_or("?".to_string()));
    println!("size: {}", list.size());

    // insert from tail
    list.enqueue("1".to_string())
        .enqueue("2".to_string())
        .enqueue("3".to_string());

    for s in list.iter() {
        print!("{},", s)
    }
    println!();
    println!("size: {}", list.size());
    println!("dequeue: {}", list.dequeue().unwrap_or("?".to_string()));
    println!("dequeue: {}", list.dequeue().unwrap_or("?".to_string()));
    println!("dequeue: {}", list.dequeue().unwrap_or("?".to_string()));
    println!("dequeue: {}", list.dequeue().unwrap_or("?".to_string()));
    println!("size: {}", list.size());
}
