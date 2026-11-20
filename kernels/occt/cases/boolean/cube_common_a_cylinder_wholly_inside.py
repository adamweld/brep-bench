"""A cube in common with a cylinder wholly inside it. A cube of side 10 from the origin and a cylinder
of radius 3 on the axis x = 5, y = 5 from z = 3 to z = 7. Make the part common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(5, 5, 3), gp.DZ_s()), 3, 4).Shape()
    common = BRepAlgoAPI_Common(cube, cylinder)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the cylinder inside it")
    return common.Shape()
