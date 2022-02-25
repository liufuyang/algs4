/* *****************************************************************************
 *  Name:
 *  Date:
 *  Description:
 **************************************************************************** */

import edu.princeton.cs.algs4.Point2D;
import edu.princeton.cs.algs4.RectHV;

import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;

public class PointSET {
  private final Set<Point2D> set;

  public PointSET() { // construct an empty set of points
    set = new TreeSet<>();
  }

  public boolean isEmpty() // is the set empty?
      {
    return set.isEmpty();
  }

  public int size() // number of points in the set
      {
    return set.size();
  }

  // add the point to the set (if it is not already in the set)
  public void insert(Point2D p) {
    if (p == null) throw new IllegalArgumentException();
    set.add(p);
  }

  // does the set contain point p?
  public boolean contains(Point2D p) {
    if (p == null) throw new IllegalArgumentException();
    return set.contains(p);
  }

  // draw all points to standard draw
  public void draw() {
    set.forEach(Point2D::draw);
  }

  // all points that are inside the rectangle (or on the boundary)
  public Iterable<Point2D> range(RectHV rect) {
    if (rect == null) throw new IllegalArgumentException();
    return set.stream().filter(rect::contains).collect(Collectors.toCollection(TreeSet::new));
  }

  // a nearest neighbor in the set to point p; null if the set is empty
  public Point2D nearest(Point2D p) {
    if (p == null) throw new IllegalArgumentException();
    return set.stream().reduce((a, b) -> p.distanceTo(a) < p.distanceTo(b) ? a : b).orElse(null);
  }

  // unit testing of the methods (optional)
  public static void main(String[] args) {}
}
