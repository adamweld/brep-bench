"""A rounded square section carried round a sharp corner with a round joint. A pipe whose section is a closed periodic B-spline, swept along a path with one sharp corner, the
section carried round the corner by turning it about the corner.

The path: two straight runs, from (−30, 0, 0) along +x to the origin, then turning a right angle
to the left and on along +y to (0, 30, 0).

The section, square to the path and centered on it, is the periodic (unclamped, closed) uniform
quadratic B-spline, non-rational, whose control polygon is the square of side 6 with two sides
parallel to the z axis: a square rounded all the way to the middle of each side, its curve four
parabolic arcs, each from the middle of one side to the middle of the next, tangent to the sides
there. At the start its control polygon has corners (−30, 3, 3), (−30, −3, 3), (−30, −3, −3) and
(−30, 3, −3). The section keeps its two sides parallel to z along both runs, as a guide curve
running above the path at a constant height would hold it.

Each run is the section swept straight along it, ending flat at the corner, in the plane through
the origin square to the run. Inside the corner the two runs overlap. The wedge they leave open
outside the corner (x ≥ 0, y ≤ 0) is filled by the section turned about the vertical line through
the corner, the z axis, a quarter turn from square to the first run to square to the second. The
two ends of the pipe, at (−30, 0, 0) and (0, 30, 0), are flat, each the section there."""
from OCP.gp import gp_Dir

from corner import ROUND, polyline
from pipes import periodic_square, pipe, start_frame


def build():
    path = polyline([(-30, 0, 0), (0, 0, 0), (0, 30, 0)])
    start, tangent = start_frame(path)
    up = gp_Dir(0, 0, 1)
    across = up.Crossed(tangent)
    return pipe(path, periodic_square(start, across, up), binormal=up, corners=ROUND)
