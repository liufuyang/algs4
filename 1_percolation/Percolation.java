import edu.princeton.cs.algs4.WeightedQuickUnionUF;

/** Percolation. */
public class Percolation {

  /** A union find structure for telling whether percolation happens. */
  private final WeightedQuickUnionUF uf;

  private final int n;
  private final State[] state;
  private int opened = 0;
  private boolean percolated = false;

  // creates n-by-n grid, with all sites initially blocked
  /** Percolation. */
  public Percolation(int n) {
    if (n <= 0) {
      throw new IllegalArgumentException("n must above 0");
    }
    this.n = n;
    uf = new WeightedQuickUnionUF(n * n + 1); // 0 and n*n + 1 is the virtual site

    this.state = new State[n * n + 1];
    for (int i = 0; i < n * n + 1; i++) {
      this.state[i] = new State();
    }
    this.state[0].setTopConnected(true);
  }

  /** opens the site (row, col) if it is not open already. */
  public void open(int row, int col) {
    check(row, col);
    int ind = index(row, col);
    if (!state[ind + 1].isOpened()) {
      // not opened, we open it
      // handle end row case
      if (row == 1) {
        uf.union(col, 0);
        state[uf.find(ind + 1)].setTopConnected(true);
        state[ind + 1].setTopConnected(true);
      }
      if (row == n) {
        // uf.union((n - 1) * n + col, n * n + 1);
        state[uf.find(ind + 1)].setBottomConnected(true);
        state[ind + 1].setBottomConnected(true);
      }

      if (row > 1) {
        if (isOpen(row - 1, col)) {
          boolean topC =
              state[uf.find(ind + 1 - n)].isTopConnected() || state[ind + 1].isTopConnected();
          boolean botC =
              state[uf.find(ind + 1 - n)].isBottomConnected() || state[ind + 1].isBottomConnected();

          uf.union(ind + 1, ind - n + 1);

          state[uf.find(ind + 1)].setTopConnected(topC);
          state[uf.find(ind + 1)].setBottomConnected(botC);
          state[ind + 1].setTopConnected(topC);
          state[ind + 1].setBottomConnected(botC);
        }
      }
      if (row < n) {
        if (isOpen(row + 1, col)) {
          boolean topC =
              state[uf.find(ind + 1 + n)].isTopConnected() || state[ind + 1].isTopConnected();
          boolean botC =
              state[uf.find(ind + 1 + n)].isBottomConnected() || state[ind + 1].isBottomConnected();

          uf.union(ind + 1, ind + n + 1);

          state[uf.find(ind + 1)].setTopConnected(topC);
          state[uf.find(ind + 1)].setBottomConnected(botC);
          state[ind + 1].setTopConnected(topC);
          state[ind + 1].setBottomConnected(botC);
        }
      }
      if (col > 1) {
        if (isOpen(row, col - 1)) {
          boolean topC =
              state[uf.find(ind + 1 - 1)].isTopConnected() || state[ind + 1].isTopConnected();
          boolean botC =
              state[uf.find(ind + 1 - 1)].isBottomConnected() || state[ind + 1].isBottomConnected();

          uf.union(ind + 1, ind + 1 - 1);

          state[uf.find(ind + 1)].setTopConnected(topC);
          state[uf.find(ind + 1)].setBottomConnected(botC);
          state[ind + 1].setTopConnected(topC);
          state[ind + 1].setBottomConnected(botC);
        }
      }
      if (col < n) {
        if (isOpen(row, col + 1)) {
          boolean topC =
              state[uf.find(ind + 1 + 1)].isTopConnected() || state[ind + 1].isTopConnected();
          boolean botC =
              state[uf.find(ind + 1 + 1)].isBottomConnected() || state[ind + 1].isBottomConnected();

          uf.union(ind + 1, ind + 1 + 1);

          state[uf.find(ind + 1)].setTopConnected(topC);
          state[uf.find(ind + 1)].setBottomConnected(botC);
          state[ind + 1].setTopConnected(topC);
          state[ind + 1].setBottomConnected(botC);
        }
      }

      if (state[uf.find(ind + 1)].isTopConnected() && state[uf.find(ind + 1)].isBottomConnected()) {
        percolated = true;
      }

      state[ind + 1].setOpened(true);
      opened++;
    }
  }

  // is the site (row, col) open?
  public boolean isOpen(int row, int col) {
    check(row, col);
    int ind = index(row, col);
    return state[ind + 1].isOpened();
  }

  // is the site (row, col) full?
  public boolean isFull(int row, int col) {
    check(row, col);
    int ind = index(row, col);
    return connected(ind + 1, 0);
  }

  // returns the number of open sites
  public int numberOfOpenSites() {
    return opened;
  }

  // does the system percolate?
  public boolean percolates() {
    return percolated;
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

  private class State {
    private boolean opened = false;
    private boolean topConnected = false;
    private boolean bottomConnected = false;

    public boolean isOpened() {
      return opened;
    }

    public void setOpened(boolean opened) {
      this.opened = opened;
    }

    public boolean isTopConnected() {
      return topConnected;
    }

    public void setTopConnected(boolean topConnected) {
      this.topConnected = topConnected;
    }

    public boolean isBottomConnected() {
      return bottomConnected;
    }

    public void setBottomConnected(boolean bottomConnected) {
      this.bottomConnected = bottomConnected;
    }
  }
}
