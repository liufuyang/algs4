public class Board {

  private final int[][] tiles;
  private int hamming = -1;
  private int manhattan = -1;

  // create a board from an n-by-n array of tiles,
  // where tiles[row][col] = tile at (row, col)
  public Board(int[][] tiles) {
    this.tiles = tiles;
  }

  // string representation of this board
  public String toString() {
    StringBuffer buffer = new StringBuffer(this.dimension() + "\n");
    int maxDigit = (int) Math.log10(this.dimension() * this.dimension());
    for (int[] row : tiles) {
      for (int n : row) {
        int digit = n == 0 ? 1 : (int) Math.log10(n) + 1;
        String space = " ".repeat(maxDigit - digit + 1);
        buffer.append(space).append(n).append(" ");
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
    for(int r=0; r<dimension(); r++ ) {
      for(int c=0; c<dimension(); c++ ) {
          int correct = dimension()*(r) + c + 1;
          correct = correct == dimension()*dimension() ? 0 : correct;
          if (tiles[r][c] != correct) tmp++;
      }
    }
    hamming = tmp;
    return hamming;
  }

  //  // sum of Manhattan distances between tiles and goal
  public int manhattan() {
    if (manhattan != -1) return manhattan;
    return 0;
  }

  //  // is this board the goal board?
  //  public boolean isGoal()
  //
  //  // does this board equal y?
  //  public boolean equals(Object y)
  //
  //  // all neighboring boards
  //  public Iterable<Board> neighbors()
  //
  //  // a board that is obtained by exchanging any pair of tiles
  //  public Board twin()

  // unit testing (not graded)
  public static void main(String[] args) {
    System.out.println("A *");
  }
}
