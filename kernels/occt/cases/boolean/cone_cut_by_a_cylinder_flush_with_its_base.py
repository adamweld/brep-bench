"""A cone (base radius 6 at z = 0, apex at z = 12) less a cylinder of radius 6 from z = 0 to z = 4:
the cylinder's base is the cone's base."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

Z = gp_Dir(0, 0, 1)


def build():
    first = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), Z), 6, 0, 12).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), Z), 6, 4).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
