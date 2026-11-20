"""A cylinder of radius 5 and height 10 on the z axis, less a coaxial cone frustum from z = -3
(radius 2) to z = 10 (radius 4), its upper end disc in the cylinder's top face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

Z = gp_Dir(0, 0, 1)


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), Z), 5, 10).Shape()
    second = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -3), Z), 2, 4, 13).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
