"""A rectangular tube swept along a cubic Bézier with its start rim rounded with a radius that varies
round it. The Bézier with control points (0, 0, 0), (10, 0, 0), (20, 0, 10), (20, 0, 20), a 2 by 4
rectangle carried along it, square to it, its short sides in the plane y = 0. The four edges of the
start rectangle (in the plane x = 0, y in [−2, 2], z in [−1, 1]) rounded with r = 2/5 + z/20 + y/40,
each round's section square to its edge the quarter circle tangent to the end face and the side wall;
the rounds creased at the four corners, where the tube's long edges stay sharp."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipe
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused
from blend import edges_where
from vround import rounded_with_law

POLES = [(0, 0, 0), (10, 0, 0), (20, 0, 10), (20, 0, 20)]


def build():
    poles = Array1_gp_Pnt(1, 4)
    for i, p in enumerate(POLES):
        poles.SetValue(i + 1, gp_Pnt(*p))
    path = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()).Wire()
    outline = BRepBuilderAPI_MakePolygon()
    for y, z in [(-2, -1), (2, -1), (2, 1), (-2, 1)]:
        outline.Add(gp_Pnt(0, y, z))
    outline.Close()
    pipe = BRepOffsetAPI_MakePipe(path, BRepBuilderAPI_MakeFace(outline.Wire()).Face())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipe could not sweep the rectangle")
    tube = pipe.Shape()
    # The four sides of the start rectangle: edges lying in the plane x = 0 (middle point there).
    rim = edges_where(tube, lambda p: abs(p.X()) < 1e-7 and (abs(abs(p.Y()) - 2) < 1e-7 or abs(abs(p.Z()) - 1) < 1e-7)
                      and (abs(p.Y()) < 1e-7 or abs(p.Z()) < 1e-7))
    assert len(rim) == 4, len(rim)
    return rounded_with_law(tube, rim, lambda p: 0.4 + p.Z() / 20 + p.Y() / 40, "the start rim", stations=16)
