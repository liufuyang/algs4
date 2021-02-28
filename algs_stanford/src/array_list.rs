/// Array type stack queue made via Vec
///
/// (), N1(), N2(), N3()
/// ----^start------^tail
///
/// Push, pop from tail as a stack; enqueue from tail, and dequeue from head (same as pop) as a queue;
///
/// As stack and queue adding element from the same directions, the iter will work for queue but not stack.
/// Call `iter().rev()` for stack's iter
use std::mem;

pub struct ArrayList<T> {
    elements: Vec<T>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T> ArrayList<T> {
    pub fn new() -> ArrayList<T> {
        ArrayList {
            elements: Vec::new(),
            head: None,
            tail: None,
        }
    }

    pub fn with_capacity(n: usize) -> ArrayList<T> {
        ArrayList {
            elements: Vec::with_capacity(n),
            head: None,
            tail: None,
        }
    }

    pub fn size(&self) -> usize {
        match (self.head, self.tail) {
            (Some(head), Some(tail)) => {
                if head <= tail {
                    tail - head + 1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// push element from head
    pub fn push(&mut self, element: T) -> &mut ArrayList<T> {
        self.elements.push(element);
        match self.tail {
            Some(i) => self.tail = Some(i + 1),
            None => {
                self.head = Some(0);
                self.tail = Some(0);
            }
        }
        self
    }

    /// pop element from head
    pub fn pop(&mut self) -> Option<T> {
        match self.tail {
            Some(tail) => {
                let head = self.head.unwrap();
                if head <= tail {
                    match tail.checked_sub(1) {
                        Some(new_tail) => {
                            self.tail = Some(new_tail);
                        }
                        None => {
                            self.tail = None;
                            self.head = None;
                        }
                    }
                    self.elements.pop()
                } else {
                    self.elements.clear();
                    self.head = None;
                    self.tail = None;
                    None
                }
            }
            None => None,
        }
    }

    /// enqueue element to list at the end, same as push
    pub fn enqueue(&mut self, element: T) -> &mut ArrayList<T> {
        self.push(element)
    }

    /// dequeue element from head, the same as pop
    pub fn dequeue(&mut self) -> Option<T>
    where
        T: Default,
    {
        match self.head {
            Some(head) => {
                let tail = self.tail.unwrap();
                if head <= tail {
                    self.head = Some(head + 1);
                    Some(mem::replace(&mut self.elements[head], T::default()))
                } else {
                    self.elements.clear();
                    self.head = None;
                    self.tail = None;
                    None
                }
            }
            None => None,
        }
    }

    /// A queue FIFO order iter. Call `iter().rev()` for stack's iter
    pub fn iter(&self) -> core::slice::Iter<T> {
        match (self.head, self.tail) {
            (Some(head), Some(tail)) => {
                if head <= tail {
                    self.elements[head..tail + 1].iter()
                } else {
                    [].iter()
                }
            }
            _ => [].iter(),
        }
    }
}
