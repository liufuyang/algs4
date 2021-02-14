import edu.princeton.cs.algs4.StdRandom;
import edu.princeton.cs.algs4.StdStats;

public class PercolationStats {

  private final int trials;
  private final double[] openFractions;
  // perform independent trials on an n-by-n grid
  public PercolationStats(int n, int trials) {
    if(n <= 1 || trials <= 1) {
      throw new IllegalArgumentException();
    }

    this.trials = trials;
    this.openFractions = new double[trials];

    for (int t = 0; t < trials; t++) {
      Percolation percolation = new Percolation(n);

      while (!percolation.percolates()) {
        int r = StdRandom.uniform(n) + 1;
        int col = StdRandom.uniform(n) + 1;
        percolation.open(r, col);
      }

      openFractions[t] = ((double) percolation.numberOfOpenSites()) / n / n;
    }
  }

  // sample mean of percolation threshold
  public double mean() {
    return StdStats.mean(openFractions);
  }

  // sample standard deviation of percolation threshold
  public double stddev() {
    return StdStats.stddev(openFractions);
  }

  // low endpoint of 95% confidence interval
  public double confidenceLo() {
    return mean() - 1.96 * stddev() / Math.sqrt(trials);
  }

  // high endpoint of 95% confidence interval
  public double confidenceHi() {
    return mean() + 1.96 * stddev() / Math.sqrt(trials);
  }

  // test client (see below)
  public static void main(String[] args) {
    int n = Integer.parseInt(args[0]);
    int t = Integer.parseInt(args[1]);

    PercolationStats percolationStats = new PercolationStats(n, t);

    System.out.printf("mean                    = %f\n", percolationStats.mean());
    System.out.printf("stddev                  = %f\n", percolationStats.stddev());
    System.out.printf(
        "95%% confidence interval = [%f, %f]\n",
        percolationStats.confidenceLo(), percolationStats.confidenceHi());
  }
}
