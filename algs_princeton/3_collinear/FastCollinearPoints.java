import java.util.ArrayList;
import java.util.Arrays;

public class FastCollinearPoints {
    final private ArrayList<LineSegment> segments;

    public FastCollinearPoints(final Point[] pointsRaw)    // finds all line segments containing 4 points
    {
        if (pointsRaw == null)
            throw new IllegalArgumentException();
        for (Point p : pointsRaw) {
            if (p == null) throw new IllegalArgumentException();
        }
        Point[] points = pointsRaw.clone();

        Arrays.sort(points, Point::compareTo);

        ArrayList<LineSegment> segments = new ArrayList<>();

        for (int i = 0; i < points.length; i++) {
            Arrays.sort(points, Point::compareTo);
            Point anchor = points[i];
            Arrays.sort(points, anchor.slopeOrder());

            int left = 1;
            int right = 1;
            double slope = Double.NEGATIVE_INFINITY;

            for (int j = 1; j < points.length; j++) {
                if (anchor.compareTo(points[j]) == 0)
                    throw new IllegalArgumentException("duplicated points found");

                double currentSlop = anchor.slopeTo(points[j]);
                if (currentSlop > slope) {
                    if (right - left + 1 >= 3 && anchor.compareTo(points[left]) < 0) { // alignment found
                        segments.add(new LineSegment(anchor, points[right]));
                    }
                    slope = currentSlop;
                    left = j;
                    right = j;
                } else {
                    // slope equal
                    right++;
                }
            }

            // For the case where last few elements are aligned
            if (right - left + 1 >= 3 && anchor.compareTo(points[left]) < 0) { // alignment found
                segments.add(new LineSegment(anchor, points[right]));
            }
        }

        this.segments = segments;
    }

    public int numberOfSegments()        // the number of line segments
    {
        return segments.size();
    }

    public LineSegment[] segments()                // the line segments
    {
        return segments.toArray(new LineSegment[0]);
    }

    /**
     * require stdlib.jar
     */
    public static void main(String[] args) {
        // read the n points from a file
        In in = new In(args[0]);
        int n = in.readInt();
        Point[] points = new Point[n];
        for (int i = 0; i < n; i++) {
            int x = in.readInt();
            int y = in.readInt();
            points[i] = new Point(x, y);
        }

        // draw the points
        StdDraw.enableDoubleBuffering();
        StdDraw.setXscale(0, 100);
        StdDraw.setYscale(0, 100);
        for (Point p : points) {
            p.draw();
        }
        StdDraw.show();

        // print and draw the line segments
        FastCollinearPoints collinear = new FastCollinearPoints(points);
        System.out.println(collinear.numberOfSegments());
        for (LineSegment segment : collinear.segments()) {
            StdOut.println(segment);
            segment.draw();
        }
        StdDraw.show();
    }
}
