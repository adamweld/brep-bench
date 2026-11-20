"""A cone frustum on the z axis (radius 6 at z = 0, 3 at z = 10) less a coaxial narrower frustum from z = -2
(radius 1) to z = 10 (radius 2), its upper end disc in the wider one's top face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

Z = gp_Dir(0, 0, 1)


def build():
    first = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), Z), 6, 3, 10).Shape()
    second = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -2), Z), 1, 2, 12).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
