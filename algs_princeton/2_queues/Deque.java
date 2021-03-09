import java.util.Iterator;
import java.util.NoSuchElementException;

public class Deque<Item> implements Iterable<Item> {

  static class Node<Item> {
    private Node<Item> prev;
    private Node<Item> next;
    private final Item item;

    public Node(Item item) {
      if (item == null) {
        throw new IllegalArgumentException();
      }
      this.item = item;
    }
  }

  private Node<Item> head;
  private Node<Item> tail;
  private int size = 0;

  // construct an empty deque
  public Deque() {}

  // is the deque empty?
  public boolean isEmpty() {
    return this.head == null;
  }

  // return the number of items on the deque
  public int size() {
    return size;
  }

  // add the item to the front
  public void addFirst(Item item) {
    Node<Item> newNode = new Node<>(item);
    if (this.head == null) {
      this.head = newNode;
      this.tail = newNode;
    } else {
      Node<Item> tmp = this.head;
      this.head = newNode;
      this.head.next = tmp;
      tmp.prev = this.head;
    }
    size++;
  }

  // add the item to the back
  public void addLast(Item item) {
    Node<Item> newNode = new Node<>(item);
    if (this.head == null) {
      this.head = newNode;
      this.tail = newNode;
    } else {
      Node<Item> tmp = this.tail;
      this.tail = newNode;
      this.tail.prev = tmp;
      tmp.next = this.tail;
    }
    size++;
  }

  // remove and return the item from the front
  public Item removeFirst() {
    if (this.head == null) {
      throw new NoSuchElementException();
    }
    Node<Item> tmp = this.head;
    this.head = tmp.next;
    if (this.head == null) {
      this.tail = null;
    } else {
      this.head.prev = null;
    }
    size--;

    return tmp.item;
  }

  // remove and return the item from the back
  public Item removeLast() {
    if (this.head == null) {
      throw new NoSuchElementException();
    }
    Node<Item> tmp = this.tail;
    this.tail = tmp.prev;

    if (this.tail == null) {
      this.head = null;
    } else {
      this.tail.next = null;
    }
    size--;

    return tmp.item;
  }

  // return an iterator over items in order from front to back
  @Override
  public Iterator<Item> iterator() {
    return new DequeIterator(this);
  }

  class DequeIterator implements Iterator<Item> {
    private Node<Item> current;

    public DequeIterator(Deque<Item> deque) {
      this.current = deque.head;
    }

    @Override
    public boolean hasNext() {
      return this.current != null;
    }

    @Override
    public Item next() {
      if (hasNext()) {
        Node<Item> tmp = current;
        current = current.next;
        return tmp.item;
      }
      throw new NoSuchElementException();
    }
  }

  // unit testing (required)
  public static void main(String[] args) {
    Deque<String> deque = new Deque<>();
    deque.addFirst("a");
    deque.addFirst("b");
    System.out.println(deque.size());

    deque.addLast("c");
    deque.addLast("d");
    System.out.println(deque.size());

    printAll(deque);

    deque.removeFirst();
    printAll(deque);

    deque.removeFirst();
    printAll(deque);

    deque.removeLast();
    printAll(deque);

    deque.removeLast();
    printAll(deque);

    // deque.removeFirst();
    printAll(deque);

    System.out.println(deque.size());
  }

  private static <T> void printAll(Deque<T> deque) {
    for (T s : deque) {
      System.out.print(s + ", ");
    }
    System.out.println();
  }
}
