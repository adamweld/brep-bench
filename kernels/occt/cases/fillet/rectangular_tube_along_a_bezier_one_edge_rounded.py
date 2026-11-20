"""A rectangular tube swept along a cubic Bézier, one long edge rounded. The Bézier with control
points (0, 0, 0), (10, 0, 0), (20, 0, 10), (20, 0, 20), a 2 by 4 rectangle carried along it, square
to it, its short sides in the plane y = 0; the long edge u = 1, v = 2 (from (0, 2, 1)) rounded to
2/5."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipe
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused
from blend import rounded
from edit import straight_edges


def path():
    poles = Array1_gp_Pnt(1, 4)
    for i, p in enumerate([(0, 0, 0), (10, 0, 0), (20, 0, 10), (20, 0, 20)]):
        poles.SetValue(i + 1, gp_Pnt(*p))
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()).Wire()


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for y, z in [(-2, -1), (2, -1), (2, 1), (-2, 1)]:
        outline.Add(gp_Pnt(0, y, z))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    pipe = BRepOffsetAPI_MakePipe(path(), face)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipe could not sweep the rectangle")
    tube = pipe.Shape()
    edge = straight_edges(tube, lambda p: (abs(p.X()) < 1e-7 and abs(p.Y() - 2) < 1e-7
                                           and abs(p.Z() - 1) < 1e-7) or p.X() > 15 and p.Y() > 1.9)
    long_edge = [e for e in edge if not _in_plane(e)]
    assert len(long_edge) == 1, len(long_edge)
    return rounded(tube, 0.4, long_edge, "the tube's long edge")


def _in_plane(edge):
    """Whether the edge is one of an end face's four sides (both ends on x = 0 or z = 20)."""
    from OCP.BRep import BRep_Tool
    from OCP.TopExp import TopExp
    a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
    b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
    return (abs(a.X()) < 1e-7 and abs(b.X()) < 1e-7) or (abs(a.Z() - 20) < 1e-7 and abs(b.Z() - 20) < 1e-7)
