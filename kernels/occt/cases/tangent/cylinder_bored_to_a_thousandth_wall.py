"""A cylinder of radius 5 and height 10 about the z axis from z = 0, less a cylinder of radius 4.999
on the same axis between the same two planes. The first less the second."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

OUTER = 5.0
INNER = 4.999
HEIGHT = 10.0


def build():
    outer = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), OUTER, HEIGHT).Shape()
    inner = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), INNER, HEIGHT).Shape()
    result = BRepAlgoAPI_Cut(outer, inner)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
