import edu.princeton.cs.algs4.MinPQ;
import edu.princeton.cs.algs4.Stack;

public class Solver {

  private final Stack<Board> boardStack = new Stack<>();
  private final int move;

  // find a solution to the initial board (using the A* algorithm)
  public Solver(Board initial) {
    if (initial == null) {
      throw new IllegalArgumentException();
    }
    MinPQ<State> frontierInitial = new MinPQ<>();
    MinPQ<State> frontierTwin = new MinPQ<>();
    Board twin = initial.twin();
    frontierInitial.insert(new State(initial));
    frontierTwin.insert(new State(twin));

    State currentInitial;
    State currentTwin;
    while (true) {
      currentInitial = solve1Step(frontierInitial);
      currentTwin = solve1Step(frontierTwin);
      if (currentTwin.getBoard().isGoal()) {
        move = -1;
        return;
      }
      if (currentInitial.getBoard().isGoal()) {
        move = currentInitial.move;
        State end = currentInitial;
        while (end != null) {
          boardStack.push(end.board);
          end = end.prev;
        }
        return;
      }
    }
  }

  private State solve1Step(MinPQ<State> frontier) {
    State state = frontier.delMin();
    state
        .getBoard()
        .neighbors()
        .forEach(
            neighbor -> {
              State prev = state;
              boolean visited = false;
              while (prev != null) {
                if (prev.board.equals(neighbor)) {
                  visited = true;
                  break;
                }
                prev = prev.prev;
              }
              if (!visited) {
                frontier.insert(state.chain(neighbor));
              }
            });
    //    System.out.println("frontier size:" + frontier.size());
    //    System.out.println("visited size:" + visited.size());
    //    System.out.println(state.board);
    return state;
  }

  // is the initial board solvable? (see below)
  public boolean isSolvable() {
    return move > -1;
  }

  // min number of moves to solve initial board; -1 if unsolvable
  public int moves() {
    return move;
  }

  // sequence of boards in a shortest solution; null if unsolvable
  public Iterable<Board> solution() {
    if (!isSolvable()) return null;
    return boardStack;
  }

  private class State implements Comparable<State> {
    private final Board board;
    private final int move;
    private final State prev;

    public State(Board board) {
      this.board = board;
      this.move = 0;
      this.prev = null;
    }

    public State(Board board, int move, State prev) {
      this.board = board;
      this.move = move;
      this.prev = prev;
    }

    /** IMPORTANT: increase move number by one when chaining */
    public State chain(Board newBoard) {
      return new State(newBoard, this.move + 1, this);
    }

    public Board getBoard() {
      return board;
    }

    private int weight() {
      return board.manhattan() + move;
    }

    @Override
    public int compareTo(State that) {
      return Integer.compare(this.weight(), that.weight());
    }
  }

  // test client (see below)
//  public static void main(String[] args) {
//    // create initial board from file
//    In in = new In(args[0]);
//    int n = in.readInt();
//    int[][] tiles = new int[n][n];
//    for (int i = 0; i < n; i++) for (int j = 0; j < n; j++) tiles[i][j] = in.readInt();
//    Board initial = new Board(tiles);
//
//    // solve the puzzle
//    Solver solver = new Solver(initial);
//
//    // print solution to standard output
//    if (!solver.isSolvable()) StdOut.println("No solution possible");
//    else {
//      StdOut.println("Minimum number of moves = " + solver.moves());
//      for (Board board : solver.solution()) StdOut.println(board);
//    }
//  }
}
