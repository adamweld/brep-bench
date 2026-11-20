"""A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), less a frustum from z = 1.5 to z = 4.5 wholly inside it, base radius 3/2 and top radius 3/4."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-3, -3, 0), gp_Pnt(3, 3, 6)).Shape()
    void = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 1.5), gp_Dir(0, 0, 1)), 1.5, 0.75, 3).Shape()
    return _cut(cube, void)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()
