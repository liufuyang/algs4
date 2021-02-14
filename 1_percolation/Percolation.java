import edu.princeton.cs.algs4.WeightedQuickUnionUF;

/** Percolation. */
public class Percolation {

  /** WeightedQuickUnionUF. */
  private final WeightedQuickUnionUF uf;

  private final int n;
  private final int[] openReg;

  // creates n-by-n grid, with all sites initially blocked
  /** Percolation. */
  public Percolation(int n) {
    if (n <= 0) {
      throw new IllegalArgumentException("n must above 0");
    }
    this.n = n;
    uf = new WeightedQuickUnionUF(n * n + 2); // 0 and n*n + 1 is the virtual site

    this.openReg = new int[n * n];
    for (int i = 0; i < n * n; i++) {
      this.openReg[i] = 0;
    }
  }

  /** opens the site (row, col) if it is not open already. */
  public void open(int row, int col) {
    check(row, col);
    int ind = index(row, col);
    if (openReg[ind] == 0) {
      // not opened, we open it
      if (checkBool(row - 1, col)) {
        if (isOpen(row - 1, col)) {
          uf.union(ind + 1, ind - n + 1);
        }
      }
      if (checkBool(row + 1, col)) {
        if (isOpen(row + 1, col)) {
          uf.union(ind + 1, ind + n + 1);
        }
      }
      if (checkBool(row, col - 1)) {
        if (isOpen(row, col - 1)) {
          uf.union(ind + 1, ind - 1 + 1);
        }
      }
      if (checkBool(row, col + 1)) {
        if (isOpen(row, col + 1)) {
          uf.union(ind + 1, ind + 1 + 1);
        }
      }

      // handle end row case
      if (row == 1) {
        uf.union(col, 0);
      }
      if (row == n) {
        uf.union((n - 1) * n + col, n * n + 1);
      }

      openReg[ind] = 1;
    }
  }

  // is the site (row, col) open?
  public boolean isOpen(int row, int col) {
    check(row, col);
    int ind = index(row, col);
    return openReg[ind] == 1;
  }

  // is the site (row, col) full?
  public boolean isFull(int row, int col) {
    check(row, col);
    return isOpen(row, col) && connected(index(row, col) + 1, 0);
  }

  // returns the number of open sites
  public int numberOfOpenSites() {
    int open = 0;
    for (int i = 0; i < openReg.length; i++) {
      if (openReg[i] == 1) {
        open += 1;
      }
    }
    return open;
  }

  // does the system percolate?
  public boolean percolates() {
    return connected(0, n * n + 1);
  }

  private void check(int a, int b) {
    if (!checkBool(a, b)) {
      throw new IllegalArgumentException();
    }
  }

  private boolean checkBool(int a, int b) {
    return a >= 1 && a <= n && b >= 1 && b <= n;
  }

  private boolean connected(int p, int q) {
    return uf.find(p) == uf.find(q);
  }

  private int index(int row, int col) {
    return (row - 1) * n + col - 1;
  }
}
