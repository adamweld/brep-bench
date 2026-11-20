"""A rounded square section swept along a three-quarter circle. A pipe whose section is a closed periodic B-spline, swept along a circular arc.

The path is the arc of the circle of radius 12 about the z axis in the xy plane, from (12, 0, 0)
through (0, 12, 0) and (−12, 0, 0) to (0, −12, 0): three quarters of a turn, leaving the start
along +y.

The section, at every point of the path, is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of side 6 centered on the path point,
in the plane square to the path, with two sides parallel to the z axis: a square rounded all the
way to the middle of each side, so its curve is four parabolic arcs, each from the middle of one
side to the middle of the next, tangent to the sides there. Its corners are (3, 3), (−3, 3),
(−3, −3) and (3, −3) in coordinates (r, z), r measured outward from the z axis. At the start,
the section's control polygon has corners (15, 0, 3), (9, 0, 3), (9, 0, −3) and (15, 0, −3). The
two ends of the pipe are flat, each the section there."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.gp import gp, gp_Circ

from pipes import planar_pipe


def build():
    return planar_pipe(BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), 12), 0, 1.5 * math.pi).Edge())
