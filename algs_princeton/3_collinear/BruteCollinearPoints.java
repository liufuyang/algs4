import java.util.ArrayList;
import java.util.Arrays;

public class BruteCollinearPoints {
    final private ArrayList<LineSegment> segments;

    public BruteCollinearPoints(final Point[] pointsRaw)    // finds all line segments containing 4 points
    {
        if (pointsRaw == null)
            throw new IllegalArgumentException();
        for (Point p : pointsRaw) {
            if (p == null) throw new IllegalArgumentException();
        }
        Point[] points = pointsRaw.clone();

        ArrayList<LineSegment> segments = new ArrayList<>();
        for (int i = 0; i < points.length; i++)
            for (int j = i + 1; j < points.length; j++)
                for (int k = j + 1; k < points.length; k++)
                    for (int l = k + 1; l < points.length; l++) {
                        if(points[i].compareTo(points[j]) == 0)
                            throw new IllegalArgumentException("duplicated points found");

                        double s1 = points[i].slopeTo(points[j]);
                        double s2 = points[i].slopeTo(points[k]);
                        double s3 = points[i].slopeTo(points[l]);
                        if (s1 == s2 && s1 == s3) {
                            Point[] tmp = {points[i], points[j], points[k], points[l]};
                            Arrays.sort(tmp, Point::compareTo);
                            segments.add(new LineSegment(tmp[0], tmp[3]));
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
        BruteCollinearPoints collinear = new BruteCollinearPoints(points);
        System.out.println(collinear.numberOfSegments());
        for (LineSegment segment : collinear.segments()) {
            StdOut.println(segment);
            segment.draw();
        }
        StdDraw.show();
    }
}