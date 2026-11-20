"""A rounded square section swept along half an ellipse. A pipe whose section is a closed periodic B-spline, swept along half an ellipse.

The path is the half of the ellipse x²/20² + y²/12² = 1 in the xy plane that has y ≥ 0, from
(20, 0, 0) through (0, 12, 0) to (−20, 0, 0); at its start it leaves along +y.

The section, at every point of the path, is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of side 6 centered on the path point,
in the plane square to the path, with two sides parallel to the z axis: a square rounded all the
way to the middle of each side, its curve four parabolic arcs, each from the middle of one side
to the middle of the next, tangent to the sides there. Its corners are (3, 3), (−3, 3), (−3, −3)
and (3, −3) in coordinates (a, z), a measured along the path's unit normal in the xy plane,
outward at the start. At the start the section's control polygon has corners (23, 0, 3),
(17, 0, 3), (17, 0, −3) and (23, 0, −3). The ellipse is tightest at its ends, radius of curvature
144/20 = 7.2, more than the section's half-width of 3, so the pipe never folds. The two ends of
the pipe are flat, each the section there."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.Geom import Geom_Ellipse
from OCP.gp import gp

from pipes import planar_pipe


def build():
    ellipse = Geom_Ellipse(gp.XOY_s(), 20, 12)
    return planar_pipe(BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi).Edge())
