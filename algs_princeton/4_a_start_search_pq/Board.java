import edu.princeton.cs.algs4.Queue;

public class Board {

  private final int[][] tiles;
  private int hamming = -1;
  private int manhattan = -1;

  // create a board from an n-by-n array of tiles,
  // where tiles[row][col] = tile at (row, col)
  public Board(int[][] tiles) {
    if (tiles == null) {
      throw new IllegalArgumentException();
    }
    int d = tiles.length;
    int[][] newTiles = new int[d][d];
    for (int r = 0; r < d; r++) {
      System.arraycopy(tiles[r], 0, newTiles[r], 0, d);
    }
    this.tiles = newTiles;
  }

  // string representation of this board
  public String toString() {
    StringBuilder buffer = new StringBuilder(this.dimension() + "\n");
    int maxDigit = (int) Math.log10(this.dimension() * this.dimension());
    for (int[] row : tiles) {
      for (int n : row) {
        int digit = n == 0 ? 1 : (int) Math.log10(n) + 1;
        String space = " ".repeat(maxDigit - digit + 1);
        buffer.append(" ").append(n).append(space);
      }
      buffer.append('\n');
    }
    return buffer.toString();
  }

  //  // board dimension n
  public int dimension() {
    return tiles.length;
  }

  //  // number of tiles out of place
  public int hamming() {
    if (hamming != -1) return hamming;
    int tmp = 0;
    for (int r = 0; r < dimension(); r++) {
      for (int c = 0; c < dimension(); c++) {
        int correct = dimension() * (r) + c + 1;
        if (tiles[r][c] != correct && !(r == dimension() - 1 && c == dimension() - 1)) tmp++;
      }
    }
    hamming = tmp;
    return hamming;
  }

  //  // sum of Manhattan distances between tiles and goal
  public int manhattan() {
    if (manhattan != -1) return manhattan;
    int tmp = 0;
    for (int r = 0; r < dimension(); r++) {
      for (int c = 0; c < dimension(); c++) {
        int correct = dimension() * (r) + c + 1;
        if (tiles[r][c] == correct) continue;
        if (tiles[r][c] == 0) {
          continue;
        } else {
          tmp +=
              Math.abs((tiles[r][c] - 1) % dimension() - c)
                  + Math.abs((tiles[r][c] - 1) / dimension() - r);
        }
      }
    }
    manhattan = tmp;
    return manhattan;
  }

  //  // is this board the goal board?
  public boolean isGoal() {
    return this.hamming() == 0;
  }

  //  // does this board equal y?
  @Override
  public boolean equals(Object that) {
    if (this == that) {
      return true;
    }
    if (that == null || getClass() != that.getClass()) {
      return false;
    }
    Board board = (Board) that;

    if (this.dimension() != board.dimension()) return false;
    for (int r = 0; r < dimension(); r++) {
      for (int c = 0; c < dimension(); c++) {
        if (this.tiles[r][c] != board.tiles[r][c]) return false;
      }
    }
    return true;
  }

  //
  //  // all neighboring boards
  public Iterable<Board> neighbors() {
    int zeroR = -1;
    int zeroC = -1;
    for (int r = 0; r < dimension(); r++) {
      for (int c = 0; c < dimension(); c++) {
        if (tiles[r][c] == 0) {
          zeroR = r;
          zeroC = c;
        }
      }
    }

    Queue<Board> queue = new Queue<>();
    // Go down
    if (zeroR < dimension() - 1) {
      Board newBoard = this.cloneBoard();
      newBoard.swap(zeroR, zeroC, zeroR + 1, zeroC);
      queue.enqueue(newBoard);
    }
    // Go right
    if (zeroC < dimension() - 1) {
      Board newBoard = this.cloneBoard();
      newBoard.swap(zeroR, zeroC, zeroR, zeroC + 1);
      queue.enqueue(newBoard);
    }
    // Go left
    if (zeroC > 0) {
      Board newBoard = this.cloneBoard();
      newBoard.swap(zeroR, zeroC, zeroR, zeroC - 1);
      queue.enqueue(newBoard);
    }
    // Go up
    if (zeroR > 0) {
      Board newBoard = this.cloneBoard();
      newBoard.swap(zeroR, zeroC, zeroR - 1, zeroC);
      queue.enqueue(newBoard);
    }
    return queue;
  }

  private void swap(int aR, int aC, int bR, int bC) {
    int n = this.tiles[aR][aC];
    this.tiles[aR][aC] = this.tiles[bR][bC];
    this.tiles[bR][bC] = n;
  }

  //  // a board that is obtained by exchanging any pair of tiles
  public Board twin() {
    Board clone = this.cloneBoard();
    int aR = -1;
    int aC = -1;
    int bR = -1;
    int bC = -1;
    for (int r = 0; r < dimension(); r++) {
      for (int c = 0; c < dimension(); c++) {
        if ((aR == -1 || aC == -1) && clone.tiles[r][c] != 0) {
          aR = r;
          aC = c;
        } else if ((bR == -1 || bC == -1) && clone.tiles[r][c] != 0) {
          bR = r;
          bC = c;
        } else if (aR != -1 && aC != -1 && bR != -1 && bC != -1) {
          break;
        }
      }
    }
    clone.swap(aR, aC, bR, bC);
    return clone;
  }

  private Board cloneBoard() {
    int[][] newTiles = new int[dimension()][dimension()];
    for (int r = 0; r < dimension(); r++) {
      if (dimension() >= 0) System.arraycopy(this.tiles[r], 0, newTiles[r], 0, dimension());
    }
    return new Board(newTiles);
  }

  // unit testing (not graded)
  public static void main(String[] args) {
    System.out.println("A *");
  }
}
