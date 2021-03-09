import edu.princeton.cs.algs4.StdIn;
import java.util.NoSuchElementException;

public class Permutation {
  public static void main(String[] args) {
    int i = 0;
    int n = Integer.parseInt(args[0]);
    RandomizedQueue<String> randomizedQueue = new RandomizedQueue<>();
    try {
      while (true) {
        boolean empty = StdIn.isEmpty();
        if (empty) {
          break;
        }
        String s = StdIn.readString();
        randomizedQueue.enqueue(s);
        i = i + 1;
      }
    } catch (NoSuchElementException e) {
      System.out.println("NoSuchElementException exception");
    }

    for (int j = 0; j < n; j++) {
      System.out.println(randomizedQueue.dequeue());
    }
  }
}
