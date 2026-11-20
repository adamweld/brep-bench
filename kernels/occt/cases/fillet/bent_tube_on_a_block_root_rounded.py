"""A tube bent along a Bezier curve standing on a block, its root rounded. A block from
(-30, -30, 0) to (30, 30, 10), and the disk of radius 3 swept square to the cubic Bezier curve
(-4, 0, 4), (-1, 0, 14), (8, 0, 18), (20, 0, 32), fused into one solid. The curve where the tube
meets the block's top is rounded to radius 1."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipe
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.Geom import Geom_BezierCurve
from OCP.collections import Array1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where, rounded

CONTROL = [(-4, 0, 4), (-1, 0, 14), (8, 0, 18), (20, 0, 32)]


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-30, -30, 0), gp_Pnt(30, 30, 10)).Shape()
    points = Array1_gp_Pnt(1, 4)
    for i, p in enumerate(CONTROL):
        points.SetValue(i + 1, gp_Pnt(*p))
    spine = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(points)).Edge()).Wire()
    # The curve's tangent at its start, (3, 0, 10) up to scale: the disk is square to it.
    start = gp_Pnt(*CONTROL[0])
    ring = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(3, 0, 10)), 3)).Edge()
    disk = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakeWire(ring).Wire()).Face()
    pipe = BRepOffsetAPI_MakePipe(spine, disk)
    pipe.Build()
    tube = pipe.Shape()
    fuse = BRepAlgoAPI_Fuse(block, tube)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the tube to the block")
    part = fuse.Shape()
    root = edges_where(part, lambda p: abs(p.Z() - 10) < 1e-6 and abs(p.X()) < 10 and abs(p.Y()) < 10)
    if len(root) != 1:
        raise Refused(f"{len(root)} junction edges")
    return rounded(part, 1, root, "the tube's root")
