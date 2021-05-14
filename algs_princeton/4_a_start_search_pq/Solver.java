public class Solver {
  final Board initial;

  // find a solution to the initial board (using the A* algorithm)
  public Solver(Board initial) {
    this.initial = initial;
    // TODO
    System.out.println(initial);
    System.out.println("hamming: " + initial.hamming());
    System.out.println("manhattan: " + initial.manhattan());
  }

  // is the initial board solvable? (see below)
  public boolean isSolvable() {
    return false;
  }

  // min number of moves to solve initial board; -1 if unsolvable
  public int moves() {

    return 0;
  }

  // sequence of boards in a shortest solution; null if unsolvable
  public Iterable<Board> solution() {
    return null;
  }

  // test client (see below)
  public static void main(String[] args) {}
}
