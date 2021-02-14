import edu.princeton.cs.algs4.QuickFindUF;
import java.util.Arrays;
import java.util.Optional;
import java.util.stream.Stream;

public class Percolation {

  private QuickFindUF uf;
  private int n;
  private int[] openReg;

  // creates n-by-n grid, with all sites initially blocked
  public Percolation(int n) {
    if (n < 0) {
      throw new IllegalArgumentException("n must above 0");
    }
    this.n = n;
    uf = new QuickFindUF(n * n + 2); // 0 and n*n + 1 is the virtual site

    for (int i = 0; i < n; i++) {
      uf.union(i + 1, 0);
      uf.union((n - 1) * n + i + 1, n * n + 1);
    }

    this.openReg = new int[n * n];
    for (int i = 0; i < n * n; i++) {
      this.openReg[i] = 0;
    }
  }

  // opens the site (row, col) if it is not open already
  public void open(int row, int col) {
    check(row, col);
    int ind = (row - 1) * n + col - 1;
    if (openReg[ind] == 0) {
      // not opened, we open it
      Stream.of(
              position(row - 1, col, n),
              position(row + 1, col, n),
              position(row, col - 1, n),
              position(row, col + 1, n))
          .flatMap(Optional::stream)
          .forEach(
              p -> {
                if (isOpen(p.row, p.col)) {
                  uf.union(ind + 1, p.index() + 1);
                }
              });

      openReg[ind] = 1;
    }
  }

  // is the site (row, col) open?
  public boolean isOpen(int row, int col) {
    check(row, col);
    int ind = (row - 1) * n + col - 1;
    return openReg[ind] == 1;
  }

  // is the site (row, col) full?
  public boolean isFull(int row, int col) {
    check(row, col);
    return isOpen(row, col) && uf.connected((row - 1) * n + col - 1 + 1, 0);
  }

  // returns the number of open sites
  public int numberOfOpenSites() {
    return (int) Arrays.stream(openReg).filter(o -> o == 1).count();
  }

  // does the system percolate?
  public boolean percolates() {
    return uf.connected(0, n * n + 1);
  }

  private void check(int a, int b) {
    if (a < 1 || a > n || b < 1 || b > n) {
      throw new IllegalArgumentException();
    }
  }

  class Position {
    public int row;
    public int col;

    public Position(int row, int col) {
      this.row = row;
      this.col = col;
    }

    public int index() {
      return (row - 1) * n + col - 1;
    }
  }

  public Optional<Position> position(int row, int col, int n) {
    if (row < 1 || row > n || col < 1 || col > n) {
      return Optional.empty();
    }
    return Optional.of(new Position(row, col));
  }

  // test client (optional)
  public static void main(String[] args) {}
}
