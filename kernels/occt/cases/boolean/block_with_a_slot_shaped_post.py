"""A block from (0, 0, 0) to (30, 20, 5) with a slot-shaped post 8 high on its top: the stadium
within 3 of the segment from (10, 10) to (20, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

X0, X1, Y, R = 10, 20, 10, 3  # the slot's centre line runs from (X0, Y) to (X1, Y)
Z0, HEIGHT = 5, 8


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(30, 20, Z0)).Shape()
    p = lambda x, y: gp_Pnt(x, y, Z0)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(p(X0, Y - R), p(X1, Y - R)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(p(X1, Y - R), p(X1 + R, Y), p(X1, Y + R)).Value()).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(p(X1, Y + R), p(X0, Y + R)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(GC_MakeArcOfCircle(p(X0, Y + R), p(X0 - R, Y), p(X0, Y - R)).Value()).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    post = BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, HEIGHT)).Shape()
    fuse = BRepAlgoAPI_Fuse(block, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block and the post")
    fuse.SimplifyResult()
    return fuse.Shape()
