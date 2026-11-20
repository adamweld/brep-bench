"""A block 20 by 12 by 4 with its top edge along x = 20 rounded by a fillet of radius 4, and a
cylinder of radius 3 from z = -6 to z = 0 about the vertical line through (17, 3)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp, TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.BRep import BRep_Tool

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 12, 4)).Shape()
    # the top edge along x = 20 runs in y at (20, ., 4)
    fillet = BRepFilletAPI_MakeFillet(block)
    explorer = TopExp_Explorer(block, TopAbs_EDGE)
    found = False
    while explorer.More():
        edge = TopoDS.Edge(explorer.Current())
        a = BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge))
        b = BRep_Tool.Pnt_s(TopExp.LastVertex_s(edge))
        if all(abs(p.X() - 20) < 1e-9 and abs(p.Z() - 4) < 1e-9 for p in (a, b)):
            fillet.Add(4, edge)
            found = True
            break
        explorer.Next()
    if not found:
        raise Refused("the edge to round was not found")
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet declined a radius equal to the block's thickness")
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(17, 3, -6), gp.DZ_s()), 3, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(fillet.Shape(), post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the rounded block and the post")
    return fuse.Shape()
