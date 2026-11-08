"""A leaning bull-nose mill carried along a spline. A block, the box from (−6, −6, −8) to (36, 26,
0), with a groove cut into its top by a leaning bull-nose mill carried along a spline.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (10, 0,
0), (20, 20, 0) and (30, 20, 0), run from its first end to its last.

A frame rides along the curve: at each point its origin is the curve's point, its x axis the
curve's tangent there (pointing the way the curve runs), its z axis straight up and its y axis to
the left, so that at the curve's start it is the world's own frame. The tool is drawn in that
starting frame and carried rigidly by the frame as it runs the whole curve.

The mill, so drawn, is a solid of revolution about an axis leaning 30° from upright toward +x (the
way the curve sets out), along a = (sin 30°, 0, cos 30°), through c = (0, 0, −1), the middle of its
flat end, 1 below the block's top. Along its axis it runs from c up to c + 3a: a cylinder of radius
2 whose end corner is rounded to radius 0.5 (its flat end a disc of radius 1.5 about c, its side of
radius 2 from c + 0.5a to c + 3a, and between them a quarter of a torus of tube radius 0.5 centered
on the circle of radius 1.5 about c + 0.5a).

Cut from the block everything the mill passes through as the frame runs the whole curve.

The frame turns the mill as the curve turns, so no screw carries it: the curve along which the mill
touches what it sweeps moves on the mill from instant to instant."""
from bench import Refused


def build():
    # The frame turns the leaning mill as the curve turns: at each instant the mill moves by a
    # slide along the curve's tangent and a turn about the upright line through the curve's
    # point, at the curve's curvature, which changes along the curve. The mill's axis leans
    # off that line, so the curve along which each of its faces touches what it sweeps depends
    # on the curvature: it moves over the mill from instant to instant. OCCT sweeps a face along
    # a path (BRepOffsetAPI_MakePipeShell) only whole, and a face swept whole that the motion
    # runs along inside folds; there is no fixed curve to split it along, and the wall is the
    # envelope of the mill, which occt has no operation to find.
    raise Refused("occt has no sweep of a solid, and the curve along which the leaning mill "
                  "touches what it sweeps moves over it as the curve's curvature changes, so no "
                  "fixed split of its faces sweeps without folding; the groove's wall is the "
                  "envelope of the mill, which occt cannot find")
