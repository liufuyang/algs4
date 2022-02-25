/* *****************************************************************************
 *  Name:
 *  Date:
 *  Description:
 **************************************************************************** */

import edu.princeton.cs.algs4.Point2D;
import edu.princeton.cs.algs4.Queue;
import edu.princeton.cs.algs4.RectHV;

import java.util.Comparator;
import java.util.Iterator;

public class KdTree {
  private Node root;
  private int size;

  // construct an empty set of points
  public KdTree() {
    root = null;
  }

  // is the set empty?
  public boolean isEmpty() {
    return root == null;
  }

  // number of points in the set
  public int size() {
    return size;
  }

  // add the point to the set (if it is not already in the set)
  public void insert(Point2D p) {
    if (p == null) throw new IllegalArgumentException();

    if (root == null) {
      root = new Node(p, Point2D.X_ORDER);
      size = 1;
      return;
    }

    boolean inserted = root.insert(p);
    if (inserted) size++;
  }

  // does the set contain point p?
  public boolean contains(Point2D p) {
    if (p == null) throw new IllegalArgumentException();
    return root.search(p).pointEquals(p);
  }

  // draw all points to standard draw
  public void draw() {
    root.draw();
  }

  // all points that are inside the rectangle (or on the boundary)
  public Iterable<Point2D> range(RectHV rect) {
    if (rect == null) throw new IllegalArgumentException();
    return () -> new NodeIterator(this.root, rect);
  }

  // a nearest neighbor in the set to point p; null if the set is empty
  public Point2D nearest(Point2D p) {
    if (p == null) throw new IllegalArgumentException();
    Queue<Node> queue = new Queue<>();
    queue.enqueue(root);
    Point2D nearest = root.getPoint2D();
    while (!queue.isEmpty()) {
      Node current = queue.dequeue();
      if (current.getPoint2D().distanceTo(p) < nearest.distanceTo(p))
        nearest = current.getPoint2D();

      if (current.getComparator() == Point2D.X_ORDER) {
        // X_ORDER
        if (current.getComparator().compare(p, current.getPoint2D()) < 0) { // p on the left
          if (current.getLeft() != null) queue.enqueue(current.getLeft());
          if (nearest.distanceTo(p) > current.getPoint2D().x() - p.x()) {
            if (current.getRight() != null) queue.enqueue(current.getRight());
          }
        } else { // p on the right
          if (current.getRight() != null) queue.enqueue(current.getRight());
          if (nearest.distanceTo(p) > p.x() - current.getPoint2D().x()) {
            if (current.getLeft() != null) queue.enqueue(current.getLeft());
          }
        }
      } else {
        // Y_ORDER
        if (current.getComparator().compare(p, current.getPoint2D()) < 0) { // p on the bottom
          if (current.getLeft() != null) queue.enqueue(current.getLeft());
          if (nearest.distanceTo(p) > current.getPoint2D().y() - p.y()) {
            if (current.getRight() != null) queue.enqueue(current.getRight());
          }
        } else { // p on the upper
          if (current.getRight() != null) queue.enqueue(current.getRight());
          if (nearest.distanceTo(p) > p.y() - current.getPoint2D().y()) {
            if (current.getLeft() != null) queue.enqueue(current.getLeft());
          }
        }
      }
    }

    return nearest;
  }

  // unit testing of the methods (optional)
  public static void main(String[] args) {}
}

class Node {
  private Comparator<Point2D> comparator;
  private Point2D point2D;
  private Node left;
  private Node right;

  public Node(Point2D point2D, Comparator<Point2D> comparator) {
    this.point2D = point2D;
    this.comparator = comparator;
  }

  public void draw() {
    this.point2D.draw();
    if (this.left != null) this.left.draw();
    if (this.right != null) this.right.draw();
  }

  public Node search(Point2D point2D) {
    if (this.point2D.compareTo(point2D) == 0) return this;

    Node n = this;
    Node prev = n;
    while (n != null) {
      prev = n;
      if (n.comparator.compare(point2D, n.point2D) < 0) {
        n = n.left;
      } else {
        n = n.right;
      }
    }
    return prev;
  }

  public boolean insert(Point2D point2D) {
    Node found = this.search(point2D);
    if (found.point2D.compareTo(point2D) == 0) return false; // duplicated

    if (found.comparator.compare(point2D, found.point2D) < 0) {
      found.left = new Node(point2D, reverse(found.comparator));
    } else {
      found.right = new Node(point2D, reverse(found.comparator));
    }
    return true;
  }

  public boolean pointEquals(Point2D point2D) {
    return this.point2D.compareTo(point2D) == 0;
  }

  public boolean onRect(RectHV rectHV) {
    return rectHV.contains(this.point2D);
  }

  public boolean possibleLeftOnRect(RectHV rectHV) {
    if (this.left == null) return false;

    if (this.comparator == Point2D.X_ORDER) {
      if (rectHV.xmin() < this.point2D.x()) return true;
    } else {
      if (rectHV.ymin() < this.point2D.y()) return true;
    }
    return false;
  }

  public boolean possibleRightOnRect(RectHV rectHV) {
    if (this.right == null) return false;

    if (this.comparator == Point2D.X_ORDER) {
      if (rectHV.xmax() > this.point2D.x()) return true;
    } else {
      if (rectHV.ymax() > this.point2D.y()) return true;
    }
    return false;
  }

  public Node getLeft() {
    return left;
  }

  public Node getRight() {
    return right;
  }

  public Point2D getPoint2D() {
    return point2D;
  }

  public Comparator<Point2D> getComparator() {
    return comparator;
  }

  private static Comparator<Point2D> reverse(Comparator<Point2D> in) {
    if (in == Point2D.X_ORDER) return Point2D.Y_ORDER;
    if (in == Point2D.Y_ORDER) return Point2D.X_ORDER;
    throw new RuntimeException("Unreachable");
  }
}

class NodeIterator implements Iterator<Point2D> {
  private RectHV rectHV;
  private Queue<Node> queue = new Queue<>();

  public NodeIterator(Node node, RectHV rectHV) {
    this.queue.enqueue(node);
    this.rectHV = rectHV;
  }

  public boolean hasNext() {
    while (!queue.isEmpty()) {
      if (queue.peek().onRect(rectHV)) {
        return true;
      }
      Node current = queue.dequeue();
      if (current.possibleLeftOnRect(rectHV)) queue.enqueue(current.getLeft());
      if (current.possibleRightOnRect(rectHV)) queue.enqueue(current.getRight());
    }
    return false;
  }

  public Point2D next() {
    while (!queue.isEmpty()) {
      Node current = queue.dequeue();
      if (current.possibleLeftOnRect(rectHV)) queue.enqueue(current.getLeft());
      if (current.possibleRightOnRect(rectHV)) queue.enqueue(current.getRight());

      if (current.onRect(rectHV)) {
        return current.getPoint2D();
      }
    }
    return null;
  }
}
