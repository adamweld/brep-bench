"""A groove that crosses itself: a mill run round a looping spline. A block, the box from (−10,
−10, −8) to (30, 25, 0), with a groove cut into its top by a bull-nose mill run along a spline that
loops over itself, so that the groove crosses itself.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (30, 20,
0), (−10, 20, 0) and (20, 0, 0), run from its first end to its last. It crosses itself once, near
(9.98, 8.54), and at its sharpest it turns on a radius of about 1.79, tighter than the mill.

The mill is a solid of revolution about an upright axis that rides the curve: a cylinder of radius
2, from 1.5 below the block's top up to 5 above it, whose bottom corner is rounded to radius 0.5 (a
flat bottom of radius 1.5 joined to the side by a quarter torus of tube radius 0.5). Its axis
starts on the vertical line through the curve's first point and stays upright, its foot on the
curve, as it runs the whole curve. Cut from the block everything the mill passes through.

Where the curve crosses itself the groove comes back onto itself: the sheet the mill sweeps passes
through its own earlier part. Where it turns tighter than the mill, the groove's inner wall folds
and is cut away by the mill itself."""
from bench import Refused


def build():
    # The mill is a solid of revolution about the upright axis riding the curve, so it touches
    # what it sweeps along its meridians square to the curve, and each piece of it split there
    # could be carried along the curve by BRepOffsetAPI_MakePipeShell. But the curve turns on a
    # radius of about 1.79, tighter than the mill's 2: the sheet the mill's inner side sweeps
    # folds back through itself there, and OCCT's Booleans take no solid whose boundary passes
    # through itself; it has no operation that trims a sweep where it folds.
    raise Refused("occt has no sweep of a solid, and where the curve turns tighter than the "
                  "mill (radius 1.79 against 2) the sheet the mill's inner side sweeps along it "
                  "folds through itself; occt's Booleans take no self-intersecting solid and it "
                  "has no operation to trim a sweep where it folds")
