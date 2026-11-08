"""A wheel carried along a spline, cutting with its rim. A block, the box from (−6, −6, −8) to (36,
26, 0), with a groove cut into its top by a ring carried along a spline, standing up like a wheel
across it.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (10, 0,
0), (20, 20, 0) and (30, 20, 0), run from its first end to its last.

A frame rides along the curve: at each point its origin is the curve's point, its x axis the
curve's tangent there (pointing the way the curve runs), its z axis straight up and its y axis to
the left, so that at the curve's start it is the world's own frame. The tool is drawn in that
starting frame and carried rigidly by the frame as it runs the whole curve.

The ring, so drawn, is a torus centered at (0, 0, 0.5) whose axis is the y axis direction: the
circle through its tube's centers has radius 1.5, and its tube has radius 0.5. So it stands upright
across the curve like a wheel, its lowest point 1.5 below the block's top.

Cut from the block everything the ring passes through as the frame runs the whole curve.

The ring is no convex solid: as the curve bends, its inner side comes into contact too."""
from bench import Refused


def build():
    # The frame turns the ring as the curve turns: at each instant it moves by a slide along the
    # curve's tangent and a turn about the upright line through the curve's point, at the
    # curve's curvature. The ring's normal is square to that velocity on curves that depend on
    # the curvature: they move over the ring from instant to instant, and its inner side comes
    # into contact where the curve bends. OCCT sweeps a face along a path
    # (BRepOffsetAPI_MakePipeShell) only whole, and no fixed split of the torus sweeps without
    # folding; the groove's wall is the envelope of the ring, which occt has no operation to
    # find.
    raise Refused("occt has no sweep of a solid, and the curve along which the ring touches "
                  "what it sweeps moves over it as the curve's curvature changes, so no fixed "
                  "split of its torus sweeps without folding; the groove's wall is the envelope "
                  "of the ring, which occt cannot find")
