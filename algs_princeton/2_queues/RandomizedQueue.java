import edu.princeton.cs.algs4.StdRandom;
import java.util.Iterator;
import java.util.NoSuchElementException;

public class RandomizedQueue<Item> implements Iterable<Item> {

  private Item[] items;
  private int head;
  private int tail; // tail is always the same or larger than head

  // construct an empty randomized queue
  public RandomizedQueue() {
    items = (Item[]) new Object[8];
    head = 0;
    tail = 0;
  }

  // is the randomized queue empty?
  public boolean isEmpty() {
    return tail - head == 0;
  }

  // return the number of items on the randomized queue
  public int size() {
    return tail - head;
  }

  // add the item
  public void enqueue(Item item) {
    if (item == null) {
      throw new IllegalArgumentException();
    }
    if (items.length == tail) {
      resize(Math.max(items.length, size() * 2));
    }
    items[tail] = item;
    tail++;

    swap(tail - 1, StdRandom.uniform(head, tail));
  }

  // remove and return a random item
  public Item dequeue() {
    if (this.isEmpty()) {
      throw new NoSuchElementException();
    }

    if (size() < items.length / 4) {
      resize(items.length / 2);
    }

    Item tmp = items[head];
    items[head] = null;
    head++;

    if (this.isEmpty()) {
      head = 0;
      tail = 0;
    }

    return tmp;
  }

  // return a random item (but do not remove it)
  public Item sample() {
    if (this.isEmpty()) {
      throw new NoSuchElementException();
    }

    int random = StdRandom.uniform(head, tail);
    return items[random];
  }

  private void swap(int a, int b) {
    Item o = items[a];
    items[a] = items[b];
    items[b] = o;
  }

  private void resize(int n) {
    Item[] copy = (Item[]) new Object[n];
    int size = size();
    // for(int i = 0; i < tail-head; i++) {
    //   copy[i] = items[head + i];
    // }
    if (tail - head >= 0) System.arraycopy(items, head, copy, 0, size);
    this.head = 0;
    this.tail = size;

    items = copy;
  }

  // return an independent iterator over items in random order
  public Iterator<Item> iterator() {
    return new RandomizedQueueIterator(this);
  }

  private class RandomizedQueueIterator implements Iterator<Item> {
    private final RandomizedQueue<Item> queue;

    public RandomizedQueueIterator(RandomizedQueue<Item> from) {
      queue = new RandomizedQueue<>();
      for (int i = from.head; i < from.tail; i++) {
        queue.enqueue(from.items[i]);
      }
    }

    @Override
    public boolean hasNext() {
      return !queue.isEmpty();
    }

    @Override
    public Item next() {
      if (hasNext()) {
        return queue.dequeue();
      }
      throw new NoSuchElementException();
    }
  }

  // unit testing (required)
  public static void main(String[] args) {
    RandomizedQueue<String> deque = new RandomizedQueue<>();
    deque.enqueue("a");
    deque.enqueue("b");
    System.out.println(deque.size());

    deque.enqueue("c");
    deque.enqueue("d");
    System.out.println(deque.size());

    printAll(deque);
    printAll(deque);
    printAll(deque);

    System.out.println(deque.dequeue());
    printAll(deque);

    System.out.println(deque.dequeue());
    printAll(deque);

    System.out.println(deque.dequeue());
    printAll(deque);

    System.out.println(deque.dequeue());
    printAll(deque);

    // deque.removeFirst();
    printAll(deque);

    System.out.println(deque.size());
  }

  private static <T> void printAll(RandomizedQueue<T> deque) {
    for (T s : deque) {
      System.out.print(s + ", ");
    }
    System.out.println();
  }
}
