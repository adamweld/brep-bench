"""An ellipse swept along a twisted cubic Bézier curve in its Frenet frame. An ellipse swept along a
cubic Bézier curve that does not lie in a plane, carried in the curve's Frenet frame.

The curve's four control points are (0, 0, 0), (12, 16, 0), (26, 8, 0) and (24, 0, 16): it leaves
the origin along (3, 4, 0), swings round to the right and rises out of the xy plane to end at
(24, 0, 16). Its torsion never vanishes and its radius of curvature is least, 10.28, a little past
the middle.

The section is an ellipse centred on the curve and square to it, with semi-axis 6 along the
curve's principal normal and semi-axis 3 along its binormal (the Frenet frame: the principal
normal is the direction of the curve's second derivative with its tangential part removed, the
binormal the tangent crossed with it), at every point. The ends are flat, square to the curve at
its two ends. The wall does not meet itself."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt, gp_Vec

from bench import Refused

CONTROL = [(0, 0, 0), (12, 16, 0), (26, 8, 0), (24, 0, 16)]


def build():
    poles = Array1_gp_Pnt(1, len(CONTROL))
    for i, p in enumerate(CONTROL):
        poles.SetValue(i + 1, gp_Pnt(*p))
    curve = Geom_BezierCurve(poles)
    path = BRepBuilderAPI_MakeEdge(curve).Edge()

    # the Frenet frame at the start: the tangent, and the principal normal, the second derivative
    # less its part along the tangent
    start, tangent, accel = gp_Pnt(), gp_Vec(), gp_Vec()
    curve.D2(curve.FirstParameter(), start, tangent, accel)
    unit = gp_Dir(tangent)
    normal = accel - gp_Vec(unit) * accel.Dot(gp_Vec(unit))
    # semi-axis 6 along the principal normal, 3 along the binormal
    section = BRepBuilderAPI_MakeEdge(gp_Elips(gp_Ax2(start, unit, gp_Dir(normal)), 6, 3)).Edge()

    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the ellipse along the curve")
    if not pipe.MakeSolid():
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
