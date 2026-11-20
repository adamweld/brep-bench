"""A cylinder of radius 4 and length 10 on the x axis, less a thinner one of radius 2 and length 10 on the y axis."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    thick = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-5, 0, 0), gp_Dir(1, 0, 0)), 4, 10).Shape()
    thin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, -5, 0), gp_Dir(0, 1, 0)), 2, 10).Shape()
    return _cut(thick, thin)


def _cut(first, second):
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the operands")
    return cut.Shape()
