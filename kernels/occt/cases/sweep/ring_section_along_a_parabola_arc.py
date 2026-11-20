"""A rounded square section swept along a parabola arc. A pipe whose section is a closed periodic B-spline, swept along an arc of a parabola.

The path is the arc of the parabola y = x²/40 in the xy plane from (−20, 10, 0) through its
vertex (0, 0, 0) to (20, 10, 0).

The section, at every point of the path, is the periodic (unclamped, closed) uniform quadratic
B-spline, non-rational, whose control polygon is the square of side 6 centered on the path point,
in the plane square to the path, with two sides parallel to the z axis: a square rounded all the
way to the middle of each side, its curve four parabolic arcs, each from the middle of one side
to the middle of the next, tangent to the sides there. Its corners are (3, 3), (−3, 3), (−3, −3)
and (3, −3) in coordinates (a, z), a measured along the path's unit normal in the xy plane. The
parabola is tightest at its vertex, radius of curvature 20, more than the section's half-width
of 3, so the pipe never folds. The two ends of the pipe are flat, each the section there."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.Geom import Geom_Parabola
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from pipes import planar_pipe


def build():
    # gp_Parab: Y² = 4 F X in the frame, X along the axis of symmetry (global +y), so the frame's
    # Y is global −x and the parameter v gives the point (−v, v²/40, 0): v from −20 to 20 runs the arc
    # from (20, 10, 0) to (−20, 10, 0), the section being symmetric, the same solid.
    parabola = Geom_Parabola(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(0, 1, 0)), 10)
    return planar_pipe(BRepBuilderAPI_MakeEdge(parabola, -20, 20).Edge())
