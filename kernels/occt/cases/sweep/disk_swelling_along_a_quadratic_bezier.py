"""A pipe whose circular section swells along a quadratic Bezier curve. A pipe of circular section whose radius grows steadily along the path, swept along a short
quadratic Bezier curve.

The path is the quadratic Bezier curve C(t), 0 ≤ t ≤ 1, of control points (0, 0, 0), (8, 10, 0)
and (20, 4, 0), in the xy plane: it leaves the origin along (4, 5, 0) and bends to the right, to
arrive at (20, 4, 0) heading along (12, −6, 0).

The section at C(t) is the disk centered on C(t), in the plane square to the path there, of
radius 1.5 + 2t: radius 1.5 at the start, 3.5 at the end, growing in proportion to the curve's
parameter t (not to its length). The path's radius of curvature is at least 25, well above the
pipe's radius, so the pipe never folds. The two ends of the pipe are flat disks, one of radius
1.5 square to the path at the origin and one of radius 3.5 square to the path at (20, 4, 0)."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeVertex,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused
from pipes import array

CONTROL = [(0, 0, 0), (8, 10, 0), (20, 4, 0)]


def circle(center, tangent, radius):
    ring = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(gp_Pnt(*center), gp_Dir(*tangent)), radius)).Edge()
    return BRepBuilderAPI_MakeWire(ring).Wire()


def build():
    poles = array(Array1_gp_Pnt, [gp_Pnt(*p) for p in CONTROL])
    spine = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()
    path = BRepBuilderAPI_MakeWire(spine).Wire()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)
    pipe.Add(circle((0, 0, 0), (4, 5, 0), 1.5))
    pipe.Add(circle((20, 4, 0), (12, -6, 0), 3.5), BRepBuilderAPI_MakeVertex(gp_Pnt(20, 4, 0)).Vertex())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the two circles")
    if not pipe.MakeSolid():
        raise Refused("the swept circles did not close into a solid")
    return pipe.Shape()
